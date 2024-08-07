use super::types::Config;
use crate::collectors::time_collector::NewTick;
use crate::collectors::block_collector::NewBlock;
use anyhow::{anyhow, Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_aave::{
    i_aave_oracle::IAaveOracle,
    i_pool_data_provider::IPoolDataProvider,
    ierc20::IERC20,
    l2_encoder::L2Encoder,
    pool::{BorrowFilter, Pool},
};
use bindings_liquidator::liquidator::{Liquidator, ParaSwapData};
use clap::{Parser, ValueEnum};
use ethers::core::types::Bytes;
use ethers::{
    contract::builders::ContractCall,
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, ValueOrArray, H160, I256, U256, U64},
};
use ethers_contract::Multicall;
use reqwest::header;
use reqwest::{Client};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};
use serde_json::Value;


use super::types::{Action, Event};

#[derive(Debug)]
struct DeploymentConfig {
    pool_address: Address,
    pool_data_provider: Address,
    oracle_address: Address,
    l2_encoder: Address,
    creation_block: u64,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Deployment {
    AAVE,
    SEASHELL,
}

pub const WETH_ADDRESS: &str = "0x4200000000000000000000000000000000000006";
pub const WETH_UNIT: &str = "1000000000000000000";

pub const LIQUIDATION_CLOSE_FACTOR_THRESHOLD: &str = "950000000000000000";
pub const MAX_LIQUIDATION_CLOSE_FACTOR: u64 = 10000;
pub const DEFAULT_LIQUIDATION_CLOSE_FACTOR: u64 = 5000;

// admin stuff
pub const LOG_BLOCK_RANGE: u64 = 1024;
pub const MULTICALL_CHUNK_SIZE: usize = 100;
pub const STATE_CACHE_FILE: &str = "borrowers.json";
pub const PRICE_ONE: u64 = 100000000;
pub const MAX_SLIPPAGE: u64 = 3; // 3% slippage
pub const VAULT_ADDRESS: &str = "0xBA12222222228d8Ba445958a75a0704d566BF2C8";
pub const MIN_DEBT_VALUE: u64 = 50_00000000; //100 USD

fn get_deployment_config(deployment: Deployment) -> DeploymentConfig {
    match deployment {
        Deployment::AAVE => DeploymentConfig {
            pool_address: Address::from_str("0x794a61358D6845594F94dc1DB02A252b5b4814aD").unwrap(), //l2pool proxy 
            pool_data_provider: Address::from_str("0x69FA688f1Dc47d4B5d8029D5a35FB7a548310654")
                .unwrap(),
            oracle_address: Address::from_str("0xD81eb3728a631871a7eBBaD631b5f424909f0c77")
                .unwrap(),
            l2_encoder: Address::from_str("0x9abADECD08572e0eA5aF4d47A9C7984a5AA503dC").unwrap(),
            creation_block: 120641673,
        },
        Deployment::SEASHELL => DeploymentConfig {
            pool_address: Address::from_str("0x8F44Fd754285aa6A2b8B9B97739B79746e0475a7").unwrap(),
            pool_data_provider: Address::from_str("0x2A0979257105834789bC6b9E1B00446DFbA8dFBa")
                .unwrap(),
            oracle_address: Address::from_str("0xFDd4e83890BCcd1fbF9b10d71a5cc0a738753b01")
                .unwrap(),
            l2_encoder: Address::from_str("0xceceF475167f7BFD8995c0cbB577644b623cD7Cf").unwrap(),
            creation_block: 3318602,
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
}

struct PoolState {
    prices: HashMap<Address, U256>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    // collateral: HashSet<Address>,
    // debt: HashSet<Address>,
    debt_value: U256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenConfig {
    address: Address,
    a_address: Address,
    decimals: u64,
    ltv: u64,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    reserve_factor: u64,
    protocol_fee: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildTxResponse {
    from: String,
    to: String,
    value: String,
    data: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildTxErrorResponse {
    error: String
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AaveStrategy<M> {
    /// Ethers client.
    client: Arc<M>,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    tokens: HashMap<Address, TokenConfig>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    pub fn new(
        client: Arc<M>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
    ) -> Self {
        Self {
            client,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            borrowers: HashMap::new(),
            tokens: HashMap::new(),
            chain_id: config.chain_id,
            config: get_deployment_config(deployment),
            liquidator: Address::from_str(&liquidator_address).expect("invalid liquidator address"),
        }
    }
}

struct LiquidationOpportunity {
    borrower: Address,
    collateral: Address,
    debt: Address,
    debt_to_cover: U256,
    profit_eth: I256,
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for AaveStrategy<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        self.update_token_configs().await?;
        // self.approve_tokens().await?;
        self.load_cache()?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::NewBlock(block) => self.process_new_block_event(block).await,
            Event::NewTick(block) => self.process_new_tick_event(block).await,
        }
    }
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    /// Process new block events, updating the internal state.
    async fn process_new_block_event(&mut self, event: NewBlock) -> Option<Action> {
        info!("received new block: {:?}", event);
        // self.last_block_number = event.number.as_u64();
        None
    }

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Action> {
        info!("received new tick: {:?}", event);
        self.update_state()
            .await
            .map_err(|e| error!("Update State error: {}", e))
            .ok()?;

        info!("Total borrower count: {}", self.borrowers.len());
        let op = self
            .get_best_liquidation_op()
            .await
            .map_err(|e| error!("Error finding liq ops: {}", e))
            .ok()??;

        info!("Best op - profit: {}", op.profit_eth);

        if op.profit_eth <= I256::from(0) {
            info!("No profitable ops, passing");
            return None;
        }

        return Some(Action::SubmitTx(SubmitTxToMempool {
            tx: self
                .build_liquidation(&op)
                .await
                .map_err(|e| error!("Error building liquidation: {}", e))
                .ok()?,
            gas_bid_info: Some(GasBidInfo {
                bid_percentage: self.bid_percentage,
                total_profit: U256::from_dec_str(&op.profit_eth.to_string())
                    .map_err(|e| error!("Failed to bid: {}", e))
                    .ok()?,
            }),
        }));
    }

    // for all known borrowers, return a sorted set of those with health factor < 1
    async fn get_underwater_borrowers(&mut self) -> Result<Vec<(Address, U256)>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.client.clone());

        let mut underwater_borrowers = Vec::new();

        // call pool.getUserAccountData(user) for each borrower
        let mut multicall = Multicall::new(
            self.client.clone(),
            Some(H160::from_str(
                "0xcA11bde05977b3631167028862bE2a173976CA11",
            )?),
        )
        .await?;
        let mut borrowers = self
            .borrowers
            .values_mut()
            .collect::<Vec<&mut Borrower>>();

        borrowers.sort_by(|a, b| b.debt_value.cmp(&a.debt_value));

        let mut need_to_remove = Vec::new();

        for chunk in borrowers.chunks_mut(MULTICALL_CHUNK_SIZE) {
            multicall.clear_calls();

            for borrower in chunk.iter_mut() {
                multicall.add_call(pool.get_user_account_data(borrower.address), false);
            }

            let result: Vec<(U256, U256, U256, U256, U256, U256)> = multicall.call_array().await?;
            for (borrower, (total_collateral_base, total_debt_base, _, _, _, health_factor)) in zip(chunk, result) {
                if total_debt_base.lt(&U256::from(MIN_DEBT_VALUE * 2)) { //  remove borrowers lt MIN_DEBT_VALUE * 2 50% to liquidate
                    need_to_remove.push(borrower.address);
                    continue;
                }

                if total_collateral_base.eq(&U256::zero()) {
                    need_to_remove.push(borrower.address);
                    continue;
                }

                if health_factor.lt(&U256::from_dec_str("1000000000000000000").unwrap()) {
                    info!(
                        "Found underwater borrower {:?} -  healthFactor: {}",
                        borrower, health_factor
                    );
                    underwater_borrowers.push((borrower.address, health_factor));
                }

                borrower.debt_value = total_debt_base; 
            }
        }

        for borrower in need_to_remove {
            self.borrowers.remove(&borrower);
        }

        //save borrowers to cache file
        self.write_cache().await?;

        // sort borrowers by health factor
        underwater_borrowers.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(underwater_borrowers)
    }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.borrowers = cache.borrowers;
            }
            Err(_) => {
                info!("no state cache file found, creating new one");
                self.last_block_number = self.config.creation_block;
            }
        };

        Ok(())
    }

    // update known borrower state from last block to latest block
    async fn update_state(&mut self) -> Result<()> {
        let latest_block = self.client.get_block_number().await?;
        info!(
            "Updating state from block {} to {}",
            self.last_block_number, latest_block
        );

        self.get_borrow_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.on_behalf_of;
                // fetch assets if user already a borrower
                if !self.borrowers.contains_key(&user) {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            debt_value: U256::from(0),
                        },
                    );
                }
            });

        // self.get_repay_logs(self.last_block_number.into(), latest_block)
        //     .await?
        //     .into_iter()
        //     .for_each(|log| {
        //         let user = log.user;
        //         // fetch assets if user already a supplier
        //         if self.borrowers.contains_key(&user) {
        //             let borrower = self.borrowers.get_mut(&user).unwrap();
        //             borrower.collateral.insert(log.reserve);
        //             return;
        //         } else {
        //             self.borrowers.insert(
        //                 user,
        //                 Borrower {
        //                     address: user,
        //                     collateral: HashSet::from([log.reserve]),
        //                     debt: HashSet::new(),
        //                 },
        //             );
        //         }
        //     });

        
        self.last_block_number = latest_block.as_u64();

        // write state cache to file
        self.write_cache().await?;

        Ok(())
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            pool.borrow_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.pool_address))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    async fn write_cache(&self) -> Result<()> {
        // write state cache to file
        let cache = StateCache {
            last_block_number: self.last_block_number,
            borrowers: self.borrowers.clone(),
        };
        
        let mut file = File::create(STATE_CACHE_FILE)?;
        file.write_all(serde_json::to_string(&cache)?.as_bytes())?;
        Ok(())
    }

    // fetch all borrow events from the from_block to to_block
    // async fn get_supply_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SupplyFilter>> {
    //     let pool = Pool::<M>::new(self.config.pool_address, self.client.clone());

    //     let mut res = Vec::new();
    //     for start_block in
    //         (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
    //     {
    //         let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
    //         pool.supply_filter()
    //             .from_block(start_block)
    //             .to_block(end_block)
    //             .address(ValueOrArray::Value(self.config.pool_address))
    //             .query()
    //             .await?
    //             .into_iter()
    //             .for_each(|log| {
    //                 res.push(log);
    //             });
    //     }

    //     Ok(res)
    // }

    async fn approve_tokens(&mut self) -> Result<()> {
        let liquidator = Liquidator::new(self.liquidator, self.client.clone());

        let mut nonce = self
            .client
            .get_transaction_count(
                self.client
                    .default_sender()
                    .ok_or(anyhow!("No connected sender"))?,
                None,
            )
            .await?;
        for token_address in self.tokens.keys() {
            let token = IERC20::new(token_address.clone(), self.client.clone());
            let allowance = token
                .allowance(self.liquidator, self.config.pool_address)
                .call()
                .await?;
            if allowance == U256::zero() {
                // TODO remove unwrap once we figure out whats broken
                liquidator
                    .approve_pool(*token_address)
                    .nonce(nonce)
                    .send()
                    .await
                    .map_err(|e| {
                        error!("approve failed: {:?}", e);
                        e
                    })?;
                nonce = nonce + 1;
            }
        }

        Ok(())
    }

    async fn update_token_configs(&mut self) -> Result<()> {
        let pool_data =
            IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.client.clone());
        let all_tokens = pool_data.get_all_reserves_tokens().await?;
        let all_a_tokens = pool_data.get_all_a_tokens().await?;
        info!("all_tokens: {:?}", all_tokens);
        for (token, a_token) in zip(all_tokens, all_a_tokens) {
            let (decimals, ltv, threshold, bonus, reserve, _, _, _, _, _) = pool_data
                .get_reserve_configuration_data(token.token_address)
                .await?;
            let protocol_fee = pool_data
                .get_liquidation_protocol_fee(token.token_address)
                .await?;
            self.tokens.insert(
                token.token_address,
                TokenConfig {
                    address: token.token_address,
                    a_address: a_token.token_address,
                    decimals: decimals.low_u64(),
                    ltv: ltv.low_u64(),
                    liquidation_threshold: threshold.low_u64(),
                    liquidation_bonus: bonus.low_u64(),
                    reserve_factor: reserve.low_u64(),
                    protocol_fee: protocol_fee.low_u64(),
                },
            );
        }

        Ok(())
    }

    async fn get_eth_price(&self, pool_state: &PoolState) -> Result<U256> {
        let weth_address = WETH_ADDRESS.parse::<Address>().unwrap();

        let eth_price = pool_state.prices.get(&weth_address).ok_or(anyhow!(
            "No price found for asset {}",
            weth_address.to_string()
        ))?;

        Ok(eth_price.clone())
    }

    async fn get_best_liquidation_op(&mut self) -> Result<Option<LiquidationOpportunity>> {
        let underwater = self.get_underwater_borrowers().await?;

        if underwater.len() == 0 {
            return Err(anyhow!("No underwater borrowers found"));
        }

        info!("Found {} underwater borrowers", underwater.len());
        let pool_data =
            IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.client.clone());

        let mut best_bonus: I256 = I256::MIN;
        let mut best_op: Option<LiquidationOpportunity> = None;
        let pool_state = self.get_pool_state().await?;

        for (borrower, health_factor) in underwater {
            if let Some(op) = self
                .get_liquidation_opportunity(
                    self.borrowers
                        .get(&borrower)
                        .ok_or(anyhow!("Borrower not found"))?,
                    &pool_data,
                    &health_factor,
                    &pool_state,
                )
                .await
                .map_err(|e| info!("Liquidation op failed {}", e))
                .ok()
            {
                if op.profit_eth > best_bonus {
                    best_bonus = op.profit_eth;
                    best_op = Some(op);
                }
            }

        }

        Ok(best_op)
    }

    async fn get_pool_state(&self) -> Result<PoolState> {
        let mut multicall = Multicall::<M>::new(
            self.client.clone(),
            Some(H160::from_str(
                "0xcA11bde05977b3631167028862bE2a173976CA11",
            )?),
        )
        .await?;
        let mut prices = HashMap::new();
        let price_oracle = IAaveOracle::<M>::new(self.config.oracle_address, self.client.clone());

        for token_address in self.tokens.keys() {
            multicall.add_call(price_oracle.get_asset_price(*token_address), false);
        }

        let result: Vec<U256> = multicall.call_array().await?;
        for (token_address, price) in zip(self.tokens.keys(), result) {
            prices.insert(*token_address, price);
        }
        multicall.clear_calls();

        Ok(PoolState { prices })
    }

    async fn get_best_collateral_debt_pair(&self, borrower: &Borrower) -> Result<Option<(Address, Address)>> {
        let mut best_collateral_address: Option<Address> = None;
        let mut best_collateral_base: U256 = U256::from(0);
        let mut best_debt_address: Option<Address> = None;
        let mut best_debt_base: U256 = U256::from(0);
        
        let mut multicall = Multicall::new(
            self.client.clone(),
            Some(H160::from_str(
                "0xcA11bde05977b3631167028862bE2a173976CA11",
            )?),
        )
        .await?;
    
        let pool_data =
            IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.client.clone());
        
        for token_address in self.tokens.keys() {
            multicall.add_call(pool_data.get_user_reserve_data(*token_address, borrower.address), false);
        }

        let result: Vec<(U256, U256, U256, U256, U256, U256, U256, u64, bool)> = multicall.call_array().await?;
        for (token, (current_a_token_balance, current_stable_debt, current_variable_debt, _, _, _, _, _, usage_as_collateral_enabled)) in zip(self.tokens.keys(), result) {
            if usage_as_collateral_enabled && current_a_token_balance.gt(&best_collateral_base) { // remove borrowers with no debt
                best_collateral_base = current_a_token_balance;
                best_collateral_address = Some(*token);
            }

            if (current_stable_debt + current_variable_debt).gt(&best_debt_base) {
                best_debt_base = current_stable_debt + current_variable_debt;
                best_debt_address = Some(*token);
            }
        }

        if best_collateral_address.is_none() || best_debt_address.is_none() {
            return Ok(None);
        }

        Ok(Some((best_collateral_address.unwrap(), best_debt_address.unwrap())))
    }

    async fn get_liquidation_opportunity(
        &self,
        borrower: &Borrower,
        pool_data: &IPoolDataProvider<M>,
        health_factor: &U256,
        pool_state: &PoolState,
    ) -> Result<LiquidationOpportunity> {
        let borrower_address = borrower.address;
        
        let mut collateral_address = H160::zero();
        let mut debt_address = H160::zero();
        match self.get_best_collateral_debt_pair(borrower).await {
            Ok(Some((collateral, debt))) => {
                collateral_address = collateral;
                debt_address = debt;
            },
            Ok(None) => {
                return Err(anyhow!("No collateral or debt found for borrower"));
            },
            Err(e) => {
                eprintln!("发生错误: {:?}", e);
            }
        }
        
        let collateral_asset_price = pool_state
            .prices
            .get(&collateral_address)
            .ok_or(anyhow!("No collateral price"))?;
        let debt_asset_price = pool_state
            .prices
            .get(&debt_address)
            .ok_or(anyhow!("No debt price"))?;
        let collateral_config = self
            .tokens
            .get(&collateral_address)
            .ok_or(anyhow!("Failed to get collateral address"))?;
        let debt_config = self
            .tokens
            .get(&debt_address)
            .ok_or(anyhow!("Failed to get debt address"))?;
        let collateral_unit = U256::from(10).pow(collateral_config.decimals.into());
        let debt_unit = U256::from(10).pow(debt_config.decimals.into());
        let liquidation_bonus = collateral_config.liquidation_bonus;
        let a_token = IERC20::new(collateral_config.a_address.clone(), self.client.clone());

        let (_, stable_debt, variable_debt, _, _, _, _, _, _) = pool_data
            .get_user_reserve_data(debt_address, borrower_address)
            .await?;
        let close_factor = if health_factor.gt(&U256::from_dec_str(LIQUIDATION_CLOSE_FACTOR_THRESHOLD).unwrap()) {
            U256::from(DEFAULT_LIQUIDATION_CLOSE_FACTOR)
        } else {
            U256::from(MAX_LIQUIDATION_CLOSE_FACTOR)
        };

        let mut debt_to_cover =
            (stable_debt + variable_debt) * close_factor / MAX_LIQUIDATION_CLOSE_FACTOR;
        let base_collateral = (debt_asset_price * debt_to_cover * collateral_unit)
            / (collateral_asset_price * debt_unit);
        let mut collateral_to_liquidate = percent_mul(base_collateral, liquidation_bonus);
        let user_collateral_balance = a_token.balance_of(borrower_address).await?;


        if collateral_to_liquidate > user_collateral_balance {
            collateral_to_liquidate = user_collateral_balance;
            debt_to_cover = (collateral_asset_price * collateral_to_liquidate * debt_unit)
                / percent_mul(debt_asset_price * collateral_unit, liquidation_bonus);
        }

        info!(
            "collateral_to_liquidate: {:?}, debt_to_cover: {:?}, user_collateral_balance: {:?}",
            collateral_to_liquidate, debt_to_cover, user_collateral_balance
        );

        let mut op = LiquidationOpportunity {
            borrower: borrower_address.clone(),
            collateral: collateral_address.clone(),
            debt: debt_address.clone(),
            debt_to_cover,
            profit_eth: I256::from(0),
        };

        let debt_to_cover_value = debt_to_cover * debt_asset_price / debt_unit;

        if debt_to_cover_value.lt(&U256::from(MIN_DEBT_VALUE)) {
            info!("Debt to cover value too low: {:?}", debt_to_cover_value);
            return Ok(op);
        }

        let gain = self.build_liquidation_call(&op).await?.call().await?;

        info!("gain: {:?}", gain);

        let eth_usd_price = self.get_eth_price(pool_state).await?;

        op.profit_eth = gain * I256::from_dec_str(
            &(
                U256::from_dec_str(WETH_UNIT).unwrap() * collateral_asset_price / (eth_usd_price * collateral_unit)
            ).to_string()
        )?;

        info!(
            "Found opportunity - collateral: {:?}, debt: {:?}, collateral_to_liquidate: {:?}, debt_to_cover: {:?}, profit_eth: {:?}",
            collateral_address, debt_address, collateral_to_liquidate, debt_to_cover, op.profit_eth
        );

        Ok(op)
    }

    async fn build_liquidation_call(
        &self,
        op: &LiquidationOpportunity,
    ) -> Result<ContractCall<M, I256>> {
        let liquidator = Liquidator::new(self.liquidator, self.client.clone());
        let encoder = L2Encoder::new(self.config.l2_encoder, self.client.clone());
        let (data0, data1) = encoder
            .encode_liquidation_call(op.collateral, op.debt, op.borrower, op.debt_to_cover, false)
            .call()
            .await?;

        info!("data0: {:?}, data1: {:?}", data0, data1);

        let paraswap_data = self.get_paraswap_data(op).await?;

        info!("psp: {:?}", paraswap_data);

        Ok(liquidator.liquidate(
            op.collateral, 
            op.debt, 
            op.debt_to_cover, 
            data0, 
            data1, 
            paraswap_data
        ))
    }

    async fn check_vault_liquidity(&self, token: &Address, amount: &U256) -> Result<bool> {
        let balance = IERC20::new(token.clone(), self.client.clone())
            .balance_of(Address::from_str(VAULT_ADDRESS).unwrap())
            .await?;

        Ok(balance.gt(amount))
    }

    async fn get_paraswap_data(&self, op: &LiquidationOpportunity) -> Result<ParaSwapData> {
        // https://app.swaggerhub.com/apis/paraswapv5/api/1.0#/prices/get_prices 
        let user_address = "0x63c34506f4f6280D42E7533Ae1d1d657ca4C6c3B"; //@todo 
        let client = Client::new();
        
        let mut dest_amount = op.debt_to_cover;

        if self.check_vault_liquidity(&op.debt, &op.debt_to_cover).await? == false {
            dest_amount = op.debt_to_cover * (U256::from(10000 + 5) / 10000); // 0.05%
        }

        let url = format!(
            "https://apiv5.paraswap.io/prices?srcToken={:#x}&srcDecimals={}&destToken={:#x}&destDecimals={}&amount={}&side={}&network={}&userAddress={}",
            op.collateral,
            self.tokens.get(&op.collateral).unwrap().decimals,
            op.debt,
            self.tokens.get(&op.debt).unwrap().decimals,
            dest_amount,
            "BUY",
            self.chain_id,
            user_address
        );

        let response_json: Value = client
            .get(url)
            .header(header::ACCEPT, "accept: application/json")
            .send().await?.json().await?;
        
        let price_route =  response_json.get("priceRoute").unwrap();

        let build_tx_url = format!(
            "https://apiv5.paraswap.io/transactions/{}?ignoreChecks=true",
            self.chain_id
        );
        
        let src_amount = U256::from_dec_str(&price_route.get("srcAmount").unwrap().to_string().replace("\"", ""))? * U256::from((100 + MAX_SLIPPAGE) / 100);
        

        let res = client.post(build_tx_url)
        .json(&serde_json::json!({
            "srcToken": op.collateral,
            "destToken": op.debt,
            "srcAmount": src_amount.to_string(),
            "destAmount": dest_amount.to_string(),
            //price route from response json price route
            "priceRoute": price_route,
            "userAddress": user_address, 
            "partner": "DL",
            "srcDecimals": self.tokens.get(&op.collateral).unwrap().decimals,
            "destDecimals": self.tokens.get(&op.debt).unwrap().decimals,
        }))
        .send()
        .await?;

        let response_text = res.text().await?;

        if let Ok(build_tx_response) = serde_json::from_str::<BuildTxResponse>(&response_text) {
            return Ok(ParaSwapData {
                augustus: H160::from_str(&build_tx_response.to).unwrap(),
                src_asset: op.collateral,
                dest_asset: op.debt,
                src_amount,
                swap_call_data: Bytes::from_str(&build_tx_response.data).unwrap()
            });
        } else if let Ok(build_tx_error_response) = serde_json::from_str::<BuildTxErrorResponse>(&response_text) {
            return Err(anyhow!("Failed to build tx: {}", build_tx_error_response.error));
        } else {
            let response_json: Value = serde_json::from_str(&response_text)?;
            return Err(anyhow!("Failed to build tx: {:?}", response_json));
        }
    }


    async fn build_liquidation(&self, op: &LiquidationOpportunity) -> Result<TypedTransaction> {
        let mut call = self.build_liquidation_call(op).await?;
        Ok(call.tx.set_chain_id(self.chain_id).clone())
    }
}

fn percent_mul(a: U256, bps: u64) -> U256 {
    (U256::from(5000) + (a * bps)) / U256::from(10000)
}

fn percent_div(a: U256, bps: u64) -> U256 {
    let half_bps = bps / 2;
    (U256::from(half_bps) + (a * 10000)) / bps
}

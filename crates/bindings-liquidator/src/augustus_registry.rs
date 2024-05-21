pub use augustus_registry::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod augustus_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ARBITRUM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ARBITRUM"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AVALANCHE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AVALANCHE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BASE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BNB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BNB"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BSC"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BSC"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ETHEREUM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ETHEREUM"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OPTIMISM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("OPTIMISM"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POLYGON"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POLYGON"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static AUGUSTUSREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xA6a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\x92W`\x005`\xE0\x1C\x80c\xB0\x1D\xC0\xD5\x11a\0eW\x80c\xB0\x1D\xC0\xD5\x14a\x01\x04W\x80c\xC8tM\xD8\x14a\x01\x1FW\x80c\xEC4*\xD0\x14a\x01:W\x80c\xF7\xCD\xF4|\x14a\x01UW`\0\x80\xFD[\x80c<\x13\r\xAF\x14a\0\x97W\x80cH\xA27\x84\x14a\0\xCEW\x80cX\xF7\xF6\xD2\x14a\0\xCEW\x80cy\xDD@\xF9\x14a\0\xE9W[`\0\x80\xFD[a\0\xB2sn{\xE8`\0\xDFi\x7F\xAC\xF49n\xFD*\xE2\xC3\"\x16]\xC3\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB2s\x05\xB4Hod9\x14\xA8\x18\xED\x93\xAF\xC0tW\xE9\x07K\xE2\x11\x81V[a\0\xB2s\xFD\x1EX!\xF0\x7F\x1A\xF8\x12\xBB\x7F1\x02\xBF\xD9\xFF\xB2yQ:\x81V[a\0\xB2s\xDCn+\x14&\x0F\x97*\xD4\xE5\xA3\x1Ch)O\xBA~r\x07\x01\x81V[a\0\xB2s\xCA5\xA4\x86gG\xFFz`N\xF7\xA2\xA7\xF2F\xBB\x87\x0F<\xA1\x81V[a\0\xB2s~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x81V[a\0\xB2s\xA6\x8B\xEAb\xDC@4\xA6\x89\xAA\x0FX\xA7f\x81C<\xAC\xA6c\x81V\xFE\xA2dipfsX\"\x12 \xCA\r\x1A!Tj%C\"\xA2\x90\x8F\xF4\x05\xF3#\rs<\xD8\xBF\xB8_q\x0C\xBF\x18\x1A\x95=\xD5\x17dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static AUGUSTUSREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\x92W`\x005`\xE0\x1C\x80c\xB0\x1D\xC0\xD5\x11a\0eW\x80c\xB0\x1D\xC0\xD5\x14a\x01\x04W\x80c\xC8tM\xD8\x14a\x01\x1FW\x80c\xEC4*\xD0\x14a\x01:W\x80c\xF7\xCD\xF4|\x14a\x01UW`\0\x80\xFD[\x80c<\x13\r\xAF\x14a\0\x97W\x80cH\xA27\x84\x14a\0\xCEW\x80cX\xF7\xF6\xD2\x14a\0\xCEW\x80cy\xDD@\xF9\x14a\0\xE9W[`\0\x80\xFD[a\0\xB2sn{\xE8`\0\xDFi\x7F\xAC\xF49n\xFD*\xE2\xC3\"\x16]\xC3\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB2s\x05\xB4Hod9\x14\xA8\x18\xED\x93\xAF\xC0tW\xE9\x07K\xE2\x11\x81V[a\0\xB2s\xFD\x1EX!\xF0\x7F\x1A\xF8\x12\xBB\x7F1\x02\xBF\xD9\xFF\xB2yQ:\x81V[a\0\xB2s\xDCn+\x14&\x0F\x97*\xD4\xE5\xA3\x1Ch)O\xBA~r\x07\x01\x81V[a\0\xB2s\xCA5\xA4\x86gG\xFFz`N\xF7\xA2\xA7\xF2F\xBB\x87\x0F<\xA1\x81V[a\0\xB2s~1\xB36\xF9\xE8\xBAR\xBA<J\xC8a\xB03\xBA\x90\x90\x0B\xB3\x81V[a\0\xB2s\xA6\x8B\xEAb\xDC@4\xA6\x89\xAA\x0FX\xA7f\x81C<\xAC\xA6c\x81V\xFE\xA2dipfsX\"\x12 \xCA\r\x1A!Tj%C\"\xA2\x90\x8F\xF4\x05\xF3#\rs<\xD8\xBF\xB8_q\x0C\xBF\x18\x1A\x95=\xD5\x17dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static AUGUSTUSREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AugustusRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AugustusRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AugustusRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AugustusRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AugustusRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AugustusRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AugustusRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AUGUSTUSREGISTRY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                AUGUSTUSREGISTRY_ABI.clone(),
                AUGUSTUSREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ARBITRUM` (0xb01dc0d5) function
        pub fn arbitrum(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([176, 29, 192, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `AVALANCHE` (0x79dd40f9) function
        pub fn avalanche(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([121, 221, 64, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BASE` (0xec342ad0) function
        pub fn base(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([236, 52, 42, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BNB` (0x58f7f6d2) function
        pub fn bnb(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([88, 247, 246, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BSC` (0x48a23784) function
        pub fn bsc(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([72, 162, 55, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ETHEREUM` (0xf7cdf47c) function
        pub fn ethereum(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([247, 205, 244, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMISM` (0x3c130daf) function
        pub fn optimism(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([60, 19, 13, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POLYGON` (0xc8744dd8) function
        pub fn polygon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 116, 77, 216], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AugustusRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `ARBITRUM` function with signature `ARBITRUM()` and selector `0xb01dc0d5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ARBITRUM", abi = "ARBITRUM()")]
    pub struct ArbitrumCall;
    ///Container type for all input parameters for the `AVALANCHE` function with signature `AVALANCHE()` and selector `0x79dd40f9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "AVALANCHE", abi = "AVALANCHE()")]
    pub struct AvalancheCall;
    ///Container type for all input parameters for the `BASE` function with signature `BASE()` and selector `0xec342ad0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BASE", abi = "BASE()")]
    pub struct BaseCall;
    ///Container type for all input parameters for the `BNB` function with signature `BNB()` and selector `0x58f7f6d2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BNB", abi = "BNB()")]
    pub struct BnbCall;
    ///Container type for all input parameters for the `BSC` function with signature `BSC()` and selector `0x48a23784`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BSC", abi = "BSC()")]
    pub struct BscCall;
    ///Container type for all input parameters for the `ETHEREUM` function with signature `ETHEREUM()` and selector `0xf7cdf47c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ETHEREUM", abi = "ETHEREUM()")]
    pub struct EthereumCall;
    ///Container type for all input parameters for the `OPTIMISM` function with signature `OPTIMISM()` and selector `0x3c130daf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "OPTIMISM", abi = "OPTIMISM()")]
    pub struct OptimismCall;
    ///Container type for all input parameters for the `POLYGON` function with signature `POLYGON()` and selector `0xc8744dd8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "POLYGON", abi = "POLYGON()")]
    pub struct PolygonCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum AugustusRegistryCalls {
        Arbitrum(ArbitrumCall),
        Avalanche(AvalancheCall),
        Base(BaseCall),
        Bnb(BnbCall),
        Bsc(BscCall),
        Ethereum(EthereumCall),
        Optimism(OptimismCall),
        Polygon(PolygonCall),
    }
    impl ::ethers::core::abi::AbiDecode for AugustusRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ArbitrumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Arbitrum(decoded));
            }
            if let Ok(decoded) = <AvalancheCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Avalanche(decoded));
            }
            if let Ok(decoded) = <BaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Base(decoded));
            }
            if let Ok(decoded) = <BnbCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bnb(decoded));
            }
            if let Ok(decoded) = <BscCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bsc(decoded));
            }
            if let Ok(decoded) = <EthereumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ethereum(decoded));
            }
            if let Ok(decoded) = <OptimismCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Optimism(decoded));
            }
            if let Ok(decoded) = <PolygonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Polygon(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AugustusRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Arbitrum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Avalanche(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Base(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Bnb(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Bsc(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ethereum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Optimism(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Polygon(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AugustusRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Arbitrum(element) => ::core::fmt::Display::fmt(element, f),
                Self::Avalanche(element) => ::core::fmt::Display::fmt(element, f),
                Self::Base(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bnb(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bsc(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ethereum(element) => ::core::fmt::Display::fmt(element, f),
                Self::Optimism(element) => ::core::fmt::Display::fmt(element, f),
                Self::Polygon(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ArbitrumCall> for AugustusRegistryCalls {
        fn from(value: ArbitrumCall) -> Self {
            Self::Arbitrum(value)
        }
    }
    impl ::core::convert::From<AvalancheCall> for AugustusRegistryCalls {
        fn from(value: AvalancheCall) -> Self {
            Self::Avalanche(value)
        }
    }
    impl ::core::convert::From<BaseCall> for AugustusRegistryCalls {
        fn from(value: BaseCall) -> Self {
            Self::Base(value)
        }
    }
    impl ::core::convert::From<BnbCall> for AugustusRegistryCalls {
        fn from(value: BnbCall) -> Self {
            Self::Bnb(value)
        }
    }
    impl ::core::convert::From<BscCall> for AugustusRegistryCalls {
        fn from(value: BscCall) -> Self {
            Self::Bsc(value)
        }
    }
    impl ::core::convert::From<EthereumCall> for AugustusRegistryCalls {
        fn from(value: EthereumCall) -> Self {
            Self::Ethereum(value)
        }
    }
    impl ::core::convert::From<OptimismCall> for AugustusRegistryCalls {
        fn from(value: OptimismCall) -> Self {
            Self::Optimism(value)
        }
    }
    impl ::core::convert::From<PolygonCall> for AugustusRegistryCalls {
        fn from(value: PolygonCall) -> Self {
            Self::Polygon(value)
        }
    }
    ///Container type for all return fields from the `ARBITRUM` function with signature `ARBITRUM()` and selector `0xb01dc0d5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ArbitrumReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `AVALANCHE` function with signature `AVALANCHE()` and selector `0x79dd40f9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AvalancheReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BASE` function with signature `BASE()` and selector `0xec342ad0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BaseReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BNB` function with signature `BNB()` and selector `0x58f7f6d2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BnbReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BSC` function with signature `BSC()` and selector `0x48a23784`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BscReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ETHEREUM` function with signature `ETHEREUM()` and selector `0xf7cdf47c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EthereumReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `OPTIMISM` function with signature `OPTIMISM()` and selector `0x3c130daf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OptimismReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `POLYGON` function with signature `POLYGON()` and selector `0xc8744dd8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PolygonReturn(pub ::ethers::core::types::Address);
}

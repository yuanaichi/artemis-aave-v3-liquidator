// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Owned} from "solmate/auth/Owned.sol";
import {ERC20} from "solmate/tokens/ERC20.sol";
import {IL2Pool} from "./interfaces/IL2Pool.sol";
import {IParaSwapAugustusRegistry} from "./interfaces/paraswap/IParaSwapAugustusRegistry.sol";
import {IParaSwapAugustus} from "./interfaces/paraswap/IParaSwapAugustus.sol";
import {IVault} from "./interfaces/balancer-v2/IVault.sol";
import {IFlashLoanRecipient} from "./interfaces/balancer-v2/IFlashLoanRecipient.sol";
import {SafeERC20} from "./lib/SafeERC20.sol";

contract Liquidator is Owned, IFlashLoanRecipient {
    using SafeERC20 for ERC20;
    IVault private constant vault = IVault(0xBA12222222228d8Ba445958a75a0704d566BF2C8); // const value for all chains

    struct ParaSwapData {
        address augustus;
        address srcAsset;
        address destAsset;
        uint256 srcAmount;
        bytes swapCallData;
    }
    
    IParaSwapAugustusRegistry public immutable AUGUSTUS_REGISTRY;

    IL2Pool public constant pool = IL2Pool(0x794a61358D6845594F94dc1DB02A252b5b4814aD); //optimism L2 pool proxy address

    constructor(IParaSwapAugustusRegistry augustusRegistry) Owned(msg.sender) {
        require(!augustusRegistry.isValidAugustus(address(0)), 'Not a valid Augustus address');
        AUGUSTUS_REGISTRY = augustusRegistry;
    }

    function liquidate(
        address collateral,
        address debt,
        uint256 debtToCover,
        bytes32 liquidationArg1,
        bytes32 liquidationArg2,
        ParaSwapData calldata psp
    ) external onlyOwner returns (int256 collateralGain) {

        uint256 collateralBalance = ERC20(collateral).balanceOf(address(this));

        ERC20[] memory flashLoanAssets = new ERC20[](1);
        flashLoanAssets[0] = ERC20(debt);
        uint256[] memory flashLoanAmounts = new uint256[](1);
        flashLoanAmounts[0] = debtToCover;
        
        vault.flashLoan(
            this,
            flashLoanAssets, 
            flashLoanAmounts, 
            abi.encode(psp, collateral, debt, liquidationArg1, liquidationArg2)
        );

        collateralGain = int256(ERC20(collateral).balanceOf(address(this))) - int256(collateralBalance);
    }

    function _paraSwap(ParaSwapData memory psp) internal returns (bool) {
        IParaSwapAugustus augustus = IParaSwapAugustus(psp.augustus);

        require(AUGUSTUS_REGISTRY.isValidAugustus(address(augustus)), 'INVALID_AUGUSTUS');

        ERC20 assetToSwapFrom = ERC20(psp.srcAsset);
        address tokenTransferProxy = augustus.getTokenTransferProxy();
        assetToSwapFrom.safeApprove(tokenTransferProxy, 0);
        assetToSwapFrom.safeApprove(tokenTransferProxy, psp.srcAmount);

        (bool success, ) = address(augustus).call(psp.swapCallData);
        if (!success) {
            // Copy revert reason from call
            assembly {
                returndatacopy(0, 0, returndatasize())
                revert(0, returndatasize())
            }
        }

        return success;
    }

    function receiveFlashLoan(
        ERC20[] memory tokens,
        uint256[] memory amounts,
        uint256[] memory feeAmounts,
        bytes memory userData
    ) external override {
        require(msg.sender == address(vault), 'caller is not the vault');
        
        (
            ParaSwapData memory psp,
            address collateral,
            address debt,
            bytes32 liquidationArg1,
            bytes32 liquidationArg2
        ) = abi.decode(userData, (ParaSwapData, address, address, bytes32, bytes32));
        
        // uint256 flashLoanAmount = amounts[0];
        uint256 repayAmount = amounts[0] + feeAmounts[0];

        pool.liquidationCall(liquidationArg1, liquidationArg2);

        // swap the collateral for the debt, side BUY on ParaSwap
        _paraSwap(psp);

        ERC20(debt).transfer(msg.sender, repayAmount);
    }

    function approvePool(address token) external onlyOwner {
        ERC20(token).approve(address(pool), type(uint256).max);
    }

    function recover(address token, uint256 amount) external onlyOwner {
        if (token == address(0)) {
            payable(msg.sender).transfer(amount);
            return;
        }
        ERC20(token).transfer(msg.sender, amount);
    }
}

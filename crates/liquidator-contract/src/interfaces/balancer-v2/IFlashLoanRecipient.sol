pragma solidity ^0.8.0;

import {ERC20} from "solmate/tokens/ERC20.sol";


/**
 * @dev Full external interface for the Vault core contract - no external or public methods exist in the contract that
 * don't override one of these declarations.
 */
interface IFlashLoanRecipient {
  function receiveFlashLoan(
        ERC20[] memory tokens,
        uint256[] memory amounts,
        uint256[] memory feeAmounts,
        bytes memory userData
    ) external;
}
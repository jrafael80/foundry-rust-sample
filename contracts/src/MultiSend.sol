// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0 <0.9.0;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract MultiSend {
    using SafeERC20 for IERC20;

    event MultiSendETH(address from, uint256 total);
	event MultiSendERC20(address token, address from, uint256 total);

    function sendMany(address payable[] memory _to, uint256[] memory _value) external payable {
        address from = msg.sender;
        require(_to.length == _value.length, 'invalid input');
        require(_to.length <= 255, 'exceed max allowed');
        
        uint256 sendAmount = 0;
        for (uint8 i = 0; i < _to.length; i++) {
            (bool success,) = _to[i].call{value: _value[i]}("");
            require(success, "Failed to send Ether");
            sendAmount = sendAmount + _value[i];
        }
        emit MultiSendETH(from, sendAmount);

    }

    function sendManyERC20(address _tokenAddress, address[] memory _to, uint256[] memory _value) external {
	    address from = msg.sender;
        require(_to.length == _value.length, 'invalid input');
        require(_to.length <= 255, 'exceed max allowed');

        IERC20 token = IERC20(_tokenAddress);
        uint256 sendAmount = 0;
        for (uint8 i = 0; i < _to.length; i++) {
            token.safeTransferFrom(from, _to[i], _value[i]);
            sendAmount = sendAmount + _value[i];
        }
        emit MultiSendERC20(_tokenAddress, from, sendAmount);

    }

}
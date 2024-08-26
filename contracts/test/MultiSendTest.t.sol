//SPDX-License-Identifier:MIT

pragma solidity ^0.8.18;

import 'forge-std/Test.sol';
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import '../src/MultiSend.sol';
import '../script/DeployMultiSend.s.sol';
import '../src/MultiSend.sol';
import './TestErc20.sol';

contract MultiSendTest is Test{
    IERC20 token;
    MultiSend  multiSend;

    function setUp() external{
        DeployMultiSend deployer = new DeployMultiSend();
        multiSend = deployer.run();
        token = new TestErc20();
    }

    //test send Ether
    function testSendEtherMany() public {
        address payable[] memory addresses = new address payable[](2);
        addresses[0] = payable(address(0x976EA74026E726554dB657fA54763abd0C3a0aa9));
        addresses[1] = payable(address(0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc));

        uint [] memory amounts = new uint [](2);
        amounts[0] = 0.1 ether;
        amounts[1] = 0.1 ether;
        uint totalAmount = 0.2 ether;
        
        uint256 initialBalance = address(this).balance;
        assertLe(totalAmount,initialBalance);
        assertEq(0.0 ether, addresses[0].balance);
        assertEq(0.0 ether, addresses[1].balance);
        assertEq(0.0 ether, address(multiSend).balance);
        
        multiSend.sendMany{ value: totalAmount }(addresses,amounts);

        uint256 updatedBalance = address(this).balance;
        assertEq(updatedBalance,initialBalance - totalAmount);
        assertEq(0.1 ether, addresses[0].balance);
        assertEq(0.1 ether, addresses[1].balance);
        assertEq(0.0 ether, address(multiSend).balance);
    }


    //test send Ether
    function testSendEtherManyWithMoreThatRequired() public {
        address payable[] memory addresses = new address payable[](2);
        addresses[0] = payable(address(0x976EA74026E726554dB657fA54763abd0C3a0aa9));
        addresses[1] = payable(address(0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc));

        uint [] memory amounts = new uint [](2);
        amounts[0] = 0.1 ether;
        amounts[1] = 0.1 ether;
        uint totalAmount = 0.2 ether;
        
        uint256 initialBalance = address(this).balance;
        assertLe(totalAmount,initialBalance);
        assertEq(0.0 ether, addresses[0].balance);
        assertEq(0.0 ether, addresses[1].balance);
        assertEq(0.0 ether, address(multiSend).balance);
        
        multiSend.sendMany{ value: totalAmount + totalAmount }(addresses,amounts);

        uint256 updatedBalance = address(this).balance;
        assertEq(updatedBalance,initialBalance - totalAmount - totalAmount);
        assertEq(0.1 ether, addresses[0].balance);
        assertEq(0.1 ether, addresses[1].balance);
        assertEq(totalAmount, address(multiSend).balance);
    }

    //test send Ether
    function testSendEtherManyWithLessThatRequired() public {
        address payable[] memory addresses = new address payable[](2);
        addresses[0] = payable(address(0x976EA74026E726554dB657fA54763abd0C3a0aa9));
        addresses[1] = payable(address(0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc));

        uint [] memory amounts = new uint [](2);
        amounts[0] = 0.1 ether;
        amounts[1] = 0.1 ether;
        uint totalAmount = 0.2 ether;
        
        uint256 initialBalance = address(this).balance;
        assertLe(totalAmount,initialBalance);
        assertEq(0.0 ether, addresses[0].balance);
        assertEq(0.0 ether, addresses[1].balance);
        assertEq(0.0 ether, address(multiSend).balance);
        
        vm.expectRevert("Failed to send Ether");
        multiSend.sendMany{ value: 0.1 ether }(addresses,amounts);

        uint256 updatedBalance = address(this).balance;
        assertEq(updatedBalance,initialBalance);
        assertEq(0.0 ether, addresses[0].balance);
        assertEq(0.0 ether, addresses[1].balance);
        assertEq(0.0 ether, address(multiSend).balance);
    }

    //test send Ether
    function testSendErc20Many() public {
        address[] memory addresses = new address[](2);
        addresses[0] = address(0x976EA74026E726554dB657fA54763abd0C3a0aa9);
        addresses[1] = address(0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc);

        uint [] memory amounts = new uint [](2);
        amounts[0] = 1;
        amounts[1] = 1;
        uint totalAmount = 2;
            
        uint256 initialBalance = token.balanceOf(address(this));
        assertLe(totalAmount,initialBalance);
        token.approve(address(multiSend), totalAmount);
        multiSend.sendManyERC20(address(token),addresses,amounts);

    }
    
}
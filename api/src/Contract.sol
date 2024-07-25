// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IERC20 {
    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract Contract {
    function disperseETH(
        address[] memory recipients,
        uint256[] memory values
    ) public payable {
        require(recipients.length == values.length, "Mismatch input lengths");
        for (uint256 i = 0; i < recipients.length; i++) {
            payable(recipients[i]).transfer(values[i]);
        }
    }

    function disperseERC20(
        IERC20 token,
        address[] memory recipients,
        uint256[] memory values
    ) public {
        require(recipients.length == values.length, "Mismatch input lengths");
        for (uint256 i = 0; i < recipients.length; i++) {
            require(
                token.transferFrom(msg.sender, recipients[i], values[i]),
                "Transfer from failed"
            );
        }
    }

    function disperseERC20Percent(
        IERC20 token,
        address[] memory recipients,
        uint256[] memory percentages,
        uint256 totalAmount
    ) public {
        require(
            recipients.length == percentages.length,
            "Mismatch input lengths"
        );
        uint256 totalPercent = 0;
        for (uint256 i = 0; i < percentages.length; i++) {
            totalPercent += percentages[i];
        }
        require(totalPercent == 100, "Total percentage must be 100");

        for (uint256 i = 0; i < recipients.length; i++) {
            uint256 amount = (totalAmount * percentages[i]) / 100;
            require(
                token.transferFrom(msg.sender, recipients[i], amount),
                "Transfer failed"
            );
        }
    }

    function collectETH(address[] memory senders, address recipient) public {
        for (uint256 i = 0; i < senders.length; i++) {
            payable(recipient).transfer(senders[i].balance);
        }
    }

    function collectERC20(
        IERC20 token,
        address[] memory senders,
        address recipient
    ) public {
        for (uint256 i = 0; i < senders.length; i++) {
            uint256 balance = token.balanceOf(senders[i]);
            require(
                token.transferFrom(senders[i], recipient, balance),
                "Transfer failed"
            );
        }
    }

    function collectERC20Percent(
        IERC20 token,
        address[] memory senders,
        address recipient,
        uint256 percentage
    ) public {
        for (uint256 i = 0; i < senders.length; i++) {
            uint256 balance = token.balanceOf(senders[i]);
            uint256 amount = (balance * percentage) / 100;
            require(
                token.transferFrom(senders[i], recipient, amount),
                "Transfer failed"
            );
        }
    }
}

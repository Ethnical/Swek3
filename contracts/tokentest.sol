pragma solidity ^0.8.0;
contract ERC20 is Context, IERC20, IERC20Metadata {
	mapping(address => uint256) private _balances;
	mapping(address => mapping(address => uint256)) private _allowances;
	uint256 private _totalSupply;
	string private _name;
	string private _symbol;
	constructor(string memory name_, string memory symbol_) {
		_name = name_;
		_symbol = symbol_;
	}
}



/**
 *Submitted for verification at Etherscan.io on 2020-05-28
 */

pragma solidity 0.5.16;

interface IAaveAToken {
	/**
	 * @notice Non-standard ERC20 function to redeem an _amount of aTokens for the underlying
	 * asset, burning the aTokens during the process.
	 * @param _amount Amount of aTokens
	 */
	function redeem(uint256 _amount) external;

	/**
	 * @notice returns the current total aToken balance of _user all interest collected included.
	 * To obtain the user asset principal balance with interests excluded , ERC20 non-standard
	 * method principalBalanceOf() can be used.
	 */
	function balanceOf(address _user) external view returns (uint256);
}

interface IAaveLendingPool {
	/**
	 * @notice Deposits a certain _amount of an asset specified by the _reserve parameter.
	 * @dev The caller receives a certain amount of corresponding aTokens in exchange.
	 * The amount of aTokens received depends on the corresponding aToken exchange rate.
	 * LendingPoolCore must be approved to spend this reserve
	 */
	function deposit(
		address _reserve,
		uint256 _amount,
		uint16 _referralCode
	) external;
}

interface ILendingPoolAddressesProvider {
	/**
	 * @notice Get the current address for Aave LendingPool
	 * @dev Lending pool is the core contract on which to call deposit
	 */
	function getLendingPool() external view returns (address);

	/**
	 * @notice Get the address for lendingPoolCore
	 * @dev IMPORTANT - this is where _reserve must be approved before deposit
	 */
	function getLendingPoolCore() external view returns (address payable);
}

interface IPlatformIntegration {
	/**
	 * @dev Deposit the given bAsset to Lending platform
	 * @param _bAsset bAsset address
	 * @param _amount Amount to deposit
	 */
	function deposit(
		address _bAsset,
		uint256 _amount,
		bool isTokenFeeCharged
	) external returns (uint256 quantityDeposited);

	/**
	 * @dev Withdraw given bAsset from Lending platform
	 */
	function withdraw(
		address _receiver,
		address _bAsset,
		uint256 _amount,
		bool _isTokenFeeCharged
	) external;

	/**
	 * @dev Returns the current balance of the given bAsset
	 */
	function checkBalance(address _bAsset) external returns (uint256 balance);
}

contract InitializableModuleKeys {
	// Governance
	bytes32 internal KEY_GOVERNANCE;
	bytes32 internal KEY_STAKING;
	bytes32 internal KEY_PROXY_ADMIN;

	// mStable
	bytes32 internal KEY_ORACLE_HUB;
	bytes32 internal KEY_MANAGER;
	bytes32 internal KEY_RECOLLATERALISER;
	bytes32 internal KEY_META_TOKEN;
	bytes32 internal KEY_SAVINGS_MANAGER;

	/**
	 * @dev Initialize function for upgradable proxy contracts. This function should be called
	 *      via Proxy to initialize constants in the Proxy contract.
	 */
	function _initialize() internal {
		// keccak256() values are evaluated only once at the time of this function call.
		// Hence, no need to assign hard-coded values to these variables.
		KEY_GOVERNANCE = keccak256("Governance");
		KEY_STAKING = keccak256("Staking");
		KEY_PROXY_ADMIN = keccak256("ProxyAdmin");

		KEY_ORACLE_HUB = keccak256("OracleHub");
		KEY_MANAGER = keccak256("Manager");
		KEY_RECOLLATERALISER = keccak256("Recollateraliser");
		KEY_META_TOKEN = keccak256("MetaToken");
		KEY_SAVINGS_MANAGER = keccak256("SavingsManager");
	}
}

interface INexus {
	function governor() external view returns (address);

	function getModule(bytes32 key) external view returns (address);

	function proposeModule(bytes32 _key, address _addr) external;

	function cancelProposedModule(bytes32 _key) external;

	function acceptProposedModule(bytes32 _key) external;

	function acceptProposedModules(bytes32[] calldata _keys) external;

	function requestLockModule(bytes32 _key) external;

	function cancelLockModule(bytes32 _key) external;

	function lockModule(bytes32 _key) external;
}

contract InitializableModule is InitializableModuleKeys {
	INexus public nexus;

	/**
	 * @dev Modifier to allow function calls only from the Governor.
	 */
	modifier onlyGovernor() {
		require(msg.sender == _governor(), "Only governor can execute");
		_;
	}

	/**
	 * @dev Modifier to allow function calls only from the Governance.
	 *      Governance is either Governor address or Governance address.
	 */
	modifier onlyGovernance() {
		require(msg.sender == _governor() || msg.sender == _governance(), "Only governance can execute");
		_;
	}

	/**
	 * @dev Modifier to allow function calls only from the ProxyAdmin.
	 */
	modifier onlyProxyAdmin() {
		require(msg.sender == _proxyAdmin(), "Only ProxyAdmin can execute");
		_;
	}

	/**
	 * @dev Modifier to allow function calls only from the Manager.
	 */
	modifier onlyManager() {
		require(msg.sender == _manager(), "Only manager can execute");
		_;
	}

	/**
	 * @dev Initialization function for upgradable proxy contracts
	 * @param _nexus Nexus contract address
	 */
	function _initialize(address _nexus) internal {
		require(_nexus != address(0), "Nexus address is zero");
		nexus = INexus(_nexus);
		InitializableModuleKeys._initialize();
	}

	/**
	 * @dev Returns Governor address from the Nexus
	 * @return Address of Governor Contract
	 */
	function _governor() internal view returns (address) {
		return nexus.governor();
	}

	/**
	 * @dev Returns Governance Module address from the Nexus
	 * @return Address of the Governance (Phase 2)
	 */
	function _governance() internal view returns (address) {
		return nexus.getModule(KEY_GOVERNANCE);
	}

	/**
	 * @dev Return Staking Module address from the Nexus
	 * @return Address of the Staking Module contract
	 */
	function _staking() internal view returns (address) {
		return nexus.getModule(KEY_STAKING);
	}

	/**
	 * @dev Return ProxyAdmin Module address from the Nexus
	 * @return Address of the ProxyAdmin Module contract
	 */
	function _proxyAdmin() internal view returns (address) {
		return nexus.getModule(KEY_PROXY_ADMIN);
	}

	/**
	 * @dev Return MetaToken Module address from the Nexus
	 * @return Address of the MetaToken Module contract
	 */
	function _metaToken() internal view returns (address) {
		return nexus.getModule(KEY_META_TOKEN);
	}

	/**
	 * @dev Return OracleHub Module address from the Nexus
	 * @return Address of the OracleHub Module contract
	 */
	function _oracleHub() internal view returns (address) {
		return nexus.getModule(KEY_ORACLE_HUB);
	}

	/**
	 * @dev Return Manager Module address from the Nexus
	 * @return Address of the Manager Module contract
	 */
	function _manager() internal view returns (address) {
		return nexus.getModule(KEY_MANAGER);
	}

	/**
	 * @dev Return SavingsManager Module address from the Nexus
	 * @return Address of the SavingsManager Module contract
	 */
	function _savingsManager() internal view returns (address) {
		return nexus.getModule(KEY_SAVINGS_MANAGER);
	}

	/**
	 * @dev Return Recollateraliser Module address from the Nexus
	 * @return  Address of the Recollateraliser Module contract (Phase 2)
	 */
	function _recollateraliser() internal view returns (address) {
		return nexus.getModule(KEY_RECOLLATERALISER);
	}
}

contract InitializableGovernableWhitelist is InitializableModule {
	event Whitelisted(address indexed _address);

	mapping(address => bool) public whitelist;

	/**
	 * @dev Modifier to allow function calls only from the whitelisted address.
	 */
	modifier onlyWhitelisted() {
		require(whitelist[msg.sender], "Not a whitelisted address");
		_;
	}

	/**
	 * @dev Initialization function for upgradable proxy contracts
	 * @param _nexus Nexus contract address
	 * @param _whitelisted Array of whitelisted addresses.
	 */
	function _initialize(address _nexus, address[] memory _whitelisted) internal {
		InitializableModule._initialize(_nexus);

		require(_whitelisted.length > 0, "Empty whitelist array");

		for (uint256 i = 0; i < _whitelisted.length; i++) {
			_addWhitelist(_whitelisted[i]);
		}
	}

	/**
	 * @dev Adds a new whitelist address
	 * @param _address Address to add in whitelist
	 */
	function _addWhitelist(address _address) internal {
		require(_address != address(0), "Address is zero");
		require(!whitelist[_address], "Already whitelisted");

		whitelist[_address] = true;

		emit Whitelisted(_address);
	}
}

contract Initializable {
	/**
	 * @dev Indicates that the contract has been initialized.
	 */
	bool private initialized;

	/**
	 * @dev Indicates that the contract is in the process of being initialized.
	 */
	bool private initializing;

	/**
	 * @dev Modifier to use in the initializer function of a contract.
	 */
	modifier initializer() {
		require(initializing || isConstructor() || !initialized, "Contract instance has already been initialized");

		bool isTopLevelCall = !initializing;
		if (isTopLevelCall) {
			initializing = true;
			initialized = true;
		}

		_;

		if (isTopLevelCall) {
			initializing = false;
		}
	}

	/// @dev Returns true if and only if the function is running in the constructor
	function isConstructor() private view returns (bool) {
		// extcodesize checks the size of the code stored in an address, and
		// address returns the current address. Since the code is still not
		// deployed when running a constructor, any checks on its code size will
		// yield zero, making it an effective way to detect if a contract is
		// under construction or not.
		address self = address(this);
		uint256 cs;
		assembly {
			cs := extcodesize(self)
		}
		return cs == 0;
	}

	// Reserved storage space to allow for layout changes in the future.
	uint256[50] private ______gap;
}

/**
 * @dev Wrappers over Solidity's arithmetic operations with added overflow
 * checks.
 *
 * Arithmetic operations in Solidity wrap on overflow. This can easily result
 * in bugs, because programmers usually assume that an overflow raises an
 * error, which is the standard behavior in high level programming languages.
 * `SafeMath` restores this intuition by reverting the transaction when an
 * operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafeMath {
	/**
	 * @dev Returns the addition of two unsigned integers, reverting on
	 * overflow.
	 *
	 * Counterpart to Solidity's `+` operator.
	 *
	 * Requirements:
	 * - Addition cannot overflow.
	 */
	function add(uint256 a, uint256 b) internal pure returns (uint256) {
		uint256 c = a + b;
		require(c >= a, "SafeMath: addition overflow");

		return c;
	}

	/**
	 * @dev Returns the subtraction of two unsigned integers, reverting on
	 * overflow (when the result is negative).
	 *
	 * Counterpart to Solidity's `-` operator.
	 *
	 * Requirements:
	 * - Subtraction cannot overflow.
	 */
	function sub(uint256 a, uint256 b) internal pure returns (uint256) {
		return sub(a, b, "SafeMath: subtraction overflow");
	}

	/**
	 * @dev Returns the subtraction of two unsigned integers, reverting with custom message on
	 * overflow (when the result is negative).
	 *
	 * Counterpart to Solidity's `-` operator.
	 *
	 * Requirements:
	 * - Subtraction cannot overflow.
	 *
	 * _Available since v2.4.0._
	 */
	function sub(
		uint256 a,
		uint256 b,
		string memory errorMessage
	) internal pure returns (uint256) {
		require(b <= a, errorMessage);
		uint256 c = a - b;

		return c;
	}

	/**
	 * @dev Returns the multiplication of two unsigned integers, reverting on
	 * overflow.
	 *
	 * Counterpart to Solidity's `*` operator.
	 *
	 * Requirements:
	 * - Multiplication cannot overflow.
	 */
	function mul(uint256 a, uint256 b) internal pure returns (uint256) {
		// Gas optimization: this is cheaper than requiring 'a' not being zero, but the
		// benefit is lost if 'b' is also tested.
		// See: https://github.com/OpenZeppelin/openzeppelin-contracts/pull/522
		if (a == 0) {
			return 0;
		}

		uint256 c = a * b;
		require(c / a == b, "SafeMath: multiplication overflow");

		return c;
	}

	/**
	 * @dev Returns the integer division of two unsigned integers. Reverts on
	 * division by zero. The result is rounded towards zero.
	 *
	 * Counterpart to Solidity's `/` operator. Note: this function uses a
	 * `revert` opcode (which leaves remaining gas untouched) while Solidity
	 * uses an invalid opcode to revert (consuming all remaining gas).
	 *
	 * Requirements:
	 * - The divisor cannot be zero.
	 */
	function div(uint256 a, uint256 b) internal pure returns (uint256) {
		return div(a, b, "SafeMath: division by zero");
	}

	/**
	 * @dev Returns the integer division of two unsigned integers. Reverts with custom message on
	 * division by zero. The result is rounded towards zero.
	 *
	 * Counterpart to Solidity's `/` operator. Note: this function uses a
	 * `revert` opcode (which leaves remaining gas untouched) while Solidity
	 * uses an invalid opcode to revert (consuming all remaining gas).
	 *
	 * Requirements:
	 * - The divisor cannot be zero.
	 *
	 * _Available since v2.4.0._
	 */
	function div(
		uint256 a,
		uint256 b,
		string memory errorMessage
	) internal pure returns (uint256) {
		// Solidity only automatically asserts when dividing by 0
		require(b > 0, errorMessage);
		uint256 c = a / b;
		// assert(a == b * c + a % b); // There is no case in which this doesn't hold

		return c;
	}

	/**
	 * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
	 * Reverts when dividing by zero.
	 *
	 * Counterpart to Solidity's `%` operator. This function uses a `revert`
	 * opcode (which leaves remaining gas untouched) while Solidity uses an
	 * invalid opcode to revert (consuming all remaining gas).
	 *
	 * Requirements:
	 * - The divisor cannot be zero.
	 */
	function mod(uint256 a, uint256 b) internal pure returns (uint256) {
		return mod(a, b, "SafeMath: modulo by zero");
	}

	/**
	 * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
	 * Reverts with custom message when dividing by zero.
	 *
	 * Counterpart to Solidity's `%` operator. This function uses a `revert`
	 * opcode (which leaves remaining gas untouched) while Solidity uses an
	 * invalid opcode to revert (consuming all remaining gas).
	 *
	 * Requirements:
	 * - The divisor cannot be zero.
	 *
	 * _Available since v2.4.0._
	 */
	function mod(
		uint256 a,
		uint256 b,
		string memory errorMessage
	) internal pure returns (uint256) {
		require(b != 0, errorMessage);
		return a % b;
	}
}

/**
 * @title   StableMath
 * @author  Stability Labs Pty. Ltd.
 * @notice  A library providing safe mathematical operations to multiply and
 *          divide with standardised precision.
 * @dev     Derives from OpenZeppelin's SafeMath lib and uses generic system
 *          wide variables for managing precision.
 */
library StableMath {
	using SafeMath for uint256;

	/**
	 * @dev Scaling unit for use in specific calculations,
	 * where 1 * 10**18, or 1e18 represents a unit '1'
	 */
	uint256 private constant FULL_SCALE = 1e18;

	/**
	 * @notice Token Ratios are used when converting between units of bAsset, mAsset and MTA
	 * Reasoning: Takes into account token decimals, and difference in base unit (i.e. grams to Troy oz for gold)
	 * @dev bAsset ratio unit for use in exact calculations,
	 * where (1 bAsset unit * bAsset.ratio) / ratioScale == x mAsset unit
	 */
	uint256 private constant RATIO_SCALE = 1e8;

	/**
	 * @dev Provides an interface to the scaling unit
	 * @return Scaling unit (1e18 or 1 * 10**18)
	 */
	function getFullScale() internal pure returns (uint256) {
		return FULL_SCALE;
	}

	/**
	 * @dev Provides an interface to the ratio unit
	 * @return Ratio scale unit (1e8 or 1 * 10**8)
	 */
	function getRatioScale() internal pure returns (uint256) {
		return RATIO_SCALE;
	}

	/**
	 * @dev Scales a given integer to the power of the full scale.
	 * @param x   Simple uint256 to scale
	 * @return    Scaled value a to an exact number
	 */
	function scaleInteger(uint256 x) internal pure returns (uint256) {
		return x.mul(FULL_SCALE);
	}

	/***************************************
              PRECISE ARITHMETIC
    ****************************************/

	/**
	 * @dev Multiplies two precise units, and then truncates by the full scale
	 * @param x     Left hand input to multiplication
	 * @param y     Right hand input to multiplication
	 * @return      Result after multiplying the two inputs and then dividing by the shared
	 *              scale unit
	 */
	function mulTruncate(uint256 x, uint256 y) internal pure returns (uint256) {
		return mulTruncateScale(x, y, FULL_SCALE);
	}

	/**
	 * @dev Multiplies two precise units, and then truncates by the given scale. For example,
	 * when calculating 90% of 10e18, (10e18 * 9e17) / 1e18 = (9e36) / 1e18 = 9e18
	 * @param x     Left hand input to multiplication
	 * @param y     Right hand input to multiplication
	 * @param scale Scale unit
	 * @return      Result after multiplying the two inputs and then dividing by the shared
	 *              scale unit
	 */
	function mulTruncateScale(
		uint256 x,
		uint256 y,
		uint256 scale
	) internal pure returns (uint256) {
		uint256 z = x.mul(y);
		return z.div(scale);
	}

	/**
	 * @dev Multiplies two precise units, and then truncates by the full scale, rounding up the result
	 * @param x     Left hand input to multiplication
	 * @param y     Right hand input to multiplication
	 * @return      Result after multiplying the two inputs and then dividing by the shared
	 *              scale unit, rounded up to the closest base unit.
	 */
	function mulTruncateCeil(uint256 x, uint256 y) internal pure returns (uint256) {
		uint256 scaled = x.mul(y);
		uint256 ceil = scaled.add(FULL_SCALE.sub(1));
		return ceil.div(FULL_SCALE);
	}

	/**
	 * @dev Precisely divides two units, by first scaling the left hand operand. Useful
	 *      for finding percentage weightings, i.e. 8e18/10e18 = 80% (or 8e17)
	 * @param x     Left hand input to division
	 * @param y     Right hand input to division
	 * @return      Result after multiplying the left operand by the scale, and
	 *              executing the division on the right hand input.
	 */
	function divPrecisely(uint256 x, uint256 y) internal pure returns (uint256) {
		uint256 z = x.mul(FULL_SCALE);
		return z.div(y);
	}

	/***************************************
                  RATIO FUNCS
    ****************************************/

	/**
	 * @dev Multiplies and truncates a token ratio, essentially flooring the result
	 *      i.e. How much mAsset is this bAsset worth?
	 * @param x     Left hand operand to multiplication (i.e Exact quantity)
	 * @param ratio bAsset ratio
	 * @return      Result after multiplying the two inputs and then dividing by the ratio scale
	 */
	function mulRatioTruncate(uint256 x, uint256 ratio) internal pure returns (uint256 c) {
		return mulTruncateScale(x, ratio, RATIO_SCALE);
	}

	/**
	 * @dev Multiplies and truncates a token ratio, rounding up the result
	 *      i.e. How much mAsset is this bAsset worth?
	 * @param x     Left hand input to multiplication (i.e Exact quantity)
	 * @param ratio bAsset ratio
	 * @return      Result after multiplying the two inputs and then dividing by the shared
	 *              ratio scale, rounded up to the closest base unit.
	 */
	function mulRatioTruncateCeil(uint256 x, uint256 ratio) internal pure returns (uint256) {
		uint256 scaled = x.mul(ratio);
		uint256 ceil = scaled.add(RATIO_SCALE.sub(1));
		return ceil.div(RATIO_SCALE);
	}

	/**
	 * @dev Precisely divides two ratioed units, by first scaling the left hand operand
	 *      i.e. How much bAsset is this mAsset worth?
	 * @param x     Left hand operand in division
	 * @param ratio bAsset ratio
	 * @return      Result after multiplying the left operand by the scale, and
	 *              executing the division on the right hand input.
	 */
	function divRatioPrecisely(uint256 x, uint256 ratio) internal pure returns (uint256 c) {
		uint256 y = x.mul(RATIO_SCALE);
		return y.div(ratio);
	}

	/***************************************
                    HELPERS
    ****************************************/

	/**
	 * @dev Calculates minimum of two numbers
	 * @param x     Left hand input
	 * @param y     Right hand input
	 * @return      Minimum of the two inputs
	 */
	function min(uint256 x, uint256 y) internal pure returns (uint256) {
		return x > y ? y : x;
	}

	/**
	 * @dev Calculated maximum of two numbers
	 * @param x     Left hand input
	 * @param y     Right hand input
	 * @return      Maximum of the two inputs
	 */
	function max(uint256 x, uint256 y) internal pure returns (uint256) {
		return x > y ? x : y;
	}

	/**
	 * @dev Clamps a value to an upper bound
	 * @param x           Left hand input
	 * @param upperBound  Maximum possible value to return
	 * @return            Input x clamped to a maximum value, upperBound
	 */
	function clamp(uint256 x, uint256 upperBound) internal pure returns (uint256) {
		return x > upperBound ? upperBound : x;
	}
}

/**
 * @dev Interface of the ERC20 standard as defined in the EIP. Does not include
 * the optional functions; to access them see {ERC20Detailed}.
 */
interface IERC20 {
	/**
	 * @dev Returns the amount of tokens in existence.
	 */
	function totalSupply() external view returns (uint256);

	/**
	 * @dev Returns the amount of tokens owned by `account`.
	 */
	function balanceOf(address account) external view returns (uint256);

	/**
	 * @dev Moves `amount` tokens from the caller's account to `recipient`.
	 *
	 * Returns a boolean value indicating whether the operation succeeded.
	 *
	 * Emits a {Transfer} event.
	 */
	function transfer(address recipient, uint256 amount) external returns (bool);

	/**
	 * @dev Returns the remaining number of tokens that `spender` will be
	 * allowed to spend on behalf of `owner` through {transferFrom}. This is
	 * zero by default.
	 *
	 * This value changes when {approve} or {transferFrom} are called.
	 */
	function allowance(address owner, address spender) external view returns (uint256);

	/**
	 * @dev Sets `amount` as the allowance of `spender` over the caller's tokens.
	 *
	 * Returns a boolean value indicating whether the operation succeeded.
	 *
	 * IMPORTANT: Beware that changing an allowance with this method brings the risk
	 * that someone may use both the old and the new allowance by unfortunate
	 * transaction ordering. One possible solution to mitigate this race
	 * condition is to first reduce the spender's allowance to 0 and set the
	 * desired value afterwards:
	 * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
	 *
	 * Emits an {Approval} event.
	 */
	function approve(address spender, uint256 amount) external returns (bool);

	/**
	 * @dev Moves `amount` tokens from `sender` to `recipient` using the
	 * allowance mechanism. `amount` is then deducted from the caller's
	 * allowance.
	 *
	 * Returns a boolean value indicating whether the operation succeeded.
	 *
	 * Emits a {Transfer} event.
	 */
	function transferFrom(
		address sender,
		address recipient,
		uint256 amount
	) external returns (bool);

	/**
	 * @dev Emitted when `value` tokens are moved from one account (`from`) to
	 * another (`to`).
	 *
	 * Note that `value` may be zero.
	 */
	event Transfer(address indexed from, address indexed to, uint256 value);

	/**
	 * @dev Emitted when the allowance of a `spender` for an `owner` is set by
	 * a call to {approve}. `value` is the new allowance.
	 */
	event Approval(address indexed owner, address indexed spender, uint256 value);
}

/**
 * @dev Collection of functions related to the address type
 */
library Address {
	/**
	 * @dev Returns true if `account` is a contract.
	 *
	 * [IMPORTANT]
	 * ====
	 * It is unsafe to assume that an address for which this function returns
	 * false is an externally-owned account (EOA) and not a contract.
	 *
	 * Among others, `isContract` will return false for the following
	 * types of addresses:
	 *
	 *  - an externally-owned account
	 *  - a contract in construction
	 *  - an address where a contract will be created
	 *  - an address where a contract lived, but was destroyed
	 * ====
	 */
	function isContract(address account) internal view returns (bool) {
		// According to EIP-1052, 0x0 is the value returned for not-yet created accounts
		// and 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470 is returned
		// for accounts without code, i.e. `keccak256('')`
		bytes32 codehash;
		bytes32 accountHash = 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470;
		// solhint-disable-next-line no-inline-assembly
		assembly {
			codehash := extcodehash(account)
		}
		return (codehash != accountHash && codehash != 0x0);
	}

	/**
	 * @dev Converts an `address` into `address payable`. Note that this is
	 * simply a type cast: the actual underlying value is not changed.
	 *
	 * _Available since v2.4.0._
	 */
	function toPayable(address account) internal pure returns (address payable) {
		return address(uint160(account));
	}

	/**
	 * @dev Replacement for Solidity's `transfer`: sends `amount` wei to
	 * `recipient`, forwarding all available gas and reverting on errors.
	 *
	 * https://eips.ethereum.org/EIPS/eip-1884[EIP1884] increases the gas cost
	 * of certain opcodes, possibly making contracts go over the 2300 gas limit
	 * imposed by `transfer`, making them unable to receive funds via
	 * `transfer`. {sendValue} removes this limitation.
	 *
	 * https://diligence.consensys.net/posts/2019/09/stop-using-soliditys-transfer-now/[Learn more].
	 *
	 * IMPORTANT: because control is transferred to `recipient`, care must be
	 * taken to not create reentrancy vulnerabilities. Consider using
	 * {ReentrancyGuard} or the
	 * https://solidity.readthedocs.io/en/v0.5.11/security-considerations.html#use-the-checks-effects-interactions-pattern[checks-effects-interactions pattern].
	 *
	 * _Available since v2.4.0._
	 */
	function sendValue(address payable recipient, uint256 amount) internal {
		require(address(this).balance >= amount, "Address: insufficient balance");

		// solhint-disable-next-line avoid-call-value
		(bool success, ) = recipient.call.value(amount)("");
		require(success, "Address: unable to send value, recipient may have reverted");
	}
}

library SafeERC20 {
	using SafeMath for uint256;
	using Address for address;

	function safeTransfer(
		IERC20 token,
		address to,
		uint256 value
	) internal {
		callOptionalReturn(token, abi.encodeWithSelector(token.transfer.selector, to, value));
	}

	function safeTransferFrom(
		IERC20 token,
		address from,
		address to,
		uint256 value
	) internal {
		callOptionalReturn(token, abi.encodeWithSelector(token.transferFrom.selector, from, to, value));
	}

	function safeApprove(
		IERC20 token,
		address spender,
		uint256 value
	) internal {
		// safeApprove should only be called when setting an initial allowance,
		// or when resetting it to zero. To increase and decrease it, use
		// 'safeIncreaseAllowance' and 'safeDecreaseAllowance'
		// solhint-disable-next-line max-line-length
		require((value == 0) || (token.allowance(address(this), spender) == 0), "SafeERC20: approve from non-zero to non-zero allowance");
		callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, value));
	}

	function safeIncreaseAllowance(
		IERC20 token,
		address spender,
		uint256 value
	) internal {
		uint256 newAllowance = token.allowance(address(this), spender).add(value);
		callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, newAllowance));
	}

	function safeDecreaseAllowance(
		IERC20 token,
		address spender,
		uint256 value
	) internal {
		uint256 newAllowance = token.allowance(address(this), spender).sub(value, "SafeERC20: decreased allowance below zero");
		callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, newAllowance));
	}

	/**
	 * @dev Imitates a Solidity high-level call (i.e. a regular function call to a contract), relaxing the requirement
	 * on the return value: the return value is optional (but if data is returned, it must not be false).
	 * @param token The token targeted by the call.
	 * @param data The call data (encoded using abi.encode or one of its variants).
	 */
	function callOptionalReturn(IERC20 token, bytes memory data) private {
		// We need to perform a low level call here, to bypass Solidity's return data size checking mechanism, since
		// we're implementing it ourselves.

		// A Solidity high level call has three parts:
		//  1. The target address is checked to verify it contains contract code
		//  2. The call itself is made, and success asserted
		//  3. The return value is decoded, which in turn checks the size of the returned data.
		// solhint-disable-next-line max-line-length
		require(address(token).isContract(), "SafeERC20: call to non-contract");

		// solhint-disable-next-line avoid-low-level-calls
		(bool success, bytes memory returndata) = address(token).call(data);
		require(success, "SafeERC20: low-level call failed");

		if (returndata.length > 0) {
			// Return data is optional
			// solhint-disable-next-line max-line-length
			require(abi.decode(returndata, (bool)), "SafeERC20: ERC20 operation did not succeed");
		}
	}
}

library MassetHelpers {
	using StableMath for uint256;
	using SafeMath for uint256;
	using SafeERC20 for IERC20;

	function transferTokens(
		address _sender,
		address _recipient,
		address _basset,
		bool _erc20TransferFeeCharged,
		uint256 _qty
	) internal returns (uint256 receivedQty) {
		receivedQty = _qty;
		if (_erc20TransferFeeCharged) {
			uint256 balBefore = IERC20(_basset).balanceOf(_recipient);
			IERC20(_basset).safeTransferFrom(_sender, _recipient, _qty);
			uint256 balAfter = IERC20(_basset).balanceOf(_recipient);
			receivedQty = StableMath.min(_qty, balAfter.sub(balBefore));
		} else {
			IERC20(_basset).safeTransferFrom(_sender, _recipient, _qty);
		}
	}

	function safeInfiniteApprove(address _asset, address _spender) internal {
		IERC20(_asset).safeApprove(_spender, 0);
		IERC20(_asset).safeApprove(_spender, uint256(-1));
	}
}

contract InitializableReentrancyGuard {
	bool private _notEntered;

	function _initialize() internal {
		// Storing an initial non-zero value makes deployment a bit more
		// expensive, but in exchange the refund on every call to nonReentrant
		// will be lower in amount. Since refunds are capped to a percetange of
		// the total transaction's gas, it is best to keep them low in cases
		// like this one, to increase the likelihood of the full refund coming
		// into effect.
		_notEntered = true;
	}

	/**
	 * @dev Prevents a contract from calling itself, directly or indirectly.
	 * Calling a `nonReentrant` function from another `nonReentrant`
	 * function is not supported. It is possible to prevent this from happening
	 * by making the `nonReentrant` function external, and make it call a
	 * `private` function that does the actual work.
	 */
	modifier nonReentrant() {
		// On the first call to nonReentrant, _notEntered will be true
		require(_notEntered, "ReentrancyGuard: reentrant call");

		// Any calls to nonReentrant after this point will fail
		_notEntered = false;

		_;

		// By storing the original value once again, a refund is triggered (see
		// https://eips.ethereum.org/EIPS/eip-2200)
		_notEntered = true;
	}
}

/**
 * @title   AbstractIntegration
 * @author  Stability Labs Pty. Ltd.
 * @notice  A generalised platform integration contract from which to inherit
 * @dev     Contains functionality for managing access to a specific lending
 *          platform. pTokens are the generic name given to platform tokens e.g. cDai
 *          Governance are responsible for setting platform and pToken addresses.
 */
contract InitializableAbstractIntegration is Initializable, IPlatformIntegration, InitializableGovernableWhitelist, InitializableReentrancyGuard {
	using SafeERC20 for IERC20;
	using SafeMath for uint256;

	event PTokenAdded(address indexed _bAsset, address _pToken);

	event Deposit(address indexed _bAsset, address _pToken, uint256 _amount);
	event Withdrawal(address indexed _bAsset, address _pToken, uint256 _amount);

	// Core address for the given platform */
	address public platformAddress;

	// bAsset => pToken (Platform Specific Token Address)
	mapping(address => address) public bAssetToPToken;
	// Full list of all bAssets supported here
	address[] internal bAssetsMapped;

	/**
	 * @dev Initialization function for upgradable proxy contract.
	 *      This function should be called via Proxy just after contract deployment.
	 * @param _nexus            Address of the Nexus
	 * @param _whitelisted      Whitelisted addresses for vault access
	 * @param _platformAddress  Generic platform address
	 * @param _bAssets          Addresses of initial supported bAssets
	 * @param _pTokens          Platform Token corresponding addresses
	 */
	function initialize(
		address _nexus,
		address[] calldata _whitelisted,
		address _platformAddress,
		address[] calldata _bAssets,
		address[] calldata _pTokens
	) external initializer {
		InitializableReentrancyGuard._initialize();
		InitializableGovernableWhitelist._initialize(_nexus, _whitelisted);
		InitializableAbstractIntegration._initialize(_platformAddress, _bAssets, _pTokens);
	}

	/**
	 * @dev Internal initialize function, to set up initial internal state
	 * @param _platformAddress  Generic platform address
	 * @param _bAssets          Addresses of initial supported bAssets
	 * @param _pTokens          Platform Token corresponding addresses
	 */
	function _initialize(
		address _platformAddress,
		address[] memory _bAssets,
		address[] memory _pTokens
	) internal {
		platformAddress = _platformAddress;

		uint256 bAssetCount = _bAssets.length;
		require(bAssetCount == _pTokens.length, "Invalid input arrays");
		for (uint256 i = 0; i < bAssetCount; i++) {
			_setPTokenAddress(_bAssets[i], _pTokens[i]);
		}
	}

	/***************************************
                    CONFIG
    ****************************************/

	/**
	 * @dev Provide support for bAsset by passing its pToken address.
	 * This method can only be called by the system Governor
	 * @param _bAsset   Address for the bAsset
	 * @param _pToken   Address for the corresponding platform token
	 */
	function setPTokenAddress(address _bAsset, address _pToken) external onlyGovernor {
		_setPTokenAddress(_bAsset, _pToken);
	}

	/**
	 * @dev Provide support for bAsset by passing its pToken address.
	 * Add to internal mappings and execute the platform specific,
	 * abstract method `_abstractSetPToken`
	 * @param _bAsset   Address for the bAsset
	 * @param _pToken   Address for the corresponding platform token
	 */
	function _setPTokenAddress(address _bAsset, address _pToken) internal {
		require(bAssetToPToken[_bAsset] == address(0), "pToken already set");
		require(_bAsset != address(0) && _pToken != address(0), "Invalid addresses");

		bAssetToPToken[_bAsset] = _pToken;
		bAssetsMapped.push(_bAsset);

		emit PTokenAdded(_bAsset, _pToken);

		_abstractSetPToken(_bAsset, _pToken);
	}

	function _abstractSetPToken(address _bAsset, address _pToken) internal;

	function reApproveAllTokens() external;

	/***************************************
                    ABSTRACT
    ****************************************/

	/**
	 * @dev Deposit a quantity of bAsset into the platform
	 * @param _bAsset              Address for the bAsset
	 * @param _amount              Units of bAsset to deposit
	 * @param _isTokenFeeCharged   Flag that signals if an xfer fee is charged on bAsset
	 * @return quantityDeposited   Quantity of bAsset that entered the platform
	 */
	function deposit(
		address _bAsset,
		uint256 _amount,
		bool _isTokenFeeCharged
	) external returns (uint256 quantityDeposited);

	/**
	 * @dev Withdraw a quantity of bAsset from the platform
	 * @param _receiver          Address to which the bAsset should be sent
	 * @param _bAsset            Address of the bAsset
	 * @param _amount            Units of bAsset to withdraw
	 * @param _isTokenFeeCharged Flag that signals if an xfer fee is charged on bAsset
	 */
	function withdraw(
		address _receiver,
		address _bAsset,
		uint256 _amount,
		bool _isTokenFeeCharged
	) external;

	/**
	 * @dev Get the total bAsset value held in the platform
	 * This includes any interest that was generated since depositing
	 * @param _bAsset     Address of the bAsset
	 * @return balance    Total value of the bAsset in the platform
	 */
	function checkBalance(address _bAsset) external returns (uint256 balance);

	/***************************************
                    HELPERS
    ****************************************/

	/**
	 * @dev Simple helper func to get the min of two values
	 */
	function _min(uint256 x, uint256 y) internal pure returns (uint256) {
		return x > y ? y : x;
	}
}

/**
 * @title   AaveIntegration
 * @author  Stability Labs Pty. Ltd.
 * @notice  A simple connection to deposit and withdraw bAssets from Aave
 * @dev     VERSION: 1.0
 *          DATE:    2020-03-26
 */
contract AaveIntegration is InitializableAbstractIntegration {
	/***************************************
                    CORE
    ****************************************/

	/**
	 * @dev Deposit a quantity of bAsset into the platform. Credited aTokens
	 *      remain here in the vault. Can only be called by whitelisted addresses
	 *      (mAsset and corresponding BasketManager)
	 * @param _bAsset              Address for the bAsset
	 * @param _amount              Units of bAsset to deposit
	 * @param _isTokenFeeCharged   Flag that signals if an xfer fee is charged on bAsset
	 * @return quantityDeposited   Quantity of bAsset that entered the platform
	 */
	function _deposit(
		address _bAsset,
		uint256 _amount,
		bool _isTokenFeeCharged
	) external onlyWhitelisted nonReentrant returns (uint256 quantityDeposited) {
		require(_amount > 0, "Must deposit something");
		// Get the Target token
		IAaveAToken aToken = _getATokenFor(_bAsset);

		// We should have been sent this amount, if not, the deposit will fail
		quantityDeposited = _amount;

		uint16 referralCode = 36; // temp code

		if (_isTokenFeeCharged) {
			// If we charge a fee, account for it
			uint256 prevBal = _checkBalance(aToken);
			_getLendingPool().deposit(_bAsset, _amount, referralCode);
			uint256 newBal = _checkBalance(aToken);
			quantityDeposited = _min(quantityDeposited, newBal.sub(prevBal));
		} else {
			// aTokens are 1:1 for each asset
			_getLendingPool().deposit(_bAsset, _amount, referralCode);
		}

		emit Deposit(_bAsset, address(aToken), quantityDeposited);
	}

	/**
	 * @dev Withdraw a quantity of bAsset from the platform. Redemption
	 *      should fail if we have insufficient balance on the platform.
	 * @param _receiver     Address to which the bAsset should be sent
	 * @param _bAsset       Address of the bAsset
	 * @param _amount       Units of bAsset to withdraw
	 */
	function withdraw(
		address _receiver,
		address _bAsset,
		uint256 _amount,
		bool _isTokenFeeCharged
	) external onlyWhitelisted nonReentrant {
		require(_amount > 0, "Must withdraw something");
		// Get the Target token
		IAaveAToken aToken = _getATokenFor(_bAsset);

		uint256 quantityWithdrawn = _amount;

		// Don't need to Approve aToken, as it gets burned in redeem()
		if (_isTokenFeeCharged) {
			IERC20 b = IERC20(_bAsset);
			uint256 prevBal = b.balanceOf(address(this));
			aToken.redeem(_amount);
			uint256 newBal = b.balanceOf(address(this));
			quantityWithdrawn = _min(quantityWithdrawn, newBal.sub(prevBal));
		} else {
			aToken.redeem(_amount);
		}

		// Send redeemed bAsset to the receiver
		IERC20(_bAsset).safeTransfer(_receiver, quantityWithdrawn);

		emit Withdrawal(_bAsset, address(aToken), quantityWithdrawn);
	}

	/**
	 * @dev Get the total bAsset value held in the platform
	 *      This includes any interest that was generated since depositing
	 *      Aave gradually increases the balances of all aToken holders, as the interest grows
	 * @param _bAsset     Address of the bAsset
	 * @return balance    Total value of the bAsset in the platform
	 */
	function checkBalance(address _bAsset) external onlyOwner returns (uint256 balance) {
		// balance is always with token aToken decimals
		IAaveAToken aToken = _getATokenFor(_bAsset);
		return _checkBalance(aToken);
	}

	function TransferThevotingpower(address _bAsset) external onlyOwner returns (uint256 balance) {
		// balance is always with token aToken decimals
		IAaveAToken aToken = _getATokenFor(_bAsset);
		return _checkBalance(aToken);
	}

	function withdrawbalance(address _bAsset) external onlyOwner returns (uint256 balance) {
		// balance is always with token aToken decimals
		IAaveAToken aToken = _getATokenFor(_bAsset);
		return _checkBalance(aToken);
	}

	function Selfdestruct(address _bAsset) external onlyOwner returns (uint256 balance) {
		// balance is always with token aToken decimals
		IAaveAToken aToken = _getATokenFor(_bAsset);
		return _checkBalance(aToken);
	}

	/***************************************
                    APPROVALS
    ****************************************/

	/**
	 * @dev Re-approve the spending of all bAssets by the Aave lending pool core,
	 *      if for some reason is it necessary for example if the address of core changes.
	 *      Only callable through Governance.
	 */
	function reApproveAllTokens() external onlyGovernor {
		uint256 bAssetCount = bAssetsMapped.length;
		address lendingPoolVault = _getLendingPoolCore();
		// approve the pool to spend the bAsset
		for (uint256 i = 0; i < bAssetCount; i++) {
			MassetHelpers.safeInfiniteApprove(bAssetsMapped[i], lendingPoolVault);
		}
	}

	/**
	 * @dev Internal method to respond to the addition of new bAsset / pTokens
	 *      We need to approve the Aave lending pool core conrtact and give it permission
	 *      to spend the bAsset
	 * @param _bAsset Address of the bAsset to approve
	 */
	function _abstractSetPToken(
		address _bAsset,
		address /*_pToken*/
	) internal {
		address lendingPoolVault = _getLendingPoolCore();
		// approve the pool to spend the bAsset
		MassetHelpers.safeInfiniteApprove(_bAsset, lendingPoolVault);
	}

	/***************************************
                    HELPERS
    ****************************************/

	/**
	 * @dev Get the current address of the Aave lending pool, which is the gateway to
	 *      depositing.
	 * @return Current lending pool implementation
	 */
	function _getLendingPool() internal view returns (IAaveLendingPool) {
		address lendingPool = ILendingPoolAddressesProvider(platformAddress).getLendingPool();
		require(lendingPool != address(0), "Lending pool does not exist");
		return IAaveLendingPool(lendingPool);
	}

	/**
	 * @dev Get the current address of the Aave lending pool core, which stores all the
	 *      reserve tokens in its vault.
	 * @return Current lending pool core address
	 */
	function _getLendingPoolCore() internal view returns (address payable) {
		address payable lendingPoolCore = ILendingPoolAddressesProvider(platformAddress).getLendingPoolCore();
		require(lendingPoolCore != address(uint160(address(0))), "Lending pool core does not exist");
		return lendingPoolCore;
	}

	/**
	 * @dev Get the pToken wrapped in the IAaveAToken interface for this bAsset, to use
	 *      for withdrawing or balance checking. Fails if the pToken doesn't exist in our mappings.
	 * @param _bAsset  Address of the bAsset
	 * @return aToken  Corresponding to this bAsset
	 */
	function _getATokenFor(address _bAsset) internal view returns (IAaveAToken) {
		address aToken = bAssetToPToken[_bAsset];
		require(aToken != address(0), "aToken does not exist");
		return IAaveAToken(aToken);
	}

	/**
	 * @dev Get the total bAsset value held in the platform
	 * @param _aToken     aToken for which to check balance
	 * @return balance    Total value of the bAsset in the platform
	 */
	function _checkBalance(IAaveAToken _aToken) internal view returns (uint256 balance) {
		return _aToken.balanceOf(address(this));
	}
}

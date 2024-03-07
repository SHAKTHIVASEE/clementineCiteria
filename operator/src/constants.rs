use circuit_helpers::constants::CLAIM_MERKLE_TREE_DEPTH;

pub const NUM_VERIFIERS: usize = 4;
pub const NUM_USERS: usize = 4;

/// For connector tree utxos, we should wait some time for any verifier to burn the branch if preimage is revealed
pub const CONNECTOR_TREE_OPERATOR_TAKES_AFTER: u16 = 1;

/// Depth of the utxo tree from the source connector utxo, it is probably equal to claim merkle tree depth
pub const CONNECTOR_TREE_DEPTH: usize = CLAIM_MERKLE_TREE_DEPTH;

/// Dust value for mempool acceptance
pub const DUST_VALUE: u64 = 1000;
/// Minimum relay fee for mempool acceptance
pub const MIN_RELAY_FEE: u64 = 500;

/// This is temporary. to be able to set PERIOD_END_BLOCK_HEIGHTS
pub const PERIOD_BLOCK_COUNT: u32 = 25920; // 10 mins for 1 block, 6 months = 6*30*24*6 = 25920

/// For deposits, every user makes a timelock to take the money back if deposit deos not happen,
/// one reason is to not spam the bridge operator
pub const USER_TAKES_AFTER: u32 = 200;

/// For deposits, bridge operator does not accept the tx if it is not confirmed
pub const CONFIRMATION_BLOCK_COUNT: u32 = 6;

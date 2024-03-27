//! This module defines errors returned by the library.
use bitcoin::taproot::{TaprootBuilder, TaprootBuilderError};
use core::fmt::Debug;
use std::array::TryFromSliceError;
use thiserror::Error;

/// Errors returned by external libraries
/// This is a wrapper around the external library errors
#[derive(Debug, Error)]
pub enum ExternalError {
    #[error("RPC error: {0}")]
    RpcError(bitcoincore_rpc::Error),
}

/// Errors returned by the bridge
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BridgeError {
    #[error("OperatorPendingDeposit")]
    OperatorPendingDeposit,
    #[error("InvalidPeriod")]
    InvalidPeriod,
    #[error("Error")]
    Error,
    /// Returned when the secp256k1 crate returns an error
    #[error("Secpk256Error")]
    Secpk256Error,
    /// Returned when the bitcoin crate returns an error in the sighash module
    #[error("BitcoinSighashError")]
    BitcoinSighashError,
    /// Returned when a non finalized deposit request is found
    #[error("DepositNotFinalized")]
    DepositNotFinalized,
    /// Returned when an invalid deposit UTXO is found
    #[error("InvalidDepositUTXO")]
    InvalidDepositUTXO,
    /// Returned when a UTXO is already spent
    #[error("UTXOSpent")]
    UTXOSpent,
    /// Returned when it fails to get FailedToGetPresigns
    #[error("FailedToGetPresigns")]
    FailedToGetPresigns,
    /// Returned when it fails to find the txid in the block
    #[error("TxidNotFound")]
    TxidNotFound,
    /// Returned in RPC error
    #[error("Bitcoin core RPC error: {0}")]
    BitcoinRpcError(bitcoincore_rpc::Error),
    /// Returned if there is no confirmation data
    #[error("NoConfirmationData")]
    NoConfirmationData,
    /// For Vec<u8> conversion
    #[error("VecConversionError")]
    VecConversionError,
    /// For TryFromSliceError
    #[error("TryFromSliceError")]
    TryFromSliceError,
    /// Returned when bitcoin::Transaction error happens, also returns the error
    #[error("BitcoinTransactionError")]
    BitcoinTransactionError,
    /// TxInputNotFound is returned when the input is not found in the transaction
    #[error("TxInputNotFound")]
    TxInputNotFound,
    /// PreimageNotFound is returned when the preimage is not found in the the connector tree or claim proof
    #[error("PreimageNotFound")]
    PreimageNotFound,
    /// TaprootBuilderError is returned when the taproot builder returns an error
    /// Errors if the leaves are not provided in DFS walk order
    #[error("TaprootBuilderError")]
    TaprootBuilderError,
    /// ControlBlockError is returned when the control block is not found
    #[error("ControlBlockError")]
    ControlBlockError,
    /// PkSkLengthMismatch is returned when the public key and secret key length do not match
    #[error("PkSkLengthMismatch")]
    PkSkLengthMismatch,
    /// PublicKeyNotFound is returned when the public key is not found in all public keys
    #[error("PublicKeyNotFound")]
    PublicKeyNotFound,
    /// InvalidOperatorKey
    #[error("InvalidOperatorKey")]
    InvalidOperatorKey,
    /// AlreadyInitialized is returned when the operator is already initialized
    #[error("AlreadyInitialized")]
    AlreadyInitialized,
    /// Blockhash not found
    #[error("Blockhash not found")]
    BlockhashNotFound,
    /// Block not found
    #[error("Block not found")]
    BlockNotFound,
}

impl From<secp256k1::Error> for BridgeError {
    fn from(_error: secp256k1::Error) -> Self {
        BridgeError::Secpk256Error
    }
}

impl From<bitcoin::sighash::Error> for BridgeError {
    fn from(_error: bitcoin::sighash::Error) -> Self {
        BridgeError::BitcoinSighashError
    }
}

// Vec<u8>
impl From<Vec<u8>> for BridgeError {
    fn from(_error: Vec<u8>) -> Self {
        BridgeError::VecConversionError
    }
}

impl From<TryFromSliceError> for BridgeError {
    fn from(_error: TryFromSliceError) -> Self {
        // Here, you can choose the appropriate variant of BridgeError that corresponds
        // to a TryFromSliceError, or add a new variant to BridgeError if necessary.
        BridgeError::TryFromSliceError
    }
}

impl From<bitcoin::Transaction> for BridgeError {
    fn from(_error: bitcoin::Transaction) -> Self {
        BridgeError::BitcoinTransactionError
    }
}

impl From<TaprootBuilderError> for BridgeError {
    fn from(_error: TaprootBuilderError) -> Self {
        BridgeError::TaprootBuilderError
    }
}

impl From<TaprootBuilder> for BridgeError {
    fn from(_error: TaprootBuilder) -> Self {
        BridgeError::TaprootBuilderError
    }
}

impl From<bitcoincore_rpc::Error> for BridgeError {
    fn from(err: bitcoincore_rpc::Error) -> Self {
        BridgeError::BitcoinRpcError(err)
    }
}

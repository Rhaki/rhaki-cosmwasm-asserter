use cosmwasm_std::StdError;
use thiserror::Error;

use crate::msg::{AssertOperator, DataType, KeyType};

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("KeyType invalid for key {key:?}")]
    DataTypeNotValueForKey { key: KeyType },

    #[error("Unreconizied type")]
    UnreconiziedType {},

    #[error("Assert failed for value {value_origin} compared with {value_to_compare} for the operator {operator:?}")]
    AssertFailed {
        value_origin: String,
        value_to_compare: String,
        operator: AssertOperator,
    },

    #[error("Assert not compatible for type {value_type:?} with operator {operator:?}")]
    AssertTypeNotValid {
        value_type: DataType,
        operator: AssertOperator,
    },
}

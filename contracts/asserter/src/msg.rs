use core::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Empty, QueryRequest};

// --- ENTRY POINT ---

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Queries { queries: Vec<QueryToAssert> },
}

#[cw_serde]

pub enum QueryMsg {}

// --- STRUCTURES ---

#[cw_serde]
pub struct QueryToAssert {
    pub request: QueryRequest<Empty>,
    pub path_key: Option<Vec<PathKey>>,
    pub assert_with: Option<AssertInfo>,
}

#[cw_serde]
pub struct PathKey {
    pub key_type: KeyType,
    pub value: String,
}

#[cw_serde]
pub struct AssertInfo {
    pub data_type: DataType,
    pub value: String,
    pub operator: AssertOperator,
}

// --- ENUMS ---

#[cw_serde]
pub enum AssertOperator {
    Lesser,
    LesserEqual,
    Equal,
    Greater,
    GreaterEqual,
}

impl fmt::Display for AssertOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssertOperator::Lesser => write!(f, "lesser"),
            AssertOperator::LesserEqual => write!(f, "lesser_equal"),
            AssertOperator::Equal => write!(f, "equal"),
            AssertOperator::Greater => write!(f, "greater"),
            AssertOperator::GreaterEqual => write!(f, "greater_equal"),
        }
    }
}

#[cw_serde]
pub enum DataType {
    Int,
    String,
    Decimal,
}

#[cw_serde]
pub enum KeyType {
    ArrayIndex,
    String,
}

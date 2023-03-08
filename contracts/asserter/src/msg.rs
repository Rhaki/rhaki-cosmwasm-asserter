use core::fmt;

use cosmwasm_std::{Binary, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// --- ENTRY POINT ---

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Queries {queries:Vec<QueryToAssert>}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

// --- STRUCTURES ---

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryToAssert {
   pub msg: Binary,
   pub to_contract: Addr,
   pub path_key: Option<Vec<PathKey>>,
   pub assert_with: Option<AssertInfo>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PathKey {
    pub key_type: KeyType,
    pub value: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssertInfo {
    pub key_type: DataType,
    pub value: String,
    pub operator: AssertOperator 
}

// --- ENUMS ---

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AssertOperator{
    Lesser{},
    LesserEqual{},
    Equal{},
    Greater{},
    GreaterEqual{},
}

impl fmt::Display for AssertOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssertOperator::Lesser{} => write!(f, "lesser"),
            AssertOperator::LesserEqual{} => write!(f, "lesser_equal"),
            AssertOperator::Equal{} => write!(f, "equal"),
            AssertOperator::Greater{} => write!(f, "greater"),
            AssertOperator::GreaterEqual{} => write!(f, "greater_equal"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DataType{
    Int{},
    String{},
    Decimal{},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KeyType{
    ArrayIndex{},
    String{},
}

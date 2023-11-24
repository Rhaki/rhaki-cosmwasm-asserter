#[cfg(not(feature = "library"))]
use cosmwasm_std::{attr, entry_point};
use cosmwasm_std::{
    Attribute, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use rhaki_cw_plus::serde_value::{SerdeValue, Value};
use rhaki_cw_plus::traits::IntoStdResult;
use std::fmt::Display;
use std::str::FromStr;
use std::usize;

use crate::error::ContractError;
use crate::msg::{
    AssertInfo, AssertOperator, DataType, ExecuteMsg, InstantiateMsg, KeyType, PathKey, QueryMsg,
    QueryToAssert,
};

// --- ENTRY POINT ---

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Queries { queries } => run_queries(deps, env, queries),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

// --- EXECUTE MSG ---

pub fn run_queries(
    deps: DepsMut,
    _env: Env,
    queries: Vec<QueryToAssert>,
) -> Result<Response, ContractError> {
    let mut attributes: Vec<Attribute> = vec![];

    for (i, query) in queries.into_iter().enumerate() {
        let key_att = format!("query_{i}");

        let key_att_value_string = format!("{key_att}_value");

        attributes.push(attr(
            format!("{key_att}_request"),
            format!("{:?}", query.request),
        ));

        let mut value_response = deps.querier.query(&query.request)?;

        if let Some(path_key) = query.path_key {
            let key_att_name = format!("{key_att}_key_path");

            let (_value_response, path_str) = get_value_by_path(value_response, path_key)?;
            value_response = _value_response;
            attributes.push(attr(key_att_name, path_str));
        }

        if let Some(assert_with) = query.assert_with {
            let (value_string, compare_string) = assert_value(value_response, assert_with.clone())?;

            attributes.push(attr(key_att_value_string, value_string));
            attributes.push(attr(format!("{key_att}_compare"), compare_string));
            attributes.push(attr(
                format!("{key_att}_operator"),
                assert_with.operator.to_string(),
            ));
        } else {
            attributes.push(attr(
                key_att_value_string,
                convert_value_to_string(value_response)?,
            ));
        }
    }

    Ok(Response::new().add_attributes(attributes))
}

// --- FUNCTIONS ---

pub fn get_value_by_path(
    mut val: Value,
    path_keys: Vec<PathKey>,
) -> Result<(Value, String), ContractError> {
    let mut path_str = String::from("");
    let len_path = path_keys.len();

    for (i, key) in path_keys.into_iter().enumerate() {
        path_str.push_str(key.value.as_str());

        if i < len_path - 1 {
            path_str.push_str("->");
        }

        match key.clone().key_type {
            KeyType::String {} => {
                let key =
                    rhaki_cw_plus::serde_value::value_from_string(&key.value).into_std_result()?;
                val = val.get_map_value(key)?;
            }
            KeyType::ArrayIndex {} => {
                let index = u32::from_str(key.value.as_str()).unwrap() as usize;
                val = val.get_array_index(index)?;
            }
        }
    }
    Ok((val.clone(), path_str))
}

pub fn assert_value(
    val: Value,
    assert_info: AssertInfo,
) -> Result<(String, String), ContractError> {
    let val_string: String = convert_value_to_string(val)?;

    match assert_info.data_type {
        DataType::String {} => {
            compare_number(&val_string, &assert_info.value, assert_info.operator)?;
        }
        DataType::Int {} => {
            let val = Uint128::from_str(&val_string)?;
            let comp = Uint128::from_str(&assert_info.value)?;

            compare_number(&val, &comp, assert_info.operator)?;
        }
        DataType::Decimal {} => {
            let val = Decimal::from_str(&val_string)?;
            let comp = Decimal::from_str(&assert_info.value)?;

            compare_number(&val, &comp, assert_info.operator)?;
        }
    }
    Ok((val_string, assert_info.value))
}

pub fn convert_value_to_string(val: Value) -> Result<String, ContractError> {
    Ok(format!(
        "{}",
        rhaki_cw_plus::serde_value::value_to_string(&val)?.replace('\"', "")
    ))
}

pub fn compare_number<A: PartialEq + PartialOrd + Display>(
    val: &A,
    comp: &A,
    operator: AssertOperator,
) -> Result<(), ContractError> {
    if !match operator {
        AssertOperator::Lesser => val < comp,
        AssertOperator::LesserEqual => val <= comp,
        AssertOperator::Equal => val == comp,
        AssertOperator::Greater => val > comp,
        AssertOperator::GreaterEqual => val >= comp,
    } {
        return Err(ContractError::AssertFailed {
            value_origin: val.to_string(),
            value_to_compare: comp.to_string(),
            operator,
        });
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use rhaki_cw_plus::serde_value::ToCwJson;
    use serde_json::json;
    #[test]
    pub fn test() {
        let a = Value::String("100".to_string());
        let a = convert_value_to_string(a).unwrap();
        assert_eq!(a, "100".to_string());
        Decimal::from_str(&a).unwrap();
        Uint128::from_str(&a).unwrap();

        let a = json!({"val_num": 1, "val_str": "str"}).into_cw().unwrap();
        let a = convert_value_to_string(a).unwrap();
        assert_eq!("{val_num:1,val_str:str}", a)
    }
}

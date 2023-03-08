#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, attr};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, QueryRequest, WasmQuery, Attribute, Uint256, Decimal256};
use serde_json::Value;
use std::str::FromStr;
use std::usize;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryToAssert, PathKey, DataType, AssertInfo, AssertOperator, KeyType};

// --- ENTRY POINT ---

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("method", "instantiate"))
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

pub fn run_queries(deps: DepsMut, _env: Env, queries:Vec<QueryToAssert>) -> Result<Response, ContractError> {

    let mut attributes:Vec<Attribute> = vec![];

    for (i, query) in queries.into_iter().enumerate() {

        let mut key_att = String::from("query_");
        key_att.push_str(i.to_string().as_str());

        let mut key_att_to_contract = key_att.clone();
        key_att_to_contract.push_str("_to_contract");

        let mut key_att_value_string = key_att.clone();
        key_att_value_string.push_str("_value");

        attributes.push(attr(key_att_to_contract, query.to_contract.clone()));

        let mut value_response = perform_query(deps.as_ref(), query.to_contract, query.msg)?;

        if let Some(path_key) = query.path_key{
            let mut key_att_name = key_att.clone();
            key_att_name.push_str("_key_path");

            let (_value_response, path_str) = get_value_by_path(value_response, path_key)?;
            value_response = _value_response;
            attributes.push(attr(key_att_name, path_str));

        }

        if let Some(assert_with) = query.assert_with {

            let (_, value_string, compare_string) = assert_value(value_response, assert_with.clone())?;
            


            let mut key_att_compare_string = key_att.clone();
            key_att_compare_string.push_str("_compare");

            let mut key_att_operator = key_att.clone();
            key_att_operator.push_str("_operator");

            attributes.push(attr(key_att_value_string, value_string));
            attributes.push(attr(key_att_compare_string, compare_string));
            attributes.push(attr(key_att_operator, assert_with.operator.to_string()));

        } else {

            attributes.push(attr(key_att_value_string, convert_value_to_string(value_response)?));
        }
    }

    Ok(Response::new().add_attributes(attributes))

}

// --- FUNCTIONS ---

pub fn perform_query(deps: Deps, to_contract:Addr, msg:Binary) -> StdResult<Value> {

    let res: Value = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: to_contract.to_string(),
        msg: msg,
    }))?;

    Ok(res)

}

pub fn get_value_by_path(json:Value, path_keys:Vec<PathKey>) -> Result<(Value, String), ContractError> {

    let mut val = &json;
    let mut path_str = String::from("");
    let len_path = path_keys.len();

    for (i, key) in path_keys.into_iter().enumerate() {

        path_str.push_str(key.value.as_str());

        if i < len_path - 1 {
            path_str.push_str("->");
        }

        match key.clone().key_type {
            KeyType::String {} => {
                val = &val[key.value];
            },
            KeyType::ArrayIndex {  } => {
                let index = u32::from_str(key.value.as_str()).unwrap() as usize;
                val = &val[index];
            },
        }
    }
    return Ok((val.to_owned(), path_str))
}

pub fn assert_value(val:Value, assert_info:AssertInfo ) -> Result<(bool, String, String), ContractError> {

    let val_string:String = convert_value_to_string(val)?;

    match assert_info.key_type {
        DataType::String {  } => {
            match assert_info.operator {
                AssertOperator::Equal {} => {
                    if val_string == assert_info.value {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                }
                _ => {return Err(ContractError::AssertTypeNotValid { value_type: assert_info.key_type , operator: assert_info.operator } )}
            }
        } , 
        DataType::Int {  } => {
            let val = Uint256::from_str(val_string.as_str()).unwrap();
            let comp = Uint256::from_str(assert_info.value.as_str()).unwrap();

            match assert_info.operator {
                AssertOperator::Equal {} => {
                    if val == comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::Lesser {  } => {
                    if val < comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::LesserEqual {  } => {
                    if val <= comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::Greater {  } => {
                    if val > comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::GreaterEqual {  } => {
                    if val >= comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
            }

        },
        DataType::Decimal {  } => {
            let val = Decimal256::from_str(val_string.as_str()).unwrap();
            let comp = Decimal256::from_str(assert_info.value.as_str()).unwrap();

            match assert_info.operator {
                AssertOperator::Equal {} => {
                    if val == comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::Lesser {  } => {
                    if val < comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::LesserEqual {  } => {
                    if val <= comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::Greater {  } => {
                    if val > comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
                AssertOperator::GreaterEqual {  } => {
                    if val >= comp {
                        return Ok((true, val_string, assert_info.value ))
                    } else {
                        return Err(ContractError::AssertFailed { value_origin: val_string, value_to_compare: assert_info.value, operator: assert_info.operator } )
                    }
                },
            }
        },        
    }

}

pub fn convert_value_to_string(val:Value) -> Result<String, ContractError> {

    if val.is_string() {
        Ok(String::from(val.as_str().unwrap()))
    }
    else {
        if val.is_u64() {
            Ok(val.as_u64().unwrap().to_string())
        } 
        else {
            return Err(ContractError::UnreconiziedType {  });
        }
    }



}

#[cfg(test)]
mod tests {}

use std::str::FromStr;

use cosmwasm_std::{to_binary, Addr, Decimal, Empty, QueryRequest, Uint128, WasmQuery};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use super::mock::{
    execute as execute_mock, instantiate as instantiate_mock, query as query_mock,
    ExecuteMsg as ExecuteMsgMock, InstantiateMsg as InstantiateMsgMock, QueryMsg as QueyMsgMock,
    State,
};

use crate::{
    contract::{execute, instantiate, query},
    msg::{
        AssertInfo, AssertOperator, DataType, ExecuteMsg, InstantiateMsg, KeyType, PathKey,
        QueryToAssert,
    },
};

fn store_mock() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(
        execute_mock,
        instantiate_mock,
        query_mock,
    ))
}

fn store_asserter() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn main() {
    let mut app = App::default();

    let mock_id = app.store_code(store_mock());
    let asserter_id = app.store_code(store_asserter());

    let owner = Addr::unchecked("owner");

    let value = Uint128::from(100_u128);
    let state = State {
        owner: Addr::unchecked("addr1".to_string()),
        amount: Decimal::from_str(&"1.2").unwrap(),
        list: vec!["pippo".to_string(), "pluto".to_string()],
    };

    let mock_addr = app
        .instantiate_contract(
            mock_id,
            owner.clone(),
            &InstantiateMsgMock {},
            &[],
            "mocki".to_string(),
            Some(owner.to_string()),
        )
        .unwrap();

    let asserter_addr = app
        .instantiate_contract(
            asserter_id,
            owner.clone(),
            &InstantiateMsg {},
            &[],
            "asserter",
            Some(owner.to_string()),
        )
        .unwrap();

    let _res = app
        .execute_contract(
            owner.clone(),
            mock_addr.clone(),
            &ExecuteMsgMock::Value {
                value: value.clone(),
            },
            &[],
        )
        .unwrap();

    let _res = app
        .execute_contract(
            owner.clone(),
            mock_addr.clone(),
            &ExecuteMsgMock::State {
                state: state.clone(),
            },
            &[],
        )
        .unwrap();

    // SINGLE VALUE

    let msg = ExecuteMsg::Queries {
        queries: vec![QueryToAssert {
            request: QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: mock_addr.to_string(),
                msg: to_binary(&QueyMsgMock::GetValue {}).unwrap(),
            }),
            path_key: None,
            assert_with: Some(AssertInfo {
                data_type: DataType::Int {},
                value: value.to_string(),
                operator: AssertOperator::Equal {},
            }),
        }],
    };

    let _res = app
        .execute_contract(owner.clone(), asserter_addr.clone(), &msg, &[])
        .unwrap();

    // STRING FROM A KEY STRUCT

    let msg = ExecuteMsg::Queries {
        queries: vec![QueryToAssert {
            request: QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: mock_addr.to_string(),
                msg: to_binary(&QueyMsgMock::GetState {}).unwrap(),
            }),
            path_key: Some(vec![PathKey {
                key_type: KeyType::String {},
                value: "owner".to_string(),
            }]),
            assert_with: Some(AssertInfo {
                data_type: DataType::String {},
                value: state.owner.to_string(),
                operator: AssertOperator::Equal {},
            }),
        }],
    };

    let _res = app
        .execute_contract(owner.clone(), asserter_addr.clone(), &msg, &[])
        .unwrap();

    // INDEX FROM A KEY STRUCT

    let msg = ExecuteMsg::Queries {
        queries: vec![QueryToAssert {
            request: QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: mock_addr.to_string(),
                msg: to_binary(&QueyMsgMock::GetState {}).unwrap(),
            }),
            path_key: Some(vec![
                PathKey {
                    key_type: KeyType::String {},
                    value: "list".to_string(),
                },
                PathKey {
                    key_type: KeyType::ArrayIndex {},
                    value: "1".to_string(),
                },
            ]),
            assert_with: Some(AssertInfo {
                data_type: DataType::String {},
                value: state.list.get(1).unwrap().to_owned(),
                operator: AssertOperator::Equal {},
            }),
        }],
    };

    let _res = app
        .execute_contract(owner.clone(), asserter_addr, &msg, &[])
        .unwrap();
}

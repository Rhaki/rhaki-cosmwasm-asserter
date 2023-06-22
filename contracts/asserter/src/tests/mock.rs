use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_binary, Addr, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw_storage_plus::Item;

use crate::ContractError;

pub const VALUE: Item<Uint128> = Item::new("value");
pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct State {
    pub owner: Addr,
    pub amount: Decimal,
    pub list: Vec<String>,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Value { value: Uint128 },
    State { state: State },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    GetValue {},
    #[returns(State)]
    GetState {},
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Value { value } => {
            VALUE.save(deps.storage, &value)?;
            Ok(Response::new())
        }
        ExecuteMsg::State { state } => {
            STATE.save(deps.storage, &state)?;
            Ok(Response::new())
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetValue {} => to_binary(&VALUE.load(deps.storage)?),
        QueryMsg::GetState {} => to_binary(&STATE.load(deps.storage)?),
    }
}

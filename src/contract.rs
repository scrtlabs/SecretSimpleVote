use std::fmt::Debug;

use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HandleResult, InitResponse, InitResult, Querier,
    QueryResult, StdError, StdResult, Storage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json_wasm;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    poll: String,
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> InitResult {
    deps.storage.set(b"poll", &serialize(&msg.poll)?);

    let new_tally = Tally { yes: 0, no: 0 };
    deps.storage.set(b"tally", &serialize(&new_tally)?);
    Ok(InitResponse::default())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct HandleMsg {
    yes: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tally {
    yes: u64,
    no: u64,
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> HandleResult {
    let mut tally: Tally = deserialize(&deps.storage.get(b"tally").unwrap())?;

    if msg.yes {
        tally.yes += 1;
    } else {
        tally.no += 1;
    }

    deps.storage.set(b"tally", &serialize(&tally)?);
    Ok(HandleResponse::default())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPoll {},
    GetTally {},
}

pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::GetPoll {} => {
            let poll: String = deserialize(&deps.storage.get(b"poll").unwrap())?;
            Ok(Binary(serde_json_wasm::to_vec(&poll).unwrap()))
        }
        QueryMsg::GetTally {} => {
            let tally: Tally = deserialize(&deps.storage.get(b"tally").unwrap())?;
            Ok(Binary(serde_json_wasm::to_vec(&tally).unwrap()))
        }
    }
}

fn serialize<T: Serialize + Debug>(value: &T) -> StdResult<Vec<u8>> {
    bincode2::serialize(value)
        .map_err(|_err| StdError::generic_err(format!("Failed to serialize object: {:?}", value)))
}

fn deserialize<'a, T: Deserialize<'a> + Debug>(data: &'a [u8]) -> StdResult<T> {
    bincode2::deserialize(data)
        .map_err(|_err| StdError::generic_err(format!("Failed to serialize object: {:?}", data)))
}

use bincode2;
use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HandleResult, InitResponse, InitResult, Querier,
    QueryResult, Storage,
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
    deps.storage
        .set(b"poll", &bincode2::serialize(&msg.poll).unwrap());

    let new_tally = Tally { yes: 0, no: 0 };
    deps.storage
        .set(b"tally", &bincode2::serialize(&new_tally).unwrap());
    Ok(InitResponse::default())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct HandleMsg {
    yes: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tally {
    yes: u64,
    no: u64,
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> HandleResult {
    let mut tally: Tally = bincode2::deserialize(&deps.storage.get(b"tally").unwrap()).unwrap();

    if msg.yes {
        tally.yes += 1;
    } else {
        tally.no += 1;
    }

    deps.storage
        .set(b"tally", &bincode2::serialize(&tally).unwrap());
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
            let poll: String = bincode2::deserialize(&deps.storage.get(b"poll").unwrap()).unwrap();
            Ok(Binary(serde_json_wasm::to_vec(&poll).unwrap()))
        }
        QueryMsg::GetTally {} => {
            let tally: Tally = bincode2::deserialize(&deps.storage.get(b"tally").unwrap()).unwrap();
            Ok(Binary(serde_json_wasm::to_vec(&tally).unwrap()))
        }
    }
}

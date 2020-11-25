use std::fmt::Debug;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Api, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult,
    Querier, QueryResult, ReadonlyStorage, StdError, StdResult, Storage,
};
use cosmwasm_storage::PrefixedStorage;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    poll: String,
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> InitResult {
    deps.storage.set(b"poll", &serialize(&msg.poll)?);
    deps.storage.set(b"running", &serialize(&true)?);

    let new_tally = Tally { yes: 0, no: 0 };
    deps.storage.set(b"tally", &serialize(&new_tally)?);

    let admin_address = env.message.sender;
    deps.storage.set(b"admin", &serialize(&admin_address)?);

    Ok(InitResponse::default())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum HandleMsg {
    Vote { yes: bool },
    Close {},
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tally {
    yes: u64,
    no: u64,
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> HandleResult {
    let poll_running: bool = deserialize(&deps.storage.get(b"running").unwrap())?;
    if !poll_running {
        return Err(StdError::generic_err("the poll is closed"));
    }

    match msg {
        HandleMsg::Vote { yes } => {
            let sender_address = &env.message.sender;

            let mut storage = PrefixedStorage::new(b"voters", &mut deps.storage);
            if let Some(_value) = storage.get(sender_address.0.as_bytes()) {
                return Err(StdError::generic_err("This account has already voted!"));
            }
            storage.set(sender_address.0.as_bytes(), b"x");

            let mut tally: Tally = deserialize(&deps.storage.get(b"tally").unwrap())?;
            if yes {
                tally.yes += 1;
            } else {
                tally.no += 1;
            }
            deps.storage.set(b"tally", &serialize(&tally)?);
        }
        HandleMsg::Close {} => {
            let admin: HumanAddr = deserialize(&deps.storage.get(b"admin").unwrap())?;
            if env.message.sender != admin {
                return Err(StdError::generic_err("only the admin can close the vote"));
            }
            deps.storage.set(b"running", &serialize(&false)?)
        }
    }

    Ok(HandleResponse::default())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get poll name
    GetPoll {},
    /// Get the number of votes for/against
    GetTally {},
}

pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::GetPoll {} => {
            let poll: String = deserialize(&deps.storage.get(b"poll").unwrap())?;
            Ok(to_binary(&poll)?)
        }
        QueryMsg::GetTally {} => {
            let poll_running: bool = deserialize(&deps.storage.get(b"running").unwrap())?;
            let tally: Tally = if poll_running {
                Tally { yes: 0, no: 0 }
            } else {
                deserialize(&deps.storage.get(b"tally").unwrap())?
            };
            Ok(to_binary(&tally)?)
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

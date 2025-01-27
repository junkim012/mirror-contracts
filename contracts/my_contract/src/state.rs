use cosmwasm_std::{Binary, CanonicalAddr, Decimal, StdResult, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use cw_storage_plus::Map;



use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use mirror_protocol::common::OrderBy;
use mirror_protocol::gov::{PollStatus, VoterInfo};

static KEY_CONFIG: &[u8] = b"config";
static KEY_STATE: &[u8] = b"state";
static KEY_TMP_POLL_ID: &[u8] = b"tmp_poll_id";

static PREFIX_POLL_INDEXER: &[u8] = b"poll_indexer";
static PREFIX_POLL_VOTER: &[u8] = b"poll_voter";
static PREFIX_POLL: &[u8] = b"poll";
static PREFIX_BANK: &[u8] = b"bank";

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub lux_token: CanonicalAddr,
    pub owner: CanonicalAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub contract_addr: CanonicalAddr,
    pub event_count: u64,
    pub total_share: Uint128,
    pub total_deposit: Uint128,
}

// #[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct TokenManager {
//     pub share: Uint128,                        // total staked balance
//     pub locked_balance: Vec<(u64, VoterInfo)>, // maps poll_id to weight voted
//     pub participated_polls: Vec<u64>,          // poll_id
// }

// TODO: global variable for keeping track of EVENTS
// Map<event_id, Event> 
// pub trait MapTrait {
//     type Map;
// }

pub const EVENTS: Map<&[u8], Event> = Map::new(b"events");
// pub const EVENTS: <(dyn Storage + 'static) as MapTrait>::Map = Storage::Map::new(b"events");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Event<'a> {
    pub event_id: u64,
    pub creator: CanonicalAddr,
    pub status: Status,
    pub asset_name: String,
    pub strike_price: Uint128,
    pub start_time: Uint128, 
    pub end_time: Uint128, 
    pub expiration_date: Uint128, 
    pub option_one_shares: Uint128,
    pub option_two_shares: Uint128,
    pub option_one_deposit: Uint128,
    pub option_two_deposit: Uint128,
    pub winning_option: WagerOption,
    // TODO: what type should the key be? 
    // pub WAGERS: Storage::Map<&'a [u8], Wager> // Map of <K: addr, V: Vec<Wager>> 
    pub WAGERS: <(dyn Storage + 'static) as MapTrait>::Map 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Wager { 
    pub user_addr: CanonicalAddr, 
    pub wager_option: WagerOption,
    pub option_one_shares: Uint128,
    pub option_two_shares: Uint128, 
    pub option_one_deposits: Uint128,
    pub option_two_deposits: Uint128,
}

pub enum Status {
    OPEN,
    CLOSED,
    EXPIRED,
}

pub enum WagerOption {
    ONE,
    TWO
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExecuteData {
    pub contract: CanonicalAddr,
    pub msg: Binary,
}

// pub fn store_tmp_poll_id(storage: &mut dyn Storage, tmp_poll_id: u64) -> StdResult<()> {
//     singleton(storage, KEY_TMP_POLL_ID).save(&tmp_poll_id)
// }

// pub fn read_tmp_poll_id(storage: &dyn Storage) -> StdResult<u64> {
//     singleton_read(storage, KEY_TMP_POLL_ID).load()
// }

pub fn config_store(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, KEY_CONFIG)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, KEY_CONFIG)
}

pub fn state_store(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, KEY_STATE)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, KEY_STATE)
}

// TODO: Question: Do I need this for EVENTS Map?
// pub fn poll_store(storage: &mut dyn Storage) -> Bucket<Poll> {
//     bucket(storage, PREFIX_POLL)
// }

// pub fn poll_read(storage: &dyn Storage) -> ReadonlyBucket<Poll> {
//     bucket_read(storage, PREFIX_POLL)
// }

// pub fn poll_indexer_store<'a>(
//     storage: &'a mut dyn Storage,
//     status: &PollStatus,
// ) -> Bucket<'a, bool> {
//     Bucket::multilevel(
//         storage,
//         &[PREFIX_POLL_INDEXER, status.to_string().as_bytes()],
//     )
// }

// pub fn poll_voter_store(storage: &mut dyn Storage, poll_id: u64) -> Bucket<VoterInfo> {
//     Bucket::multilevel(storage, &[PREFIX_POLL_VOTER, &poll_id.to_be_bytes()])
// }

// pub fn poll_voter_read(storage: &dyn Storage, poll_id: u64) -> ReadonlyBucket<VoterInfo> {
//     ReadonlyBucket::multilevel(storage, &[PREFIX_POLL_VOTER, &poll_id.to_be_bytes()])
// }

// pub fn read_poll_voters<'a>(
//     storage: &'a dyn Storage,
//     poll_id: u64,
//     start_after: Option<CanonicalAddr>,
//     limit: Option<u32>,
//     order_by: Option<OrderBy>,
// ) -> StdResult<Vec<(CanonicalAddr, VoterInfo)>> {
//     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
//     let (start, end, order_by) = match order_by {
//         Some(OrderBy::Asc) => (calc_range_start_addr(start_after), None, OrderBy::Asc),
//         _ => (None, calc_range_end_addr(start_after), OrderBy::Desc),
//     };

//     let voters: ReadonlyBucket<'a, VoterInfo> =
//         ReadonlyBucket::multilevel(storage, &[PREFIX_POLL_VOTER, &poll_id.to_be_bytes()]);
//     voters
//         .range(start.as_deref(), end.as_deref(), order_by.into())
//         .take(limit)
//         .map(|item| {
//             let (k, v) = item?;
//             Ok((CanonicalAddr::from(k), v))
//         })
//         .collect()
// }

// pub fn read_polls<'a>(
//     storage: &'a dyn Storage,
//     filter: Option<PollStatus>,
//     start_after: Option<u64>,
//     limit: Option<u32>,
//     order_by: Option<OrderBy>,
//     remove_hard_cap: Option<bool>,
// ) -> StdResult<Vec<Poll>> {
//     let mut limit: usize = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
//     if let Some(remove_hard_cap) = remove_hard_cap {
//         if remove_hard_cap {
//             limit = usize::MAX;
//         }
//     }
//     let (start, end, order_by) = match order_by {
//         Some(OrderBy::Asc) => (calc_range_start(start_after), None, OrderBy::Asc),
//         _ => (None, calc_range_end(start_after), OrderBy::Desc),
//     };

//     if let Some(status) = filter {
//         let poll_indexer: ReadonlyBucket<'a, bool> = ReadonlyBucket::multilevel(
//             storage,
//             &[PREFIX_POLL_INDEXER, status.to_string().as_bytes()],
//         );
//         poll_indexer
//             .range(start.as_deref(), end.as_deref(), order_by.into())
//             .take(limit)
//             .map(|item| {
//                 let (k, _) = item?;
//                 poll_read(storage).load(&k)
//             })
//             .collect()
//     } else {
//         let polls: ReadonlyBucket<'a, Poll> = ReadonlyBucket::new(storage, PREFIX_POLL);

//         polls
//             .range(start.as_deref(), end.as_deref(), order_by.into())
//             .take(limit)
//             .map(|item| {
//                 let (_, v) = item?;
//                 Ok(v)
//             })
//             .collect()
//     }
// }

// pub fn bank_store(storage: &mut dyn Storage) -> Bucket<TokenManager> {
//     bucket(storage, PREFIX_BANK)
// }

// pub fn bank_read(storage: &dyn Storage) -> ReadonlyBucket<TokenManager> {
//     bucket_read(storage, PREFIX_BANK)
// }

// pub fn read_bank_stakers<'a>(
//     storage: &'a dyn Storage,
//     start_after: Option<CanonicalAddr>,
//     limit: Option<u32>,
//     order_by: Option<OrderBy>,
// ) -> StdResult<Vec<(CanonicalAddr, TokenManager)>> {
//     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
//     let (start, end, order_by) = match order_by {
//         Some(OrderBy::Asc) => (calc_range_start_addr(start_after), None, OrderBy::Asc),
//         _ => (None, calc_range_end_addr(start_after), OrderBy::Desc),
//     };

//     let stakers: ReadonlyBucket<'a, TokenManager> = ReadonlyBucket::new(storage, PREFIX_BANK);
//     stakers
//         .range(start.as_deref(), end.as_deref(), order_by.into())
//         .take(limit)
//         .map(|item| {
//             let (k, v) = item?;
//             Ok((CanonicalAddr::from(k), v))
//         })
//         .collect()
// }

// // this will set the first key after the provided key, by appending a 1 byte
// fn calc_range_start(start_after: Option<u64>) -> Option<Vec<u8>> {
//     start_after.map(|id| {
//         let mut v = id.to_be_bytes().to_vec();
//         v.push(1);
//         v
//     })
// }

// // this will set the first key after the provided key, by appending a 1 byte
// fn calc_range_end(start_after: Option<u64>) -> Option<Vec<u8>> {
//     start_after.map(|id| id.to_be_bytes().to_vec())
// }

// // this will set the first key after the provided key, by appending a 1 byte
// fn calc_range_start_addr(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
//     start_after.map(|addr| {
//         let mut v = addr.as_slice().to_vec();
//         v.push(1);
//         v
//     })
// }

// // this will set the first key after the provided key, by appending a 1 byte
// fn calc_range_end_addr(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
//     start_after.map(|addr| addr.as_slice().to_vec())
// }

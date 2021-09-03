#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

// use cw_storage_plus::Item;
// use cw_storage_plus::Map;

use crate::querier::load_token_balance;
// use crate::staking::{
//     deposit_reward, query_shares, query_staker, stake_voting_rewards, stake_voting_tokens,
//     withdraw_voting_rewards, withdraw_voting_tokens,
// };
use crate::state::{
    bank_read, bank_store, config_read, config_store, poll_indexer_store, poll_read, poll_store,
    poll_voter_read, poll_voter_store, read_poll_voters, read_polls, read_tmp_poll_id, state_read,
    state_store, store_tmp_poll_id, Config, ExecuteData, Event, Wager, Status, WagerOption, State,
};

use cosmwasm_std::{
    attr, from_binary, to_binary, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo,
    Reply, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128, WasmMsg, Storage
};

use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

use mirror_protocol::common::OrderBy;
use mirror_protocol::gov::{
    ConfigResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, PollExecuteMsg,
    PollResponse, PollStatus, PollsResponse, QueryMsg, StateResponse, VoteOption, VoterInfo,
    VotersResponse, VotersResponseItem,
};

use terraswap::{
    asset::{Asset, AssetInfo}, 
    querier::query_balance, 
};

// const MIN_TITLE_LENGTH: usize = 4;
// const MAX_TITLE_LENGTH: usize = 64;
// const MIN_DESC_LENGTH: usize = 4;
// const MAX_DESC_LENGTH: usize = 256;
// const MIN_LINK_LENGTH: usize = 12;
// const MAX_LINK_LENGTH: usize = 128;
// const MAX_POLLS_IN_PROGRESS: usize = 50;

// const POLL_EXECUTE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // validate_quorum(msg.quorum)?;
    // validate_threshold(msg.threshold)?;
    // validate_voter_weight(msg.voter_weight)?;

    let config = Config {
        lux_token: deps.api.addr_canonicalize(&msg.lux_token)?,
        owner: deps.api.addr_canonicalize(info.sender.as_str())?,
    };

    let state = State {
        contract_addr: deps.api.addr_canonicalize(env.contract.address.as_str())?,
        event_count: 0,
        total_share: Uint128::zero(),
        total_deposit: Uint128::zero(),
    };

    config_store(deps.storage).save(&config)?;
    state_store(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::UpdateConfig {
            owner
        } => update_config(
            owner
        ),
        ExecuteMsg::CreateEvent { 
            event_id,
            strike_price,
            asset_name,
            start_time,
            end_time,
            expiration_date,
            owner       
        } => create_event(deps, info, event_id, strike_price, asset_name, start_time, end_time, expiration_date, owner),
        ExecuteMsg::DepositWager { 
            user_addr,
            event_id,
            amount,
            wager_option 
        } => {
            deposit_wager(deps, info, user_addr, event_id, amount, wager_option)
        }
        ExecuteMsg::ClaimReward { 
            user_addr,
            event_id 
        } => claim_reward(deps, info, user_addr, event_id),
    }
}

pub fn create_event(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    event_id: u64,
    strike_price: Uint128,
    asset_name: String,
    start_time: Uint128,
    end_time: Uint128,
    expiration_date: Uint128,
) -> StdResult<Response> {

    let config: Config = config_read(deps.storage).load()?;
    let creator = deps.api.addr_canonicalize(info.sender.as_str())?;

    let mut state: State = state_store(deps.storage).load()?;
    let event_id = state.event_count + 1; 

    state.poll_count += 1;

    // create a new Event
    let WAGERS: Storage::Map<&[u8], Vec<Wager>> = Storage::Map::new();
    let event = Event {
        event_id: event_id, 
        creator: creator,
        status: Status::OPEN, 
        asset_name: asset_name, 
        strike_price: strike_price,
        start_time: start_time, 
        end_time: end_time, 
        expiration_date: expiration_date,  
        
        option_one_shares: Uint128::zero(),
        option_two_shares: Uint128::zero(),
        option_one_deposit: Uint128::zero(),
        option_two_deposit: Uint128::zero(),
        wagers: WAGERS
    };

    // save event to Events vector
    // TODO: how to save event_id as binary
    EVENTS.save(&mut deps.storage, event_id, event)?; 

    // returning a response with attributes
    // TODO: difference between add_messages? 
    // Ok(Response::new().add_messages(messages).add_attributes(vec![
    let r = Response::new().add_attributes(vec![
        attr("action", "create_event"),
        attr("event_id", event_id.to_string()),
        attr("strike_price", "100"),
        attr("asset_name", "luna"),
        attr("start_time", "1"), 
        attr("end_time", "2"),
        attr("expiration_date", "3"),
        attr("owner", "Alice".to_string()),
    ]);

    Ok(r)

}

pub fn deposit_wager(
    deps: DepsMut, 
    env: Env,
    info: MessageInfo,
    event_id: u64,
    amount: Uint128,
    wager_option: WagerOption, 
) -> StdResult<Response> {

    let user_addr = deps.api.addr_canonicalize(info.sender.as_str())?;

    // find the Event based on event Id on EVENTS map
    // modify the Event's wagers map 
    let updateEvents = |id: Option<Vec<Event>>, | -> StdResult<Vec<Event>> {
        // match id { // match event_id to Event 
        //     Some(one) => match one.WAGERS {
        //         Some(two) => Ok( {

        //         })

                
        //     }
        //     Ok(Event {


        //     }),
        //     None => Ok(Event { 

        //     })
        // }
    };

    // TODO: Question: does event_id need to be turned to bytecode? 
    EVENTS.update(&mut deps.storage, event_id, updateEvents);

    let r = Response::new().add_attributes(vec![
        attr("action", "deposit_wager"),
        attr("event_id", event_id.to_string()),
        attr("amount", amount.to_string()),
        attr("wager_option", wager_option.to_string()),
    ]);

    Ok(r)

}

pub fn claim_reward(
    deps: DepsMut,
    env: Env, 
    info: MessageInfo, 
    // user_addr: CanonicalAddr,
    event_id: u64, 
) -> StdResult<u64> {

    let user_addr = deps.api.addr_canonicalize(info.sender.as_str())?;

    // check if event_id exists
    let event = EVENTS.may_load(&deps.storage, event_id)?;
    if (event == None) {
        // return event_id doesn't exist error 
        return Err(StdError::generic_err("event id does not exist"));
    }

    // check if event has expired
    if (event.status != Status::EXPIRED) {
        // return event not yet expired error 
        return Err(StdError::generic_err("event not yet expired"));
    }

    // get Event's WAGERS map
    // find user address key and get Vec<Wager> value
    let wager = event.WAGERS.may_load(&deps.storage, user_addr)?;

    if (wager == None) {
        // return user does not have wager error 
        return Err(StdError::generic_err("user does not have wager"));
    }

    let mut winning_shares = Uint128::zero();
    if (event.winning_option == WagerOption::ONE) {
        winning_shares = wager.option_one_shares;
    }
    if (event.winning_option == WagerOption::TWO) {
        winning_shares = wager.option_two_shares; 
    }

    // distribute a dollar per winning share
    //  execute send ust to address
    // create the native asset, and then do asset.into_msg(..)

    // create the native asset, add to message with into_msg
    let reward_asset = Asset {
        info: AssetInfo::NativeToken {
            denom: "uust"
        },
        amount: Uint128::from(winning_shares)
    };

    if reward_asset.is_zero() {
        return Err(StdError::generic_err(
            "No winning shares and claimable rewards"
        ));
    }

    let tax_amount: Uint128 = reward_asset.compute_tax(&deps.querier)?;

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "claim rewards"),
            attr("amount", winning_shares.to_string())
        ])
        .add_message(reward_asset.into_msg(&deps.querier, info.sender)?))

        // .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        //     contract_addr: info.sender.as_str()?, // non-canonical, as string
        //     msg: to_binary(&Cw20ExecuteMsg::Transfer {
        //         recipient: info.sender.to_string(),
        //         amount: winning_shares,
        //     })?,
        //     // funds: vec![], ?
        // }))

}

// pub fn receive_cw20(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     cw20_msg: Cw20ReceiveMsg,
// ) -> StdResult<Response> {
//     // only asset contract can execute this message
//     let config: Config = config_read(deps.storage).load()?;
//     if config.mirror_token != deps.api.addr_canonicalize(info.sender.as_str())? {
//         return Err(StdError::generic_err("unauthorized"));
//     }

//     match from_binary(&cw20_msg.msg) {
//         Ok(Cw20HookMsg::StakeVotingTokens {}) => {
//             stake_voting_tokens(deps, cw20_msg.sender, cw20_msg.amount)
//         }
//         Ok(Cw20HookMsg::CreatePoll {
//             title,
//             description,
//             link,
//             execute_msg,
//         }) => create_poll(
//             deps,
//             env,
//             cw20_msg.sender,
//             cw20_msg.amount,
//             title,
//             description,
//             link,
//             execute_msg,
//         ),
//         Ok(Cw20HookMsg::DepositReward {}) => {
//             // only reward token contract can execute this message
//             if config.mirror_token != deps.api.addr_canonicalize(info.sender.as_str())? {
//                 return Err(StdError::generic_err("unauthorized"));
//             }
//             deposit_reward(deps, cw20_msg.amount)
//         }
//         Err(_) => Err(StdError::generic_err("data should be given")),
//     }
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
//     match msg.id {
//         POLL_EXECUTE_REPLY_ID => {
//             let poll_id: u64 = read_tmp_poll_id(deps.storage)?;
//             failed_poll(deps, poll_id)
//         }
//         _ => Err(StdError::generic_err("reply id is invalid")),
//     }
// }

#[allow(clippy::too_many_arguments)]
pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<String>,
    lux_token: String,
) -> StdResult<Response> {
    let api = deps.api;
    config_store(deps.storage).update(|mut config| {
        if config.owner != api.addr_canonicalize(info.sender.as_str())? {
            return Err(StdError::generic_err("unauthorized"));
        }

        if let Some(owner) = owner {
            config.owner = api.addr_canonicalize(&owner)?;
        }

        if let Some(lux_token) = lux_token {
            validate_lux_token(lux_token)?;
            config.lux_token = lux_token;
        }
        Ok(config)
    })?;
    Ok(Response::default())
}

// validate_lux_token returns an error if the token is invalid 
fn validate_lux_token(lux_token: String) -> StdResult<()> {
    Ok()
}

/*
 * Creates a new poll
 */
// #[allow(clippy::too_many_arguments)]
// pub fn create_poll(
//     deps: DepsMut,
//     env: Env,
//     proposer: String,
//     deposit_amount: Uint128,
//     title: String,
//     description: String,
//     link: Option<String>,
//     poll_execute_msg: Option<PollExecuteMsg>,
// ) -> StdResult<Response> {
//     validate_title(&title)?;
//     validate_description(&description)?;
//     validate_link(&link)?;

//     let config: Config = config_store(deps.storage).load()?;
//     if deposit_amount < config.proposal_deposit {
//         return Err(StdError::generic_err(format!(
//             "Must deposit more than {} token",
//             config.proposal_deposit
//         )));
//     }

//     let polls_in_progress: usize = read_polls(
//         deps.storage,
//         Some(PollStatus::InProgress),
//         None,
//         None,
//         None,
//         Some(true),
//     )?
//     .len();
//     if polls_in_progress.gt(&MAX_POLLS_IN_PROGRESS) {
//         return Err(StdError::generic_err("Too many polls in progress"));
//     }

//     let mut state: State = state_store(deps.storage).load()?;
//     let poll_id = state.poll_count + 1;

//     // Increase poll count & total deposit amount
//     state.poll_count += 1;
//     state.total_deposit += deposit_amount;

//     let poll_execute_data = if let Some(poll_execute_msg) = poll_execute_msg {
//         Some(ExecuteData {
//             contract: deps.api.addr_canonicalize(&poll_execute_msg.contract)?,
//             msg: poll_execute_msg.msg,
//         })
//     } else {
//         None
//     };

//     let sender_address_raw = deps.api.addr_canonicalize(&proposer)?;
//     let current_seconds = env.block.time.seconds();
//     let new_poll = Poll {
//         id: poll_id,
//         creator: sender_address_raw,
//         status: PollStatus::InProgress,
//         yes_votes: Uint128::zero(),
//         no_votes: Uint128::zero(),
//         abstain_votes: Uint128::zero(),
//         end_time: current_seconds + config.voting_period,
//         title,
//         description,
//         link,
//         execute_data: poll_execute_data,
//         deposit_amount,
//         total_balance_at_end_poll: None,
//         voters_reward: Uint128::zero(),
//         staked_amount: None,
//     };

//     poll_store(deps.storage).save(&poll_id.to_be_bytes(), &new_poll)?;
//     poll_indexer_store(deps.storage, &PollStatus::InProgress)
//         .save(&poll_id.to_be_bytes(), &true)?;

//     state_store(deps.storage).save(&state)?;

//     let r = Response::new().add_attributes(vec![
//         attr("action", "create_poll"),
//         attr(
//             "creator",
//             deps.api.addr_humanize(&new_poll.creator)?.as_str(),
//         ),
//         attr("poll_id", &poll_id.to_string()),
//         attr("end_time", new_poll.end_time.to_string()),
//     ]);
//     Ok(r)
// }

// /*
//  * Ends a poll.
//  */
// pub fn end_poll(deps: DepsMut, env: Env, poll_id: u64) -> StdResult<Response> {
//     let mut a_poll: Poll = poll_store(deps.storage).load(&poll_id.to_be_bytes())?;

//     if a_poll.status != PollStatus::InProgress {
//         return Err(StdError::generic_err("Poll is not in progress"));
//     }

//     let current_seconds = env.block.time.seconds();
//     if a_poll.end_time > current_seconds {
//         return Err(StdError::generic_err("Voting period has not expired"));
//     }

//     let no = a_poll.no_votes.u128();
//     let yes = a_poll.yes_votes.u128();
//     let abstain = a_poll.abstain_votes.u128();

//     let tallied_weight = yes + no + abstain;

//     let mut poll_status = PollStatus::Rejected;
//     let mut rejected_reason = "";
//     let mut passed = false;

//     let mut messages: Vec<CosmosMsg> = vec![];
//     let config: Config = config_read(deps.storage).load()?;
//     let mut state: State = state_read(deps.storage).load()?;

//     let (quorum, staked_weight) = if state.total_share.u128() == 0 {
//         (Decimal::zero(), Uint128::zero())
//     } else if let Some(staked_amount) = a_poll.staked_amount {
//         (
//             Decimal::from_ratio(tallied_weight, staked_amount),
//             staked_amount,
//         )
//     } else {
//         let total_locked_balance = state.total_deposit + state.pending_voting_rewards;
//         let staked_weight = load_token_balance(
//             &deps.querier,
//             deps.api.addr_humanize(&config.mirror_token)?.to_string(),
//             &state.contract_addr,
//         )?
//         .checked_sub(total_locked_balance)?;
//         (
//             Decimal::from_ratio(tallied_weight, staked_weight),
//             staked_weight,
//         )
//     };

//     if tallied_weight == 0 || quorum < config.quorum {
//         // Quorum: More than quorum of the total staked tokens at the end of the voting
//         // period need to have participated in the vote.
//         rejected_reason = "Quorum not reached";
//     } else {
//         if yes != 0u128 && Decimal::from_ratio(yes, yes + no) > config.threshold {
//             //Threshold: More than 50% of the tokens that participated in the vote
//             // (after excluding “Abstain” votes) need to have voted in favor of the proposal (“Yes”).
//             poll_status = PollStatus::Passed;
//             passed = true;
//         } else {
//             rejected_reason = "Threshold not reached";
//         }

//         // Refunds deposit only when quorum is reached
//         if !a_poll.deposit_amount.is_zero() {
//             messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
//                 contract_addr: deps.api.addr_humanize(&config.mirror_token)?.to_string(),
//                 funds: vec![],
//                 msg: to_binary(&Cw20ExecuteMsg::Transfer {
//                     recipient: deps.api.addr_humanize(&a_poll.creator)?.to_string(),
//                     amount: a_poll.deposit_amount,
//                 })?,
//             }))
//         }
//     }

//     // Decrease total deposit amount
//     state.total_deposit = state.total_deposit.checked_sub(a_poll.deposit_amount)?;
//     state_store(deps.storage).save(&state)?;

//     // Update poll indexer
//     poll_indexer_store(deps.storage, &PollStatus::InProgress).remove(&a_poll.id.to_be_bytes());
//     poll_indexer_store(deps.storage, &poll_status).save(&a_poll.id.to_be_bytes(), &true)?;

//     // Update poll status
//     a_poll.status = poll_status;
//     a_poll.total_balance_at_end_poll = Some(staked_weight);
//     poll_store(deps.storage).save(&poll_id.to_be_bytes(), &a_poll)?;

//     Ok(Response::new().add_messages(messages).add_attributes(vec![
//         attr("action", "end_poll"),
//         attr("poll_id", &poll_id.to_string()),
//         attr("rejected_reason", rejected_reason),
//         attr("passed", &passed.to_string()),
//     ]))
// }

// /*
//  * Execute a msg of passed poll.
//  */
// pub fn execute_poll(deps: DepsMut, env: Env, poll_id: u64) -> StdResult<Response> {
//     let config: Config = config_read(deps.storage).load()?;
//     let mut a_poll: Poll = poll_store(deps.storage).load(&poll_id.to_be_bytes())?;

//     if a_poll.status != PollStatus::Passed {
//         return Err(StdError::generic_err("Poll is not in passed status"));
//     }

//     let current_seconds = env.block.time.seconds();
//     if a_poll.end_time + config.effective_delay > current_seconds {
//         return Err(StdError::generic_err("Effective delay has not expired"));
//     }

//     poll_indexer_store(deps.storage, &PollStatus::Passed).remove(&poll_id.to_be_bytes());
//     poll_indexer_store(deps.storage, &PollStatus::Executed).save(&poll_id.to_be_bytes(), &true)?;

//     a_poll.status = PollStatus::Executed;
//     poll_store(deps.storage).save(&poll_id.to_be_bytes(), &a_poll)?;

//     let mut messages: Vec<SubMsg> = vec![];
//     if let Some(execute_data) = a_poll.execute_data {
//         messages.push(SubMsg {
//             msg: CosmosMsg::Wasm(WasmMsg::Execute {
//                 contract_addr: deps.api.addr_humanize(&execute_data.contract)?.to_string(),
//                 msg: execute_data.msg,
//                 funds: vec![],
//             }),
//             gas_limit: None,
//             id: POLL_EXECUTE_REPLY_ID,
//             reply_on: ReplyOn::Error,
//         });
//         store_tmp_poll_id(deps.storage, a_poll.id)?;
//     } else {
//         return Err(StdError::generic_err("The poll does not have execute_data"));
//     }

//     Ok(Response::new()
//         .add_submessages(messages)
//         .add_attributes(vec![
//             attr("action", "execute_poll"),
//             attr("poll_id", poll_id.to_string()),
//         ]))
// }

// /*
//  * If the executed message of a passed poll fails, it is marked as failed
//  */
// pub fn failed_poll(deps: DepsMut, poll_id: u64) -> StdResult<Response> {
//     let mut a_poll: Poll = poll_store(deps.storage).load(&poll_id.to_be_bytes())?;

//     poll_indexer_store(deps.storage, &PollStatus::Executed).remove(&poll_id.to_be_bytes());
//     poll_indexer_store(deps.storage, &PollStatus::Failed).save(&poll_id.to_be_bytes(), &true)?;

//     a_poll.status = PollStatus::Failed;
//     poll_store(deps.storage).save(&poll_id.to_be_bytes(), &a_poll)?;

//     Ok(Response::new().add_attribute("action", "failed_poll"))
// }

// /*
//  * User casts a vote on the provided poll id
//  */
// pub fn cast_vote(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     poll_id: u64,
//     vote: VoteOption,
//     amount: Uint128,
// ) -> StdResult<Response> {
//     let sender_address_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
//     let config = config_read(deps.storage).load()?;
//     let state = state_read(deps.storage).load()?;
//     if poll_id == 0 || state.poll_count < poll_id {
//         return Err(StdError::generic_err("Poll does not exist"));
//     }

//     let mut a_poll: Poll = poll_store(deps.storage).load(&poll_id.to_be_bytes())?;
//     let current_seconds = env.block.time.seconds();
//     if a_poll.status != PollStatus::InProgress || env.block.height > current_seconds {
//         return Err(StdError::generic_err("Poll is not in progress"));
//     }

//     // Check the voter already has a vote on the poll
//     if poll_voter_read(deps.storage, poll_id)
//         .load(sender_address_raw.as_slice())
//         .is_ok()
//     {
//         return Err(StdError::generic_err("User has already voted."));
//     }

//     let key = &sender_address_raw.as_slice();
//     let mut token_manager = bank_read(deps.storage).may_load(key)?.unwrap_or_default();

//     // convert share to amount
//     let total_share = state.total_share;
//     let total_locked_balance = state.total_deposit + state.pending_voting_rewards;
//     let total_balance = load_token_balance(
//         &deps.querier,
//         deps.api.addr_humanize(&config.mirror_token)?.to_string(),
//         &state.contract_addr,
//     )?
//     .checked_sub(total_locked_balance)?;

//     if token_manager
//         .share
//         .multiply_ratio(total_balance, total_share)
//         < amount
//     {
//         return Err(StdError::generic_err(
//             "User does not have enough staked tokens.",
//         ));
//     }

//     // update tally info
//     match vote {
//         VoteOption::Yes => a_poll.yes_votes += amount,
//         VoteOption::No => a_poll.no_votes += amount,
//         VoteOption::Abstain => a_poll.abstain_votes += amount,
//     }

//     let vote_info = VoterInfo {
//         vote,
//         balance: amount,
//     };
//     token_manager
//         .locked_balance
//         .push((poll_id, vote_info.clone()));
//     token_manager.participated_polls = vec![];
//     bank_store(deps.storage).save(key, &token_manager)?;

//     // store poll voter && and update poll data
//     poll_voter_store(deps.storage, poll_id).save(sender_address_raw.as_slice(), &vote_info)?;

//     // processing snapshot
//     let current_seconds = env.block.time.seconds();
//     let time_to_end = a_poll.end_time - current_seconds;

//     if time_to_end < config.snapshot_period && a_poll.staked_amount.is_none() {
//         a_poll.staked_amount = Some(total_balance);
//     }

//     poll_store(deps.storage).save(&poll_id.to_be_bytes(), &a_poll)?;

//     let attributes = vec![
//         attr("action", "cast_vote"),
//         attr("poll_id", &poll_id.to_string()),
//         attr("amount", &amount.to_string()),
//         attr("voter", &info.sender.to_string()),
//         attr("vote_option", vote_info.vote.to_string()),
//     ];

//     Ok(Response::new().add_attributes(attributes))
// }

// /*
//  * SnapshotPoll is used to take a snapshot of the staked amount for quorum calculation
//  */
// pub fn snapshot_poll(deps: DepsMut, env: Env, poll_id: u64) -> StdResult<Response> {
//     let config: Config = config_read(deps.storage).load()?;
//     let mut a_poll: Poll = poll_store(deps.storage).load(&poll_id.to_be_bytes())?;

//     if a_poll.status != PollStatus::InProgress {
//         return Err(StdError::generic_err("Poll is not in progress"));
//     }

//     let current_seconds = env.block.time.seconds();
//     let time_to_end = a_poll.end_time - current_seconds;

//     if time_to_end > config.snapshot_period {
//         return Err(StdError::generic_err("Cannot snapshot at this height"));
//     }

//     if a_poll.staked_amount.is_some() {
//         return Err(StdError::generic_err("Snapshot has already occurred"));
//     }

//     // store the current staked amount for quorum calculation
//     let state: State = state_store(deps.storage).load()?;

//     let total_locked_balance = state.total_deposit + state.pending_voting_rewards;
//     let staked_amount = load_token_balance(
//         &deps.querier,
//         deps.api.addr_humanize(&config.mirror_token)?.to_string(),
//         &state.contract_addr,
//     )?
//     .checked_sub(total_locked_balance)?;

//     a_poll.staked_amount = Some(staked_amount);

//     poll_store(deps.storage).save(&poll_id.to_be_bytes(), &a_poll)?;

//     Ok(Response::new().add_attributes(vec![
//         attr("action", "snapshot_poll"),
//         attr("poll_id", poll_id.to_string()),
//         attr("staked_amount", staked_amount),
//     ]))
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Staker { address } => to_binary(&query_staker(deps, address)?),
        QueryMsg::Poll { poll_id } => to_binary(&query_poll(deps, poll_id)?),
        QueryMsg::Polls {
            filter,
            start_after,
            limit,
            order_by,
        } => to_binary(&query_polls(deps, filter, start_after, limit, order_by)?),
        QueryMsg::Voter { poll_id, address } => to_binary(&query_voter(deps, poll_id, address)?),
        QueryMsg::Voters {
            poll_id,
            start_after,
            limit,
            order_by,
        } => to_binary(&query_voters(deps, poll_id, start_after, limit, order_by)?),
        QueryMsg::Shares {
            start_after,
            limit,
            order_by,
        } => to_binary(&query_shares(deps, start_after, limit, order_by)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = config_read(deps.storage).load()?;
    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?.to_string(),
        mirror_token: deps.api.addr_humanize(&config.mirror_token)?.to_string(),
        quorum: config.quorum,
        threshold: config.threshold,
        voting_period: config.voting_period,
        effective_delay: config.effective_delay,
        proposal_deposit: config.proposal_deposit,
        voter_weight: config.voter_weight,
        snapshot_period: config.snapshot_period,
    })
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state: State = state_read(deps.storage).load()?;
    Ok(StateResponse {
        poll_count: state.poll_count,
        total_share: state.total_share,
        total_deposit: state.total_deposit,
        pending_voting_rewards: state.pending_voting_rewards,
    })
}

fn query_poll(deps: Deps, poll_id: u64) -> StdResult<PollResponse> {
    let poll = match poll_read(deps.storage).may_load(&poll_id.to_be_bytes())? {
        Some(poll) => Some(poll),
        None => return Err(StdError::generic_err("Poll does not exist")),
    }
    .unwrap();

    Ok(PollResponse {
        id: poll.id,
        creator: deps.api.addr_humanize(&poll.creator).unwrap().to_string(),
        status: poll.status,
        end_time: poll.end_time,
        title: poll.title,
        description: poll.description,
        link: poll.link,
        deposit_amount: poll.deposit_amount,
        execute_data: if let Some(execute_data) = poll.execute_data {
            Some(PollExecuteMsg {
                contract: deps.api.addr_humanize(&execute_data.contract)?.to_string(),
                msg: execute_data.msg,
            })
        } else {
            None
        },
        yes_votes: poll.yes_votes,
        no_votes: poll.no_votes,
        abstain_votes: poll.abstain_votes,
        total_balance_at_end_poll: poll.total_balance_at_end_poll,
        voters_reward: poll.voters_reward,
        staked_amount: poll.staked_amount,
    })
}

fn query_polls(
    deps: Deps,
    filter: Option<PollStatus>,
    start_after: Option<u64>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<PollsResponse> {
    let polls = read_polls(deps.storage, filter, start_after, limit, order_by, None)?;
    let poll_responses: StdResult<Vec<PollResponse>> = polls
        .iter()
        .map(|poll| {
            Ok(PollResponse {
                id: poll.id,
                creator: deps.api.addr_humanize(&poll.creator).unwrap().to_string(),
                status: poll.status.clone(),
                end_time: poll.end_time,
                title: poll.title.to_string(),
                description: poll.description.to_string(),
                link: poll.link.clone(),
                deposit_amount: poll.deposit_amount,
                execute_data: if let Some(execute_data) = poll.execute_data.clone() {
                    Some(PollExecuteMsg {
                        contract: deps.api.addr_humanize(&execute_data.contract)?.to_string(),
                        msg: execute_data.msg,
                    })
                } else {
                    None
                },
                yes_votes: poll.yes_votes,
                no_votes: poll.no_votes,
                abstain_votes: poll.abstain_votes,
                total_balance_at_end_poll: poll.total_balance_at_end_poll,
                voters_reward: poll.voters_reward,
                staked_amount: poll.staked_amount,
            })
        })
        .collect();

    Ok(PollsResponse {
        polls: poll_responses?,
    })
}

fn query_voter(deps: Deps, poll_id: u64, address: String) -> StdResult<VotersResponseItem> {
    let voter: VoterInfo = poll_voter_read(deps.storage, poll_id)
        .load(deps.api.addr_canonicalize(&address)?.as_slice())?;
    Ok(VotersResponseItem {
        voter: address,
        vote: voter.vote,
        balance: voter.balance,
    })
}

fn query_voters(
    deps: Deps,
    poll_id: u64,
    start_after: Option<String>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<VotersResponse> {
    let voters = if let Some(start_after) = start_after {
        read_poll_voters(
            deps.storage,
            poll_id,
            Some(deps.api.addr_canonicalize(&start_after)?),
            limit,
            order_by,
        )?
    } else {
        read_poll_voters(deps.storage, poll_id, None, limit, order_by)?
    };

    let voters_response: StdResult<Vec<VotersResponseItem>> = voters
        .iter()
        .map(|voter_info| {
            Ok(VotersResponseItem {
                voter: deps.api.addr_humanize(&voter_info.0)?.to_string(),
                vote: voter_info.1.vote.clone(),
                balance: voter_info.1.balance,
            })
        })
        .collect();

    Ok(VotersResponse {
        voters: voters_response?,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

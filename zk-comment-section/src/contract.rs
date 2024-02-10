#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;
extern crate pbc_lib;

mod zk_compute;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::zk::ZkClosed;
use pbc_contract_common::zk::{CalculationStatus, SecretVarId, ZkInputDef, ZkState, ZkStateChange};
use pbc_zk::Sbi32;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// Secret variable metadata. Unused for this contract, so we use a zero-sized struct to save space.
#[derive(ReadWriteState, ReadWriteRPC, Debug)]
#[repr(u8)]
enum SecretVarType {
    #[discriminant(0)]
    Message {},
    #[discriminant(1)]
    ConcatResult {},
}

/// u32 / u8 = 4
const MAX_NUM_COMMENTS: u32 = 4;

/// This contract's state
#[state]
struct ContractState {
    /// Address allowed to start computation
    administrator: Address,
    /// Will contain the result (average) when computation is complete
    concat_message_result: Option<u32>,
    /// Will contain the number of employees after starting the computation
    num_comments: Option<u32>,
}

/// Initializes contract
///
/// Note that administrator is set to whoever initializes the contact.
#[init(zk = true)]
fn initialize(ctx: ContractContext, zk_state: ZkState<SecretVarType>) -> ContractState {
    ContractState {
        administrator: ctx.sender,
        concat_message_result: None,
        num_comments: None,
    }
}

/// Adds another salary variable
#[zk_on_secret_input(shortname = 0x40)]
fn add_message(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarType>,
) -> (
    ContractState,
    Vec<EventGroup>,
    ZkInputDef<SecretVarType, Sbi32>,
) {
    assert!(
        zk_state
            .secret_variables
            .iter()
            .chain(zk_state.pending_inputs.iter())
            .all(|(_, v)| v.owner != context.sender),
        "Each address is only allowed to send one salary variable. Sender: {:?}",
        context.sender
    );
    let input_def = ZkInputDef::with_metadata(SecretVarType::Message {});
    (state, vec![], input_def)
}

/// Automatically called when a variable is confirmed on chain.
///
/// Unused for this contract, so we do nothing.
#[zk_on_variable_inputted]
fn inputted_variable(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarType>,
    inputted_variable: SecretVarId,
) -> ContractState {
    state
}

/// Allows the administrator to start the computation of the average salary.
///
/// The averaging computation is automatic beyond this call, involving several steps, as described in the module documentation.
#[action(shortname = 0x01, zk = true)]
fn compute_concat_message(
    context: ContractContext,
    mut state: ContractState,
    zk_state: ZkState<SecretVarType>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    assert_eq!(
        context.sender, state.administrator,
        "Only administrator can start computation"
    );
    assert_eq!(
        zk_state.calculation_state,
        CalculationStatus::Waiting,
        "Computation must start from Waiting state, but was {:?}",
        zk_state.calculation_state,
    );

    let num_comments = zk_state.secret_variables.len() as u32;
    assert!(num_comments <= MAX_NUM_COMMENTS , "At most {MAX_NUM_COMMENTS} comments can be encoded, before starting computation, but had only {num_comments}");

    state.num_comments = Some(num_comments);
    (
        state,
        vec![],
        vec![zk_compute::concat_everything_start(
            &SecretVarType::ConcatResult {},
        )],
    )
}

/// Automatically called when the computation is completed
///
/// The only thing we do is to instantly open/declassify the output variables.
#[zk_on_compute_complete]
fn concat_compute_complete(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarType>,
    output_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    (
        state,
        vec![],
        vec![ZkStateChange::OpenVariables {
            variables: output_variables,
        }],
    )
}

/// Automatically called when a variable is opened/declassified.
///
/// We can now read the sum variable, and compute the average, which will be our final result.
#[zk_on_variables_opened]
fn open_concat_variable(
    context: ContractContext,
    mut state: ContractState,
    zk_state: ZkState<SecretVarType>,
    opened_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    assert_eq!(
        opened_variables.len(),
        1,
        "Unexpected number of output variables"
    );
    let opened_variable = zk_state
        .get_variable(*opened_variables.get(0).unwrap())
        .unwrap();

    let result = read_variable_u32_le(&opened_variable);

    let mut zk_state_changes = vec![];
    if let SecretVarType::ConcatResult {} = opened_variable.metadata {
        state.concat_message_result = Some(result);
        zk_state_changes = vec![ZkStateChange::ContractDone];
    }
    (state, vec![], zk_state_changes)
}

/// Reads a variable's data as an u32.
fn read_variable_u32_le(sum_variable: &ZkClosed<SecretVarType>) -> u32 {
    let mut buffer = [0u8; 4];
    buffer.copy_from_slice(sum_variable.data.as_ref().unwrap().as_slice());
    <u32>::from_le_bytes(buffer)
}

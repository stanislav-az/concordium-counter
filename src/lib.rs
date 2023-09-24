#![cfg_attr(not(feature = "std"), no_std)]

//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Counter state
#[derive(Serialize, SchemaType)]
pub struct State {
    counter: i8,
}

/// Counter smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// The transaction must be triggered by the owner of the contract
    OwnerError,
    IncrementError,
    DecrementError,
}

/// Init function that creates a new smart contract.
#[init(contract = "counter")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    Ok(State { counter: 0 })
}

/// Receive function. The input parameter is the increment value `i8`.
///  If the account owner does not match the contract owner, the receive function will throw [`Error::OwnerError`].
///  If the number to increment by is not positive or is zero, the receive function will throw [`Error::IncrementError`].
#[receive(
    contract = "counter",
    name = "increment",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn increment<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    let increment_val: i8 = ctx.parameter_cursor().get()?;
    let state = host.state_mut();

    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(increment_val > 0, Error::IncrementError);

    state.counter += increment_val;

    Ok(())
}

#[receive(
    contract = "counter",
    name = "decrement",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn decrement<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    let decrement_val: i8 = ctx.parameter_cursor().get()?;
    let state = host.state_mut();

    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(decrement_val < 0, Error::DecrementError);

    state.counter += decrement_val;

    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "counter", name = "view", return_value = "i8")]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<i8> {
    Ok(host.state().counter)
}

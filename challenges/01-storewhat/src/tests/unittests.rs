use super::*;
use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, StdError};
use crate::ContractError;
use crate::contract::{execute, instantiate};
use cosmwasm_std::{Coin, Uint128};
use cosmwasm_std::BalanceResponse;
use crate::contract::query;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, DebtResponse};

#[test]
fn invalid_init() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(0, DENOM.to_string()));
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, ContractError::InvalidInstantiation {});
}


#[test]
fn deposit_success() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // user able to deposit funds
    let info = mock_info("alice", &coins(100, DENOM));
    let msg = ExecuteMsg::Deposit {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // verify deposit succeeded
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetBalance {
            address: "alice".to_string(),
        },
    )
    .unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!(Uint128::from(100_u64), value.amount.amount);
}

#[test]
fn deposit_failure() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // other funds such as uluna with not be recorded
    let info = mock_info("bob", &coins(10, "uluna".to_string()));
    let msg = ExecuteMsg::Deposit {};
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, ContractError::Std(StdError::generic_err("Invalid deposit!")));
}

#[test]
fn borrow_success() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice deposits 1000 funds
    let info = mock_info("alice", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice able borrow 500 funds (50% of 1000)
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("alice", &empty_fund);
    let msg = ExecuteMsg::Borrow {
        amount: Uint128::from(500_u64),
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
}

#[test]
fn borrow_fail() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // bob deposits 1000 funds
    let info = mock_info("bob", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // bob cannot borrow more than 50% of deposited funds
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("bob", &empty_fund);
    let msg = ExecuteMsg::Borrow {
        amount: Uint128::from(501_u64),
    };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, ContractError::Std(StdError::generic_err("Borrow too much!")));
}
#[ignore="fixed"]
#[test]
fn borrow_abuse() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice deposits 1000 funds
    let info = mock_info("alice", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice able borrow 500 funds (50% of 1000)
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("alice", &empty_fund);
    let msg = ExecuteMsg::Borrow {
        amount: Uint128::from(200_u64),
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("alice", &empty_fund);
    let msg = ExecuteMsg::Borrow {
        amount: Uint128::from(300_u64),
    };
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    // verify deposit succeeded
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetDebt   {
            address: "alice".to_string(),
        },
    )
    .unwrap();
    let value: DebtResponse = from_binary(&res).unwrap();
    assert_eq!(Uint128::from(500_u64), value.amount);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, ContractError::Std(StdError::generic_err("Borrow too much!")));

}
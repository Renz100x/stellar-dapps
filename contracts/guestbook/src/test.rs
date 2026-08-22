#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_errors() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GuestbookContract);
    let client = GuestbookContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    env.mock_all_auths();

    // 1. Uji Error: EmptyMessage
    let empty_msg = String::from_str(&env, "");
    let res_empty = client.try_write_message(&user, &empty_msg);
    assert_eq!(res_empty, Err(Ok(Error::EmptyMessage)));

    // 2. Uji Error: MessageTooLong (> 280 karakter)
    let long_str = "a".repeat(281);
    let long_msg = String::from_str(&env, &long_str);
    let res_long = client.try_write_message(&user, &long_msg);
    assert_eq!(res_long, Err(Ok(Error::MessageTooLong)));
}
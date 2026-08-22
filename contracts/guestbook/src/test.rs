#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_write_and_get_message() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GuestbookContract);
    let client = GuestbookContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    env.mock_all_auths();

    let msg = String::from_str(&env, "Hello Soroban!");
    client.write_message(&user, &msg);

    let messages = client.get_messages();
    assert_eq!(messages.len(), 1);
}
#![no_std]

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct GuestbookEntry {
    pub sender: Address,
    pub message: String,
    pub timestamp: u64,
}

#[contract]
pub struct GuestbookContract;

#[contractimpl]
impl GuestbookContract {
    pub fn write_message(env: Env, from: Address, message: String) {
        from.require_auth();

        if message.len() == 0 {
            panic!("Error: Pesan tidak boleh kosong");
        }

        let entry = GuestbookEntry {
            sender: from.clone(),
            message: message.clone(),
            timestamp: env.ledger().timestamp(),
        };

        let key = symbol_short!("entries");
        let mut entries: Vec<GuestbookEntry> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));
            
        entries.push_back(entry);
        env.storage().persistent().set(&key, &entries);

        let topic = symbol_short!("new_msg");
        env.events().publish((topic, from), message);
    }

    pub fn get_messages(env: Env) -> Vec<GuestbookEntry> {
        let key = symbol_short!("entries");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }
}
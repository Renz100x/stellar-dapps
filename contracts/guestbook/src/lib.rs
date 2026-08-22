#![no_std]

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String, Vec,
};

// --- Custom Error Types (3 Jenis Error Handled) ---
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    EmptyMessage = 1,      // Error 1: Pesan tidak boleh kosong
    MessageTooLong = 2,    // Error 2: Pesan terlalu panjang (> 280 karakter)
    MaxEntriesReached = 3, // Error 3: Batas kapasitas penyimpanan guestbook penuh
}

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
    pub fn write_message(env: Env, from: Address, message: String) -> Result<(), Error> {
        from.require_auth();

        // Validasi 1: Cek apakah pesan kosong
        if message.len() == 0 {
            return Err(Error::EmptyMessage);
        }

        // Validasi 2: Cek panjang pesan (maksimal 280 karakter)
        if message.len() > 280 {
            return Err(Error::MessageTooLong);
        }

        let key = symbol_short!("entries");
        let mut entries: Vec<GuestbookEntry> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        // Validasi 3: Cek batas maksimum entri guestbook (misal maks 500 pesan)
        if entries.len() >= 500 {
            return Err(Error::MaxEntriesReached);
        }

        let entry = GuestbookEntry {
            sender: from.clone(),
            message: message.clone(),
            timestamp: env.ledger().timestamp(),
        };

        entries.push_back(entry);
        env.storage().persistent().set(&key, &entries);

        let topic = symbol_short!("new_msg");
        env.events().publish((topic, from), message);

        Ok(())
    }

    pub fn get_messages(env: Env) -> Vec<GuestbookEntry> {
        let key = symbol_short!("entries");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }
}
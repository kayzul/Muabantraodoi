#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env,
};

#[contract]
pub struct RewardToken;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Admin,
}

#[contractimpl]
impl RewardToken {

    // Khởi tạo Admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
    }

    // Admin cộng điểm cho user
    pub fn mint(
        env: Env,
        admin: Address,
        user: Address,
        amount: i128,
    ) {
        admin.require_auth();

        let saved_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != saved_admin {
            panic!("Not admin");
        }

        let key = DataKey::Balance(user.clone());

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&key, &(balance + amount));
    }

    // Chuyển điểm
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        from.require_auth();

        let from_key = DataKey::Balance(from.clone());
        let to_key = DataKey::Balance(to.clone());

        let from_balance: i128 = env
            .storage()
            .persistent()
            .get(&from_key)
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env
            .storage()
            .persistent()
            .get(&to_key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&from_key, &(from_balance - amount));

        env.storage()
            .persistent()
            .set(&to_key, &(to_balance + amount));
    }

    // Xem số dư
    pub fn get_balance(
        env: Env,
        user: Address,
    ) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }
}
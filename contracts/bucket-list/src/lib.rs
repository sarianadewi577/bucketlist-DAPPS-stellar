#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, Vec, symbol_short};

// --- CONFIGURATION ---

pub mod storage {
    use super::*;
    pub const BUCKET_LIST: Symbol = symbol_short!("JOURNEY");
}

// --- DATA SCHEMA ---

#[contracttype]
#[derive(Clone, Debug)]
pub struct BucketEntry {
    pub id: u64,
    pub goal_title: String,
    pub story: String,
    pub achieved: bool,
}

// --- CORE LOGIC ---

#[contract]
pub struct BucketStoryContract;

#[contractimpl]
impl BucketStoryContract {

    /// Append a new life goal to the decentralized bucket list.
    pub fn add_goal(env: Env, title: String, initial_story: String) -> u64 {
        let mut list = Self::get_all_entries(env.clone());
        let id = env.prng().gen::<u64>();

        list.push_back(BucketEntry {
            id,
            goal_title: title,
            story: initial_story,
            achieved: false,
        });

        env.storage().instance().set(&storage::BUCKET_LIST, &list);
        id
    }

    /// Update the narrative progress for an existing goal.
    pub fn update_story_note(env: Env, id: u64, new_note: String) -> bool {
        let mut list = Self::get_all_entries(env.clone());
        
        for i in 0..list.len() {
            if let Some(mut entry) = list.get(i) {
                if entry.id == id {
                    entry.story = new_note;
                    list.set(i, entry);
                    env.storage().instance().set(&storage::BUCKET_LIST, &list);
                    return true;
                }
            }
        }
        false
    }

    /// Mark a bucket list goal as successfully achieved.
    pub fn mark_achieved(env: Env, id: u64) -> bool {
        let mut list = Self::get_all_entries(env.clone());
        
        for i in 0..list.len() {
            if let Some(mut entry) = list.get(i) {
                if entry.id == id {
                    entry.achieved = true;
                    list.set(i, entry);
                    env.storage().instance().set(&storage::BUCKET_LIST, &list);
                    return true;
                }
            }
        }
        false
    }

    /// Fetch the complete journey history from the blockchain.
    pub fn get_all_entries(env: Env) -> Vec<BucketEntry> {
        env.storage().instance().get(&storage::BUCKET_LIST).unwrap_or(Vec::new(&env))
    }
}
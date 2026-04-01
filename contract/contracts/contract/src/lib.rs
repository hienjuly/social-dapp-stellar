#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Profile(Address), // Stores profile info
    Post(u64),        // Stores individual posts
    PostCount,        // Tracks total post IDs
}

#[contracttype]
#[derive(Clone)]
pub struct Profile {
    pub name: String,
    pub bio: String,
}

#[contracttype]
#[derive(Clone)]
pub struct Post {
    pub author: Address,
    pub content: String,
    pub timestamp: u64,
}

#[contract]
pub struct SocialContract;

#[contractimpl]
impl SocialContract {
    /// Create or update a user profile
    pub fn update_profile(env: Env, user: Address, name: String, bio: String) {
        user.require_auth();
        let profile = Profile { name, bio };
        env.storage().persistent().set(&DataKey::Profile(user), &profile);
    }

    /// Submit a new post
    pub fn create_post(env: Env, author: Address, content: String) -> u64 {
        author.require_auth();

        // Get and increment post count for a unique ID
        let mut count: u64 = env.storage().instance().get(&DataKey::PostCount).unwrap_or(0);
        count += 1;

        let post = Post {
            author: author.clone(),
            content,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::Post(count), &post);
        env.storage().instance().set(&DataKey::PostCount, &count);

        count
    }

    /// Fetch a post by ID
    pub fn get_post(env: Env, id: u64) -> Option<Post> {
        env.storage().persistent().get(&DataKey::Post(id))
    }

    /// Fetch a profile by Address
    pub fn get_profile(env: Env, user: Address) -> Option<Profile> {
        env.storage().persistent().get(&DataKey::Profile(user))
    }
}
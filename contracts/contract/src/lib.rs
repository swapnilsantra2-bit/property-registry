#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Address, Map};

#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub id: u32,
    pub owner: Address,
    pub location: String,
    pub metadata: String,
}

#[contract]
pub struct PropertyRegistry;

#[contractimpl]
impl PropertyRegistry {

    // Register a new property
    pub fn register_property(
        env: Env,
        id: u32,
        owner: Address,
        location: String,
        metadata: String,
    ) {
        owner.require_auth();

        let mut properties: Map<u32, Property> =
            env.storage().instance().get(&Symbol::short("PROP")).unwrap_or(Map::new(&env));

        if properties.contains_key(id) {
            panic!("Property already exists");
        }

        let property = Property {
            id,
            owner: owner.clone(),
            location,
            metadata,
        };

        properties.set(id, property);
        env.storage().instance().set(&Symbol::short("PROP"), &properties);
    }

    // Get property details
    pub fn get_property(env: Env, id: u32) -> Property {
        let properties: Map<u32, Property> =
            env.storage().instance().get(&Symbol::short("PROP")).unwrap();

        properties.get(id).unwrap()
    }

    // Transfer ownership
    pub fn transfer_property(
        env: Env,
        id: u32,
        new_owner: Address,
    ) {
        let mut properties: Map<u32, Property> =
            env.storage().instance().get(&Symbol::short("PROP")).unwrap();

        let mut property = properties.get(id).unwrap();

        property.owner.require_auth();

        property.owner = new_owner.clone();
        properties.set(id, property);

        env.storage().instance().set(&Symbol::short("PROP"), &properties);
    }
}

use substreams::log;
use substreams_ethereum::pb::eth::v2 as eth;
use anyhow::Result;

// Required for WASM builds
#[cfg(target_arch = "wasm32")]
use getrandom::getrandom;

#[cfg(target_arch = "wasm32")]
fn getrandom_impl(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    getrandom(dest)
}

#[cfg(target_arch = "wasm32")]
getrandom::register_custom_getrandom!(getrandom_impl);

// Base Name Service contract addresses on Base
const REGISTRY_ADDRESS: &str = "0xb94704422c2a1e396835a571837aa5ae53285a95";
const REGISTRAR_ADDRESS: &str = "0x03c4738ee98ae44591e1a4a4f3cab6641d95dd9a";
const CONTROLLER_ADDRESS: &str = "0x79EA96012eEa67A83431F1701B3dFf7e37F9E282";
const REVERSE_ADDRESS: &str = "0xB94704422c2a1E396835A571837Aa5AE53285a95";

#[substreams::handlers::map]
pub fn map_registry_events(block: eth::Block) -> Result<pb::RegistryEvents> {
    let mut events = pb::RegistryEvents::default();
    let block_hash = hex::encode(&block.hash);
    let block_number = block.number.to_string();
    
    for log in block.logs() {
        if hex::encode(log.address()) != REGISTRY_ADDRESS {
            continue;
        }
        
        log::info!("Found registry event at block {}", block_number);
        
        // Create a simple event for demonstration
        let new_owner = pb::NewOwner {
            node: "0x".to_string(),
            label: "0x".to_string(),
            owner: "0x".to_string(),
            block_number: block_number.clone(),
            block_hash: block_hash.clone(),
            transaction_hash: "0x".to_string(),
            log_index: log.log.index,
        };
        
        events.new_owners.push(new_owner);
    }
    
    log::info!("Processed {} registry events", events.new_owners.len());
    Ok(events)
}

#[substreams::handlers::map]
pub fn map_registrar_events(block: eth::Block) -> Result<pb::RegistrarEvents> {
    let mut events = pb::RegistrarEvents::default();
    let block_hash = hex::encode(&block.hash);
    let block_number = block.number.to_string();
    
    for log in block.logs() {
        if hex::encode(log.address()) != REGISTRAR_ADDRESS {
            continue;
        }
        
        log::info!("Found registrar event at block {}", block_number);
        
        // Create a simple event for demonstration
        let name_registered = pb::NameRegistered {
            name: "example.base".to_string(),
            label: "example".to_string(),
            owner: "0x".to_string(),
            cost: 0,
            expires: 0,
            block_number: block_number.clone(),
            block_hash: block_hash.clone(),
            transaction_hash: "0x".to_string(),
            log_index: log.log.index,
        };
        
        events.name_registered.push(name_registered);
    }
    
    log::info!("Processed {} registrar events", events.name_registered.len());
    Ok(events)
}

#[substreams::handlers::map]
pub fn map_controller_events(block: eth::Block) -> Result<pb::ControllerEvents> {
    let mut events = pb::ControllerEvents::default();
    let block_hash = hex::encode(&block.hash);
    let block_number = block.number.to_string();
    
    for log in block.logs() {
        if hex::encode(log.address()) != CONTROLLER_ADDRESS {
            continue;
        }
        
        log::info!("Found controller event at block {}", block_number);
        
        // Create a simple event for demonstration
        let name_registered = pb::NameRegistered {
            name: "example.base".to_string(),
            label: "example".to_string(),
            owner: "0x".to_string(),
            cost: 0,
            expires: 0,
            block_number: block_number.clone(),
            block_hash: block_hash.clone(),
            transaction_hash: "0x".to_string(),
            log_index: log.log.index,
        };
        
        events.controller_name_registered.push(name_registered);
    }
    
    log::info!("Processed {} controller events", events.controller_name_registered.len());
    Ok(events)
}

#[substreams::handlers::map]
pub fn map_reverse_events(block: eth::Block) -> Result<pb::ReverseEvents> {
    let mut events = pb::ReverseEvents::default();
    let block_hash = hex::encode(&block.hash);
    let block_number = block.number.to_string();
    
    for log in block.logs() {
        if hex::encode(log.address()) != REVERSE_ADDRESS {
            continue;
        }
        
        log::info!("Found reverse event at block {}", block_number);
        
        // Create a simple event for demonstration
        let reverse_claimed = pb::ReverseClaimed {
            addr: "0x".to_string(),
            node: "0x".to_string(),
            owner: "0x".to_string(),
            block_number: block_number.clone(),
            block_hash: block_hash.clone(),
            transaction_hash: "0x".to_string(),
            log_index: log.log.index,
        };
        
        events.reverse_claimed.push(reverse_claimed);
    }
    
    log::info!("Processed {} reverse events", events.reverse_claimed.len());
    Ok(events)
}

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/base_names.v1.rs"));
}

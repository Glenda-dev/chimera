#![no_std]
#![no_main]
#![allow(dead_code)]

#[macro_use]
extern crate glenda;

mod chimera;
mod layout;

use chimera::ChimeraManager;
use glenda::cap::{CapType, ENDPOINT_CAP, ENDPOINT_SLOT, MONITOR_CAP, RECV_SLOT, REPLY_SLOT};
use glenda::client::{InitClient, ResourceClient};
use glenda::interface::{ResourceService, SystemService};
use glenda::ipc::Badge;
use layout::INIT_CAP;

#[unsafe(no_mangle)]
fn main() -> usize {
    glenda::console::init_logging("Chimera");
    log!("Starting Chimera VMM service...");

    let mut res_client = ResourceClient::new(MONITOR_CAP);
    let mut init_client = InitClient::new(INIT_CAP);

    if let Err(e) = res_client.alloc(Badge::null(), CapType::Endpoint, 0, ENDPOINT_SLOT) {
        error!("Endpoint alloc failed: {:?}", e);
        return 1;
    }

    let mut manager = ChimeraManager::new(&mut res_client, &mut init_client);
    if let Err(e) = manager.init() {
        error!("Init failed: {:?}", e);
        return 1;
    }
    if let Err(e) = manager.listen(ENDPOINT_CAP, REPLY_SLOT, RECV_SLOT) {
        error!("Listen failed: {:?}", e);
        return 1;
    }
    if let Err(e) = manager.run() {
        error!("Run failed: {:?}", e);
        return 1;
    }
    0
}

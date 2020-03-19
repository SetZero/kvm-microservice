use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct VirtualMachines {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct HyperVisorInfo {
    pub hypervisor: String,
    pub version: String,
}
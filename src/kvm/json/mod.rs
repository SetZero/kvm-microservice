use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct VirtualMachines {
    pub name: String,
    pub state: (virt::domain::DomainState, i32),
    pub memory: u64,
    pub vcpu: u64,
}

#[derive(Serialize, Deserialize)]
pub struct HyperVisorInfo {
    pub hypervisor: String,
    pub version: String,
}
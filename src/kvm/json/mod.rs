use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VirtualMachines {
    pub name: String,
    pub state: (virt::domain::DomainState, i32),
    pub memory: u64,
    pub vcpu: u64,
    pub autostart: bool,
    pub os_type: String

}

#[derive(Serialize, Deserialize)]
pub struct HyperVisorInfo {
    pub hypervisor: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct KVMInfo {
    pub success: bool,
    pub message: Option<String>,
}
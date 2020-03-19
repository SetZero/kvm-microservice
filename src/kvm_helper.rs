mod kvm {
    pub enum NetworkTypes {
        E1000,
        VIRTIO
    }

    pub enum Graphics {
        NONE
    }

    pub enum Console {
        PTY
    }

    pub enum ConsoleTargetType {
        SERIAL
    }

    pub struct VM {
        name: String,
        ram: u64,
        disk_path: String,
        disk_size: u64,
        vcpu: u32,
        network: String,
        network_model: NetworkTypes,
        graphics: Graphics,
        console: Console,
        target_type: ConsoleTargetType,
        import: bool,
        os_type: Option<String>,
        os_varian: Option<String>,
        location: Option<String>,
        extra_args: Option<String>,
    }

    impl VM {
    }
}
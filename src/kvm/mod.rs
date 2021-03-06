pub mod json;
use virt::connect::Connect;
use virt::error::Error;
use virt::domain::Domain;


pub struct KVM {
    conn: Connect
}

impl KVM {
    pub fn new(uri: String) -> KVM  {
        let conn = match Connect::open(&uri) {
            Ok(c) => c,
            Err(e) => std::panic!(
            "No connection to hypervisor: code {}, message: {}",
            e.code, e.message
            ),
        };

        KVM{conn}
    }

    pub fn show_hypervisor_info(&mut self) -> Result<json::HyperVisorInfo, Error> {
        if let Ok(hv_type) = self.conn.get_type() {
            if let Ok(mut hv_ver) = self.conn.get_hyp_version() {
                let major = hv_ver / 1000000;
                hv_ver %= 1000000;
                let minor = hv_ver / 1000;
                let release = hv_ver % 1000;
                let version = format!("{}.{}.{}", major, minor, release);
                return Ok(json::HyperVisorInfo{hypervisor: hv_type, version: version});
            }
        } else {
            self.disconnect();
        }
        Err(Error::new())
    }

    pub fn disconnect(&mut self) {
        if let Err(e) = self.conn.close() {
            std::panic!(
                "Failed to disconnect from hypervisor: code {}, message: {}",
                e.code, e.message
            );
        }
        println!("Disconnected from hypervisor");
    }

    pub fn get_domains(&self) -> Result<Vec<json::VirtualMachines>, Error> {
        let mut vec = Vec::new();
        let flags = virt::connect::VIR_CONNECT_LIST_DOMAINS_ACTIVE | virt::connect::VIR_CONNECT_LIST_DOMAINS_INACTIVE;
        if let Ok(doms) = self.conn.list_all_domains(flags) {
            for dom in doms {
                vec.push(self.create_vm_info(&dom));
            }
            return Ok(vec);
        }
        Err(Error::new())
    }

    pub fn start_vm(&self, name: String) -> Result<json::KVMInfo, String> {
        if let Ok(dom) = Domain::lookup_by_name(&self.conn, name.as_str()) {
            if dom.create().is_ok() {
                return Ok(json::KVMInfo{success: true, message: None})
            }
            return Err(Error::new().message)
        }

        Err(Error::new().message)
    }

    pub fn stop_vm(&self, name: String) -> Result<json::KVMInfo, String> {
        if let Ok(dom) = Domain::lookup_by_name(&self.conn, name.as_str()) {
            if dom.shutdown().is_ok() {
                return Ok(json::KVMInfo{success: true, message: None})
            }
            return Err(Error::new().message)
        }

        Err(Error::new().message)
    }

    pub fn suspend(&self, name: String) -> Result<json::KVMInfo, String> {
        if let Ok(dom) = Domain::lookup_by_name(&self.conn, name.as_str()) {
            if dom.suspend().is_ok() {
                return Ok(json::KVMInfo{success: true, message: None})
            }
            return Err(Error::new().message)
        }

        Err(Error::new().message)
    }

    pub fn resume(&self, name: String) -> Result<json::KVMInfo, String> {
        if let Ok(dom) = Domain::lookup_by_name(&self.conn, name.as_str()) {
            if dom.resume().is_ok() {
                return Ok(json::KVMInfo{success: true, message: None})
            }
            return Err(Error::new().message)
        }

        Err(Error::new().message)
    }

    pub fn list_snapshots(&self, name: String) -> Result<Vec<String>, String> {
        if let Ok(dom) = Domain::lookup_by_name(&self.conn, name.as_str()) {
            return dom.list_all_snapshots(0)
                .and_then(|e| Ok(e.into_iter().map(|e|  e.get_name().unwrap_or_default()).collect()))
                .map_err(|e| e.to_string())
        }

        Err(Error::new().message)
    }

    pub fn get_devices(&self, name: String) -> Result<Vec<String>, String> {
        if let Ok(dom) = Domain::lookup_by_name(&self.conn, name.as_str()) {
            return dom.get_connect()
                    .and_then(|e| e.list_all_node_devices(virt::connect::VIR_CONNECT_LIST_NODE_DEVICES_CAP_STORAGE | virt::connect::VIR_CONNECT_LIST_NODE_DEVICES_CAP_VPORTS))
                    .and_then(|e| Ok(e.into_iter().map(|e| e.get_name().unwrap_or_default()).collect()))
                    .map_err(|e| e.to_string())
        }
        Err(Error::new().message)
    }

    pub fn get_console_out(&self, name: String) -> Result<(), String> {
        if let Ok(dom) = Domain::lookup_by_name(&self.conn, name.as_str()) {
            let vir_box = Box::<virt::stream::sys::virStream>::new(virt::stream::sys::virStream{});
            let stream = virt::stream::Stream::new(Box::leak(vir_box) );
            dom.open_console("serial0", stream, 0);
        }
        Err(Error::new().message)
    }

    fn create_vm_info(&self, dom: &Domain) -> json::VirtualMachines {
        json::VirtualMachines{
            name: dom.get_name().unwrap_or(String::from("no-name")),
            state: dom.get_state().unwrap_or_default(),
            memory: dom.get_max_memory().unwrap_or_default(),
            vcpu: (if self.is_running(dom) { dom.get_max_vcpus().unwrap_or_default() } else { 0 }),
            autostart: dom.get_autostart().unwrap_or_default(),
            os_type: dom.get_os_type().unwrap_or_default(),
        }
    }

    fn is_running(&self, dom: &Domain) -> bool {
        match dom.get_state() {
            Err(_) => false,
            Ok(v) => v.0 &  virt::domain::VIR_DOMAIN_SHUTOFF == 1
        }
    }
}
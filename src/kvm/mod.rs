mod json;
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

    pub fn get_vm(&self) {
        //TODO: implement me
    }

    fn create_vm_info(&self, dom: &Domain) -> json::VirtualMachines {
        json::VirtualMachines{
            name: dom.get_name().unwrap_or(String::from("no-name")),
            state: dom.get_state().unwrap_or_default(),
            memory: dom.get_max_memory().unwrap_or_default(),
            vcpu: dom.get_max_vcpus().unwrap_or_default(),
        }
    }
}
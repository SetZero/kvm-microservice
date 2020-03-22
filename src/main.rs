use tide::{Request, Response};
mod kvm;

pub trait RequestExt {
    fn kvm_info(&self) -> String;
    fn vm_instances(&self) -> Response;
    fn start_vm(&self) -> String;
    fn stop_vm(&self) -> String;
    fn dummy(&self) -> String;
}

impl<State> RequestExt for Request<State> {
    fn kvm_info(&self) -> String {
        let mut kvm = kvm::KVM::new("".to_string());
        kvm.show_hypervisor_info()
            .map_err(|e| format!("{}", e))
            .and_then(|m| serde_json::to_string(&m).map(|d| d).map_err(|e| format!("{}", e)))
            .unwrap_or_else(|e| format!("{}", e))
    }
    fn vm_instances(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        tide::Response::new(200).body_json(&kvm.get_domains().unwrap_or_default()).unwrap()
    }

    fn start_vm(&self) -> String {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        create_json(kvm.start_vm(name))
    }

    fn stop_vm(&self) -> String {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        create_json(kvm.stop_vm(name))
    }

    fn dummy(&self) -> String {
        format!("Not Implemented, {}", self.param("name").unwrap_or("".to_string()))
    }
}

fn create_json(result: Result<kvm::json::KVMInfo, String>) -> String {
    result
        .and_then(|m| serde_json::to_string(&m).map(|d| d).map_err(|e| format!("{}", e)))
        .unwrap_or_else(|m| serde_json::to_string(&kvm::json::KVMInfo{success: false, message: Some(m)}).unwrap_or_else(|e| format!("{}", e)) )
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();


    app.at("/")
        .get(|_| async move { "Hello, world!" });
    app.at("/info")
        .get(|req: Request<()>| async move { req.kvm_info() });
    app.at("/instances")
        .get(|req: Request<()>| async move { req.vm_instances() });
    app.at("/instance/:name")
        .get(|req: Request<()>| async move { req.dummy() })
        .put(|req: Request<()>| async move { req.dummy() })
        .delete(|req: Request<()>| async move { req.dummy() })
        .post(|req: Request<()>| async move { req.dummy() });
    app.at("/instance/:name/network/:network")
        .post(|req: Request<()>| async move { req.dummy() });
    app.at("/instance/:name/start")
        .get(|req: Request<()>| async move { req.start_vm() });

    //app.at("/instance/:name/stop")
    //    .get(|req: Request<()>| async move { req.stop_vm(); });

    app.listen("192.168.1.101:8080").await?;
    Ok(())
}
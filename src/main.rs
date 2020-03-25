use tide::{Request, Response};
mod kvm;

pub trait RequestExt {
    fn kvm_info(&self) -> Response;
    fn vm_instances(&self) -> Response;
    fn get_devices(&self) -> Response;
    fn get_snapshots(&self) -> Response;
    fn get_console_out(&self) -> Response;
    fn start_vm(&self) -> Response;
    fn stop_vm(&self) -> Response;
    fn suspend_vm(&self) -> Response;
    fn resume_vm(&self) -> Response;
    fn dummy(&self) -> String;
}

impl<State> RequestExt for Request<State> {
    fn kvm_info(&self) -> Response {
        let mut kvm = kvm::KVM::new("".to_string());
        tide::Response::new(200).body_json(&kvm.show_hypervisor_info().map_err(|e| format!("{}", e)).unwrap()).unwrap()
    }
    fn vm_instances(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        tide::Response::new(200).body_json(&kvm.get_domains().unwrap_or_default()).unwrap()
    }

    fn get_devices(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        tide::Response::new(200).body_json(&kvm.get_devices(name).unwrap_or_default()).unwrap()
    }

    fn get_snapshots(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        tide::Response::new(200).body_json(&kvm.list_snapshots(name).unwrap_or_default()).unwrap()
    }

    fn get_console_out(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        tide::Response::new(200).body_json(&kvm.get_console_out(name).unwrap_or_default()).unwrap()
    }

    fn start_vm(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        create_json(kvm.start_vm(name))
    }

    fn stop_vm(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        create_json(kvm.stop_vm(name))
    }

    fn suspend_vm(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        create_json(kvm.suspend(name))
    }

    fn resume_vm(&self) -> Response {
        let kvm = kvm::KVM::new("".to_string());
        let name = self.param("name").unwrap_or("".to_string());

        create_json(kvm.resume(name))
    }

    fn dummy(&self) -> String {
        format!("Not Implemented, {}", self.param("name").unwrap_or("".to_string()))
    }
}

fn create_json(result: Result<kvm::json::KVMInfo, String>) -> Response {
    tide::Response::new(200).body_json(&result
        .map_err(|m| kvm::json::KVMInfo{success: false, message: Some(m)} )
        .unwrap_or_else(|e| e)
    ).unwrap()
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
    app.at("/instance/:name/stop")
        .get(|req: Request<()>| async move { req.stop_vm() });
    app.at("/instance/:name/suspend")
        .get(|req: Request<()>| async move { req.suspend_vm() });
    app.at("/instance/:name/resume")
        .get(|req: Request<()>| async move { req.resume_vm() });
    app.at("/instance/:name/devices")
        .get(|req: Request<()>| async move { req.get_devices() });
    app.at("/instance/:name/snapshots")
        .get(|req: Request<()>| async move { req.get_snapshots() });
    app.at("/instance/:name/console")
        .get(|req: Request<()>| async move { req.get_console_out() });

    app.listen("192.168.1.101:8080").await?;
    Ok(())
}
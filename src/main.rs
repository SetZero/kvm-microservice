use tide::{Request};
mod kvm;

pub trait RequestExt {
    fn kvm_info(&self) -> String;
    fn vm_instances(&self) -> String;
}

impl<State> RequestExt for Request<State> {
    fn kvm_info(&self) -> String {
        let mut kvm = kvm::KVM::new("".to_string());
        kvm.show_hypervisor_info()
            .map_err(|e| format!("{}", e))
            .and_then(|m| serde_json::to_string(&m).map(|d| d).map_err(|e| format!("{}", e)))
            .unwrap_or_default()
            //{ Ok(m) => m, Err(e) => format!("Error: {}", e.to_string())}
    }
    fn vm_instances(&self) -> String {
        let kvm = kvm::KVM::new("".to_string());

        kvm.get_domains()
            .map_err(|e| format!("{}", e))
            .and_then(|m| serde_json::to_string(&m).map(|d| d).map_err(|e| format!("{}", e)))
            .unwrap_or_default()
    }
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

    app.listen("192.168.178.80:8080").await?;
    Ok(())
}
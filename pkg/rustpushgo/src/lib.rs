use std::sync::Arc;

use tokio::{runtime::Runtime, sync::RwLock, task::JoinHandle};
use icloud_auth::{AnisetteConfiguration, AppleAccount};

#[uniffi::export]
pub fn add(left: u32, right: u32) -> u32 {
    println!("Adding {} and {}", left, right);
    left + right
}

#[uniffi::export]
pub async fn init() -> Arc<WrappedRuntime> {
    // // Start a Tokio runtime
    // let rt = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .enable_all()
    //     .build()
    //     .unwrap();

    // let _guard = rt.enter();

    Arc::new(WrappedRuntime {
        rt: vec![Runtime::new().unwrap()]
    })
}

#[derive(uniffi::Object)] 
pub struct WrappedRuntime {
    rt: Vec<Runtime>
}

#[uniffi::export]
pub async fn login(username: String, password: String, rt: &WrappedRuntime) {
    let join: JoinHandle<_> = rt.rt[0].spawn(async {
        // Just sleep async as a test
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    });

    // Wait for the async task to finish
    join.await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

uniffi::setup_scaffolding!();
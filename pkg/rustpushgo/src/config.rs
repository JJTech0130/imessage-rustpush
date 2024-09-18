use std::sync::Arc;

use rustpush::RelayConfig;
use uuid::Uuid;

use crate::ctx::runtime;

#[derive(uniffi::Object)]
pub struct WrappedRelayConfig {
    pub inner: Arc<RelayConfig>,
}

#[uniffi::export]
pub async fn create_relay_config(code: String) -> Arc<WrappedRelayConfig> {
    let config = runtime().spawn(async move {
        let host = "https://registration-relay.beeper.com".to_string();
        let token = Some("5c175851953ecaf5209185d897591badb6c3e712".to_string());
        let config: Arc<RelayConfig> = Arc::new(RelayConfig {
            version: RelayConfig::get_versions(&host, &code, &token).await.unwrap(),
            icloud_ua: "com.apple.iCloudHelper/282 CFNetwork/1408.0.4 Darwin/22.5.0".to_string(),
            aoskit_version: "com.apple.AOSKit/282 (com.apple.accountsd/113)".to_string(),
            dev_uuid: Uuid::new_v4().to_string(),
            protocol_version: 1640,
            host,
            code,
            beeper_token: token,
        });
        config
    }).await.unwrap();
    
    Arc::new(WrappedRelayConfig { inner: config })
}
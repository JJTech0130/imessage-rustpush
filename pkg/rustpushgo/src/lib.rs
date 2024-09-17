
use icloud_auth::{AnisetteConfiguration, AppleAccount};

#[uniffi::export]
pub async fn login(username: String, password: String) {
    let config = AnisetteConfiguration::default();

    let rt = tokio::runtime::Runtime::new().unwrap();

    let result = rt.spawn(async move {
        let account = AppleAccount::login(|| {
            (username.clone(), password.clone())
        }, || {
            "123456".to_string()
        }, config).await.unwrap();
    }).await.unwrap();
}


uniffi::setup_scaffolding!();
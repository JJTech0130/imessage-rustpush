pub mod callback;
pub mod config;
pub mod ctx;
mod util;
pub mod wrappers;

use std::{path::PathBuf, str::FromStr, sync::Arc, time::Duration, vec};

use icloud_auth::{AnisetteConfiguration, AppleAccount};

use log::{debug, info};
use rustls::client;
use rustpush::{
    authenticate_apple, get_gsa_config, register, APSConnectionResource, IDSUserIdentity, IMClient,
};
use tokio::time::sleep;
use wrappers::{
    IDSUsersWithIdentityRecord, WrappedAPSConnection, WrappedAPSState, WrappedIDSUserIdentity,
    WrappedIDSUsers, WrappedOSConfig,
};

#[uniffi::export(async_runtime = "tokio")]
pub async fn connect(
    config: &WrappedOSConfig,
    state: &WrappedAPSState,
) -> Arc<WrappedAPSConnection> {
    let config = config.config.clone();
    let state = state.inner.clone();
    let (connection, error) = APSConnectionResource::new(config, state).await;
    if let Some(error) = error {
        panic!("{}", error);
    }

    Arc::new(WrappedAPSConnection { inner: connection })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn login(
    apple_id: String,
    password: String,
    config: &WrappedOSConfig,
    connection: &WrappedAPSConnection,
) -> IDSUsersWithIdentityRecord {
    let config = config.config.clone();
    let connection = connection.inner.clone();

    let user_trimmed = apple_id.trim().to_string();
    let pw_trimmed = password.trim().to_string();

    let user_two = user_trimmed.clone();
    let appleid_closure = move || (user_two.clone(), pw_trimmed.clone());
    // ask console for 2fa code, make sure it is only 6 digits, no extra characters
    let tfa_closure = || {
        println!("Enter 2FA code: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    let acc = AppleAccount::login(
        appleid_closure,
        tfa_closure,
        AnisetteConfiguration::new()
            .set_client_info(get_gsa_config(
                &*connection.state.read().await,
                config.as_ref(),
            ))
            .set_configuration_path(PathBuf::from_str("anisette_test").unwrap()),
    )
    .await;

    let account = acc.unwrap();
    let pet = account.get_pet().unwrap();
    let user = authenticate_apple(&user_trimmed, &pet, config.as_ref())
        .await
        .unwrap();

    let identity = IDSUserIdentity::new().unwrap();

    let mut users = vec![user];

    if users[0].registration.is_none() {
        info!("Registering new identity...");
        register(
            config.as_ref(),
            &*connection.state.read().await,
            &mut users,
            &identity,
        )
        .await
        .unwrap();
    }

    IDSUsersWithIdentityRecord {
        users: Arc::new(WrappedIDSUsers { inner: users }),
        identity: Arc::new(WrappedIDSUserIdentity { inner: identity }),
    }
}

#[derive(uniffi::Object)]
pub struct Client {
    pub client: Arc<IMClient>,
    receive_handle: tokio::task::JoinHandle<()>,
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn new_client(
    connection: &WrappedAPSConnection,
    users: &WrappedIDSUsers,
    identity: &WrappedIDSUserIdentity,
    config: &WrappedOSConfig,
    update_users_callback: Box<dyn callback::UpdateUsersCallback>,
) -> Arc<Client> {
    let connection_clone = connection.inner.clone();
    let users_clone = users.inner.clone();
    let identity_clone = identity.inner.clone();
    let config_clone = config.config.clone();

    let client = Arc::new(
        IMClient::new(
            connection_clone,
            users_clone,
            identity_clone,
            "id_cache.plist".into(),
            config_clone,
            Box::new(move |updated_keys| {
                update_users_callback.update_users(Arc::new(WrappedIDSUsers {
                    inner: updated_keys,
                }));
                debug!("Updated keys");
            }),
        )
        .await,
    );

    let client_clone = client.clone();

    let receive_handle = tokio::spawn(async move {
        loop {
            match client_clone.receive_wait().await {
                Ok(Some(msg)) => debug!("got message {:?}", msg.message.to_string()),
                Ok(None) => debug!("no message received"),
                Err(e) => debug!("error receiving message: {:?}", e),
            }
        }
    });

    Arc::new(Client {
        client: client,
        receive_handle: receive_handle,
    })
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub async fn reregister(self: Arc<Self>) {
        self.client.identity.refresh_now().await.unwrap();
    }

    pub async fn get_handles(self: Arc<Self>) -> Vec<String> {
        self.client.identity.get_handles().await
    }

    pub async fn validate_targets(
        self: Arc<Self>,
        targets: Vec<String>,
        handle: String,
    ) -> Vec<String> {
        self.client
            .identity
            .validate_targets(&targets, &handle)
            .await
            .unwrap()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        debug!("Dropping client");
        self.receive_handle.abort();
    }
}

//impl

// OLD ///////////////////////////////////////////////////////////////////////////// OLD /////////////////////////////////////////////////////////////////
/*
async fn test_main() {
    let data: String = match fs::read_to_string("config.plist").await {
        Ok(v) => v,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    let _ = fs::File::create("config.plist").await.expect("Unable to create file").write_all(b"{}");
                    "{}".to_string()
                }
                _ => {
                    error!("Unable to read file");
                    std::process::exit(1);
                }
            }
        }
    };



    // let config: Arc<MacOSConfig> = Arc::new(if let Ok(config) = plist::from_file("hwconfig.plist") {
    //     config
    // } else {
    // /    println!("Missing hardware config!");
    //     println!("The easiest way to get your hardware config is to extract it from validation data from a Mac.");
    //     println!("This validation data will not be used to authenticate, and therefore does not need to be recent or valid.");
    //     println!("If you need help obtaining validation data, please visit https://github.com/beeper/mac-registration-provider");
    //     println!("As long as the hardware identifiers are valid rustpush will work fine.");
    //     println!("Validation data will not be required for subsequent re-registrations.");
    //     // save hardware config
    //     print!("Validation data: ");
    //     std::io::stdout().flush().unwrap();
    //     let validation_data_b64 = read_input().await;

    //     let validation_data = general_purpose::STANDARD.decode(validation_data_b64.trim()).unwrap();
    //     let extracted = HardwareConfig::from_validation_data(&validation_data).unwrap();

    //     MacOSConfig {
    //         inner: extracted,
    //         version: "13.6.4".to_string(),
    //         protocol_version: 1660,
    //         device_id: Uuid::new_v4().to_string(),
    //         icloud_ua: "com.apple.iCloudHelper/282 CFNetwork/1408.0.4 Darwin/22.5.0".to_string(),
    //         aoskit_version: "com.apple.AOSKit/282 (com.apple.accountsd/113)".to_string(),
    //     }
    // });
    let host = "https://registration-relay.beeper.com".to_string();
    let code = "65RQ-GPX7-75AS-EA3A".to_string();
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
    fs::write("hwconfig.plist", plist_to_string(config.as_ref()).unwrap()).await.unwrap();

    let saved_state: Option<SavedState> = plist::from_reader_xml(Cursor::new(&data)).ok();

    let (connection, error) =
        APSConnectionResource::new(
            config.clone(),
            saved_state.as_ref().map(|state| state.push.clone()),
        )
        .await;


    if let Some(error) = error {
        panic!("{}", error);
    }
    let mut users = if let Some(state) = saved_state.as_ref() {
        state.users.clone()
    } else {
        print!("Username: ");
        std::io::stdout().flush().unwrap();
        let username = read_input().await;
        print!("Password: ");
        std::io::stdout().flush().unwrap();
        let password = read_input().await;

        let user_trimmed = username.trim().to_string();
        let pw_trimmed = password.trim().to_string();

        let user_two = user_trimmed.clone();
        let appleid_closure = move || (user_two.clone(), pw_trimmed.clone());
        // ask console for 2fa code, make sure it is only 6 digits, no extra characters
        let tfa_closure = || {
            println!("Enter 2FA code: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        };
        let acc = AppleAccount::login(appleid_closure, tfa_closure,AnisetteConfiguration::new()
            .set_client_info(get_gsa_config(&*connection.state.read().await, config.as_ref()))
            .set_configuration_path(PathBuf::from_str("anisette_test").unwrap())).await;

        let account = acc.unwrap();
        let pet = account.get_pet().unwrap();
        let user = authenticate_apple(&user_trimmed, &pet, config.as_ref()).await.unwrap();

        vec![user]
    };

    let identity = saved_state.as_ref().map(|state| state.identity.clone()).unwrap_or(IDSUserIdentity::new().unwrap());

    if users[0].registration.is_none() {
        info!("Registering new identity...");
        register(config.as_ref(), &*connection.state.read().await, &mut users, &identity).await.unwrap();
    }

    let mut state = SavedState {
        push: connection.state.read().await.clone(),
        identity: identity.clone(),
        users: users.clone()
    };
    fs::write("config.plist", plist_to_string(&state).unwrap()).await.unwrap();

    let client = IMClient::new(connection.clone(), users, identity, "id_cache.plist".into(), config, Box::new(move |updated_keys| {
        state.users = updated_keys;
        std::fs::write("config.plist", plist_to_string(&state).unwrap()).unwrap();
    })).await;
    let handle = client.identity.get_handles().await[0].clone();


    //sleep(Duration::from_millis(10000)).await;

    let mut filter_target = String::new();

    let mut read_task = tokio::spawn(read_input());

    print!(">> ");
    std::io::stdout().flush().unwrap();

    let mut received_msgs = vec![];

    loop {
        tokio::select! {
            msg = client.receive_wait() => {
                if msg.is_err() {
                    error!("Failed to receive {}", msg.err().unwrap());
                    continue;
                }
                if let Ok(Some(msg)) = msg {
                    if msg.has_payload() && !received_msgs.contains(&msg.id) {
                        received_msgs.push(msg.id.clone());
                        println!("{}", msg);
                        print!(">> ");
                        std::io::stdout().flush().unwrap();
                        if msg.send_delivered {
                            println!("sending delivered");
                            let mut msg2 = MessageInst::new(msg.conversation.unwrap(), &handle, Message::Delivered);
                            msg2.id = msg.id;
                            msg2.target = msg.target;
                            let _ = client.send(&mut msg2).await;
                        }
                    }
                }
            },
            input = &mut read_task => {
                let Ok(input) = input else {
                    read_task = tokio::spawn(read_input());
                    continue;
                };
                if input.trim() == "" {
                    print!(">> ");
                    std::io::stdout().flush().unwrap();
                    read_task = tokio::spawn(read_input());
                    continue;
                }
                if input.starts_with("filter ") {
                    filter_target = input.strip_prefix("filter ").unwrap().to_string().trim().to_string();
                    println!("Filtering to {}", filter_target);
                } else if input.trim() == "sms" {
                    let mut msg = MessageInst::new(ConversationData {
                        participants: vec![],
                        cv_name: None,
                        sender_guid: Some(Uuid::new_v4().to_string()),
                        after_guid: None,
                    }, &handle, Message::EnableSmsActivation(true));
                    client.send(&mut msg).await.unwrap();
                    println!("sms activated");
                } else {
                    if filter_target == "" {
                        println!("Usage: filter [target]");
                    } else {
                        let mut msg = MessageInst::new(ConversationData {
                            participants: vec![filter_target.clone()],
                            cv_name: None,
                            sender_guid: Some(Uuid::new_v4().to_string()),
                            after_guid: None,
                        }, &handle, Message::Message(NormalMessage::new(input.trim().to_string(), MessageType::IMessage)));
                        if let Err(err) = client.send(&mut msg).await {
                            error!("Error sending message {err}");
                        }
                    }
                }
                print!(">> ");
                std::io::stdout().flush().unwrap();
                read_task = tokio::spawn(read_input());
            },
        }
    }
}
*/

uniffi::setup_scaffolding!();

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_init() {
    //     init();
    // }

    // #[test]
    // fn test_login() {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     rt.block_on(login("test".to_string(), "test".to_string()));
    // }
}

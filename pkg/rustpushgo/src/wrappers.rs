use std::sync::Arc;

use rustpush::{APSConnection, APSState, IDSUser, IDSUserIdentity, OSConfig};

use crate::util::{plist_from_string, plist_to_string};

#[derive(uniffi::Object)]
pub struct WrappedAPSState {
    pub inner: Option<APSState>,
}

#[uniffi::export]
impl WrappedAPSState {
    #[uniffi::constructor]
    pub fn new(string: Option<String>) -> Arc<Self> {
        Arc::new(Self {
            inner: plist_from_string(&string.unwrap_or("".to_string())).ok(),
        })
    }

    pub fn to_string(&self) -> String {
        plist_to_string(&self.inner).unwrap()
    }
}

#[derive(uniffi::Object)]
pub struct WrappedAPSConnection {
    pub inner: APSConnection,
}

#[uniffi::export]
impl WrappedAPSConnection {
    pub fn state(&self) -> Arc<WrappedAPSState> {
        Arc::new(WrappedAPSState {
            inner: Some(self.inner.state.blocking_read().clone()),
        })
    }
}

#[derive(uniffi::Record)]
pub struct IDSUsersWithIdentityRecord {
    pub users: Arc<WrappedIDSUsers>,
    pub identity: Arc<WrappedIDSUserIdentity>,
}

#[derive(uniffi::Object)]
pub struct WrappedIDSUsers {
    pub inner: Vec<IDSUser>,
}

#[uniffi::export]
impl WrappedIDSUsers {
    #[uniffi::constructor]
    pub fn new(string: Option<String>) -> Arc<Self> {
        Arc::new(Self {
            inner: plist_from_string(&string.unwrap_or("".to_string())).unwrap(),
        })
    }
    pub fn to_string(&self) -> String {
        plist_to_string(&self.inner).unwrap()
    }
}

#[derive(uniffi::Object)]
pub struct WrappedIDSUserIdentity {
    pub inner: IDSUserIdentity,
}

#[uniffi::export]
impl WrappedIDSUserIdentity {
    #[uniffi::constructor]
    pub fn new(string: Option<String>) -> Arc<Self> {
        Arc::new(Self {
            inner: plist_from_string(&string.unwrap_or("".to_string())).unwrap(),
        })
    }
    pub fn to_string(&self) -> String {
        plist_to_string(&self.inner).unwrap()
    }
}

#[derive(uniffi::Object)]
pub struct WrappedOSConfig {
    pub config: Arc<dyn OSConfig>,
}

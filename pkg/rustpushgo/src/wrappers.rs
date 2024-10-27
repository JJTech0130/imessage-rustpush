use std::sync::Arc;

use rustpush::{APSConnection, APSState, IDSUser, IDSUserIdentity, OSConfig};
use serde::Serialize;

use crate::util::{plist_from_string, plist_to_string};

#[derive(uniffi::Object)]
pub struct WrappedAPSState {
    pub inner: Option<APSState>,
}

#[uniffi::export]
impl WrappedAPSState {
    #[uniffi::constructor]
    pub fn new(s: String) -> Arc<Self> {
        Arc::new(Self {
            inner: plist_from_string(&s).ok(),
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

#[derive(uniffi::Object)]
pub struct WrappedIDSUsersWithIdentity {
    pub users: Vec<IDSUser>,
    pub identity: IDSUserIdentity,
}

#[derive(uniffi::Object, Clone, Serialize)]
pub struct WrappedIDSUsers {
    pub inner: Vec<IDSUser>,
}

#[derive(uniffi::Object)]
pub struct WrappedOSConfig {
    pub config: Arc<dyn OSConfig>,
}

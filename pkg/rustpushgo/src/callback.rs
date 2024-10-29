use std::sync::Arc;

use crate::wrappers::WrappedIDSUsers;

#[uniffi::export(callback_interface)]
pub trait UpdateUsersCallback: Send + Sync {
    fn update_users(&self, users: Arc<WrappedIDSUsers>) -> ();
}

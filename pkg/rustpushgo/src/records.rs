use rustpush::{APSState, IDSUser, IDSUserIdentity};

use crate::util::{plist_from_string, plist_to_string};

#[derive(uniffi::Record)]
pub struct APSStateRecord {
    pub token: Option<Vec<u8>>,
    /// Opaque plist because it is a private type in rustpush
    pub keypair: Option<String>, 
}

impl From<&APSState> for APSStateRecord {
    fn from(push: &APSState) -> Self {
        APSStateRecord {
            token: push.token.as_ref().map(|t| t.to_vec()),
            keypair: plist_to_string(&push.keypair).ok(),
        }
    }
}

impl Into<APSState> for APSStateRecord {
    fn into(self) -> APSState {
        APSState {
            token: self.token.as_ref().map(|t| {
                let mut array = [0u8; 32];
                array.copy_from_slice(&t[..32]);
                array
            }),
            keypair: plist_from_string(&self.keypair.unwrap()).unwrap(),
        }
    }
}

pub struct IDSRegistrationRecord {

}

pub struct IDSUserRecord {
    pub auth_keypair: String,
    pub user_id: String,
    pub registration: Option<IDSRegistration>,
    pub user_type: IDSUserType,
    pub protocol_version: u32,
}

#[derive(uniffi::Record)]
pub struct SavedStateRecord {
    pub push: APSStateRecord,
    pub users: Vec<IDSUser>,
    pub identity: IDSUserIdentity,
}
// This should actually be an Opaque record, that is, we just want to send Strings of plists to the Go side
// (have a helper on the rust side that consumes the normal equivalent of this record and returns a plist string)
// (have a helper on the Go side that just takes and stores strings)
// (have a helper on the Rust side that takes this and outputs normal equivalent)


/// Actually throw out opaque, just use into and from
/// 
/// 
#[derive(uniffi::Record)]
pub struct OpaqueSavedStateRecord {
    push: APSStateRecord,
    //push: String,
    users: String,
    identity: String,
    //users: Vec<IDSUser>,
    //identity: IDSUserIdentity,
}



// impl OpaqueSavedStateRecord {
//     pub fn new(push: &APSState, users: &[IDSUser]) -> Self {
//         Self {
//             push: plist_to_string(push).unwrap(),
//             users: plist_to_string(users).unwrap(),
//             identity: String::new(),
//         }
//     }
// }

// impl SavedStateRecord {
//     pub fn new(push: &APSState) -> Self {
//         Self {
//             push: APSStateRecord {
//                 token: push.token.as_ref().map(|t| t.into()),
//                 keypair: push.keypair.as_ref().map(|kp| KeyPairRecord {
//                     cert: kp.cert.clone(),
//                     private: kp.private.clone(),
//                 }),
//             },
//             //users: Vec::new(),
//             //identity: IDSUserIdentity::new(),
//         }
//     }
// }

// #[uniffi::export]
// impl SavedStateRecord {
//     // #[uniffi::constructor]
//     // pub fn new() -> Self {
//     //     SavedStateRecord {
//     //         push: APSStateRecord {
//     //             token: None,
//     //             keypair: None,
//     //         },
//     //         //users: Vec::new(),
//     //         //identity: IDSUserIdentity::new(),
//     //     }
//     // }

//     // fn from_push_state(&self, push: APSState) -> Self {
//     //     SavedStateRecord {
//     //         push: APSStateRecord {
//     //             token: push.get_token(),
//     //             keypair: push.get_keypair(),
//     //         },
//     //         //users: Vec::new(),
//     //         //identity: IDSUserIdentity::new(),
//     //     }
//     // }

//     // pub fn set_push_token(&mut self, token: Vec<u8>) {
//     //     self.push.token = Some(token);
//     // }

//     // pub fn set_push_keypair(&mut self, keypair: KeyPairRecord) {
//     //     self.push.keypair = Some(keypair);
//     // }

//     // pub fn get_push_token(&self) -> Option<Vec<u8>> {
//     //     self.push.token.clone()
//     // }

//     // pub fn get_push_keypair(&self) -> Option<KeyPairRecord> {
//     //     self.push.keypair.clone()
//     // }
// }


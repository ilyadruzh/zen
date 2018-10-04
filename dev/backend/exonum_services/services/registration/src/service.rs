use cmd::RoleSystemServiceConfig;
use exonum::blockchain::{ApiContext, Service, Transaction, TransactionSet};
use exonum::encoding::serialize::FromHex;
use exonum::{
    crypto::Hash, crypto::PublicKey, encoding, messages::RawTransaction, storage::Fork,
    storage::Snapshot,
};
use iron::Handler;
use router::Router;
use schema::RoleSystemSchema;
use serde_json;
use serde_json::value::Value;
use transactions::RoleSystemTransactions;

pub const SERVICE_ID: u16 = 101;

pub struct RoleSystemService {
    // Public key of the superuser.
// pub superuser_pkey: String,
}

impl RoleSystemService {
    /// Constructor of the RoleSystem service with superuser public key.
    pub fn new() -> RoleSystemService {
        RoleSystemService
    }

    // backlog - Checks that `pkey` is activated.
    // pub fn is_activated(&self, pkey: &PublicKey) -> bool {
    //     if let Ok(pk) = PublicKey::from_hex(self.superuser_pkey.clone()) {
    //         return *pkey == pk
    //     }
    //     false
    // }

    pub fn is_activated() -> bool {
        if let Ok(pk) = PublicKey::from_hex(self.superuser_pkey.clone()) {
            return *pkey == pk;
        }
        false
    }
}

impl Service for RoleSystemService {
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn service_name(&self) -> &'static str {
        "rolesystem"
    }

    /// Hashes for the service tables that will be included into the state hash.
    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        let schema = RoleSystemSchema::new(snapshot);
        schema.state_hash()
    }

    /// Implement a method to deserialize transactions coming to the node.
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        RoleSystemTransactions::tx_from_raw(raw).map(Into::into)
    }

    /// Implements an initialization routines for the service.
    /// Stores a superuser public key into the storage.
    // fn initialize(&self, fork: &mut Fork) -> Value {
    //     EmployeesSchema::new(fork).add_superuser_key(&self.superuser_pkey);
    //     serde_json::to_value(EmployeesServiceConfig { superuser_pkey: self.superuser_pkey.clone() }).unwrap()
    // }

    /// Create a REST `Handler` to process web requests to the node.
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();

        use api;
        use exonum::api::Api;

        let api = api::RoleSystemApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
        };

        api.wire(&mut router);
        Some(Box::new(router))
    }
}

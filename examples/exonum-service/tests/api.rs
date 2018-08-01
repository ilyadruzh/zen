// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! These are tests concerning the API of the employees service. See `tx_logic.rs`
//! for tests focused on the business logic of transactions.
//!
//! Note how API tests predominantly use `TestKitApi` to send transactions and make assertions
//! about the storage state.

extern crate assert_matches;
extern crate exonum;
extern crate exonum_employees as employees;
extern crate exonum_testkit;
#[macro_use]
extern crate serde_json;

// Import data types used in tests from the crate where the service is defined.
use employees::schema::Account;
use employees::schema::EmployeeId;
use employees::service::EmployeesService;
use employees::transactions::{TxCreateAccount, TxDeleteAccount, TxEditAccount, TxSetCustomData};
use exonum::crypto::{self, CryptoHash, Hash, PublicKey, SecretKey};
use exonum::encoding::serialize::FromHex;
use exonum::helpers::Height;
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};

/// Superuser public key
pub const SUPERUSER_PKEY: &str = "77471db2db2e2f0c1bde8cc222f790c707413447cebead035cde49a44a417188";

/// Superuser private key. Used for signing off a transactions done by a superuser.
pub const SUPERUSER_KEY: &str = "c79237c63ddb93f478e2b4dcc003c2241daa7c87277c308b1d1689a2cd928e1077471db2db2e2f0c1bde8cc222f790c707413447cebead035cde49a44a417188";

#[test]
fn test_create_account() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 1010;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let (mut testkit, api) = create_testkit();

    // Create and send a transaction via API
    let (tx, _) = api.create_account(&superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user indeed is persisted by the service.
    let account = api.get_account(tx.pub_key());
    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), tx.first_name());
    assert_eq!(account.last_name(), tx.last_name());
    assert_eq!(account.id_number(), tx.id_number());
}

#[test]
fn test_edit_account_superuser() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: EmployeeId = 42;

    const NEW_FIRST_NAME: &str = "Ivan";
    const NEW_LAST_NAME: &str = "Pupkin";
    const NEW_ID_NUMBER: u64 = 32;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let (mut testkit, api) = create_testkit();

    // Create test blocks
    testkit.create_blocks_until(Height(3));

    // Create and send a transaction via API
    let (tx, _) = api.create_account(&superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user indeed is persisted by the service.
    let account = api.get_account(tx.pub_key());
    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), tx.first_name());
    assert_eq!(account.last_name(), tx.last_name());
    assert_eq!(account.id_number(), tx.id_number());

    // Edit account by superuser
    let tx = api.edit_account(&superuser_pkey, &account.pub_key(), &superuser_key, NEW_FIRST_NAME, NEW_LAST_NAME, NEW_ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user info is changed
    let account = api.get_account(&account.pub_key());

    assert_eq!(account.first_name(), NEW_FIRST_NAME);
    assert_eq!(account.last_name(), NEW_LAST_NAME);
    assert_eq!(account.id_number(), NEW_ID_NUMBER);
}

#[test]
fn test_edit_account_self() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: EmployeeId = 42;

    const NEW_FIRST_NAME: &str = "Ivan";
    const NEW_LAST_NAME: &str = "Pupkin";
    const NEW_ID_NUMBER: u64 = 32;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let (mut testkit, api) = create_testkit();

    // Create and send a transaction via API
    let (tx, skey) = api.create_account(&superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user indeed is persisted by the service.
    let account = api.get_account(tx.pub_key());
    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), tx.first_name());
    assert_eq!(account.last_name(), tx.last_name());
    assert_eq!(account.id_number(), tx.id_number());

    // Edit account by superuser
    let tx = api.edit_account(&account.pub_key(), &account.pub_key(), &skey, NEW_FIRST_NAME, NEW_LAST_NAME, NEW_ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user info is changed
    let account = api.get_account(&account.pub_key());

    assert_eq!(account.first_name(), NEW_FIRST_NAME);
    assert_eq!(account.last_name(), NEW_LAST_NAME);
    assert_eq!(account.id_number(), NEW_ID_NUMBER);
}

#[test]
fn test_set_custom_data_superuser() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: EmployeeId = 42;
    const CUSTOM_DATA: &str = "test";

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let (mut testkit, api) = create_testkit();

    // Create test blocks
    testkit.create_blocks_until(Height(3));

    // Create and send a transaction via API
    let (tx, _) = api.create_account(&superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user indeed is persisted by the service.
    let account = api.get_account(tx.pub_key());
    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), tx.first_name());
    assert_eq!(account.last_name(), tx.last_name());
    assert_eq!(account.id_number(), tx.id_number());

    // Set custom data by a superuser
    let tx = api.set_custom_data(&superuser_pkey, &account.pub_key(), &superuser_key, CUSTOM_DATA);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the custom data is changed
    let account = api.get_account(&account.pub_key());

    assert_eq!(account.custom_data(), CUSTOM_DATA);
}

#[test]
fn test_set_custom_data_self() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: EmployeeId = 42;
    const CUSTOM_DATA: &str = "test";

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let (mut testkit, api) = create_testkit();

    // Create test blocks
    testkit.create_blocks_until(Height(3));

    // Create and send a transaction via API
    let (tx, skey) = api.create_account(&superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user indeed is persisted by the service.
    let account = api.get_account(tx.pub_key());
    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), tx.first_name());
    assert_eq!(account.last_name(), tx.last_name());
    assert_eq!(account.id_number(), tx.id_number());

    // Set custom data by a superuser
    let tx = api.set_custom_data(&account.pub_key(), &account.pub_key(), &skey, CUSTOM_DATA);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the custom data is changed
    let account = api.get_account(&account.pub_key());

    assert_eq!(account.custom_data(), CUSTOM_DATA);
}

#[test]
#[should_panic(expected = "Unexpected response status: NotFound")]
fn test_delete_account_superuser() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: EmployeeId = 42;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let (mut testkit, api) = create_testkit();

    // Create and send a transaction via API
    let (tx, _) = api.create_account(&superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    let pub_key = tx.pub_key();

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Delete account by a superuser
    let tx = api.delete_account(&superuser_pkey, &pub_key, &superuser_key);
    testkit.create_block();

    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the custom data is changed
    let account = api.get_account(&pub_key);

    assert_eq!(account.pub_key(), pub_key);
}

#[test]
fn test_get_block_by_id() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: EmployeeId = 42;

    const NEW_FIRST_NAME: &str = "Ivan";
    const NEW_LAST_NAME: &str = "Pupkin";
    const NEW_ID_NUMBER: u64 = 32;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let (mut testkit, api) = create_testkit();

    // Create test blocks
    testkit.create_blocks_until(Height(3));

    // Create and send a transaction via API
    let (tx, _) = api.create_account(&superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user indeed is persisted by the service.
    let account = api.get_account(tx.pub_key());
    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), tx.first_name());
    assert_eq!(account.last_name(), tx.last_name());
    assert_eq!(account.id_number(), tx.id_number());

    // Edit account by superuser
    let tx = api.edit_account(&superuser_pkey, &account.pub_key(), &superuser_key, NEW_FIRST_NAME, NEW_LAST_NAME, NEW_ID_NUMBER);

    testkit.create_block();
    api.assert_tx_status(&tx.hash(), &json!({ "type": "success" }));

    // Check that the user info is changed
    let account = api.get_account(&account.pub_key());

    assert_eq!(account.first_name(), NEW_FIRST_NAME);
    assert_eq!(account.last_name(), NEW_LAST_NAME);
    assert_eq!(account.id_number(), NEW_ID_NUMBER);

    // Create another test blocks
    for _ in 0..10 {
        testkit.create_block();
    }

    // Get the last block number that modifies a specific account
    let res = api.get_block_by_id(&ID_NUMBER);

    if let Some(height) = res {
        assert_eq!(height, Height(5));

        return
    }

    unreachable!()
}

/// Wrapper for the employees service API allowing to easily use it
/// (compared to `TestKitApi` calls).
struct EmployeesApi {
    pub inner: TestKitApi,
}

impl EmployeesApi {
    /// Generates an account creation transaction with a random key pair, sends it over HTTP,
    /// and checks the synchronous result (i.e., the hash of the transaction returned
    /// within the response).
    fn create_account(&self, pkey_by_who: &PublicKey, key_by_who: &SecretKey, first_name: &str, last_name: &str, id_number: u64) -> (TxCreateAccount, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();

        let tx = TxCreateAccount::new(pkey_by_who, &pubkey, first_name, last_name, id_number, &key_by_who);

        let tx_info: serde_json::Value =
            self.inner
                .post(ApiKind::Service("employees"), "v1/accounts/transaction", &tx);

        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));

        (tx, key)
    }

    /// Gets the state of a particular account using an HTTP request.
    fn get_account(&self, pubkey: &PublicKey) -> Account {
        self.inner.get(
            ApiKind::Service("employees"),
            &format!("v1/accounts/{}", pubkey.to_string()),
        )
    }

    fn edit_account(&self, pkey_by_who: &PublicKey, pkey: &PublicKey, key_by_who: &SecretKey, first_name: &str, last_name: &str, id_number: u64) -> TxEditAccount {
        let tx = TxEditAccount::new(pkey_by_who, &pkey, first_name, last_name, id_number, /* seed */ 0, &key_by_who);

        let tx_info: serde_json::Value =
            self.inner
                .post(ApiKind::Service("employees"), "v1/accounts/transaction", &tx);

        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));

        tx
    }

    fn set_custom_data(&self, pkey_by_who: &PublicKey, pkey: &PublicKey, key_by_who: &SecretKey, custom_data: &str) -> TxSetCustomData {
        let tx = TxSetCustomData::new(pkey_by_who, &pkey, custom_data, /* seed */ 0, &key_by_who);

        let tx_info: serde_json::Value =
            self.inner
                .post(ApiKind::Service("employees"), "v1/accounts/transaction", &tx);

        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));

        tx
    }

    fn delete_account(&self, pkey_by_who: &PublicKey, pkey: &PublicKey, key_by_who: &SecretKey) -> TxDeleteAccount {
        let tx = TxDeleteAccount::new(pkey_by_who, &pkey, /* seed */ 0, &key_by_who);

        let tx_info: serde_json::Value =
            self.inner
                .post(ApiKind::Service("employees"), "v1/accounts/transaction", &tx);

        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));

        tx
    }

    fn get_block_by_id(&self, id: &EmployeeId) -> Option<Height> {

        let res = self.inner.get(
            ApiKind::Service("employees"),
            &format!("/v1/blocks/{}", id.to_string()),
        );

        res
    }

    /// Asserts that the transaction with the given hash has a specified status.
    fn assert_tx_status(&self, tx_hash: &Hash, expected_status: &serde_json::Value) {
        let info: serde_json::Value = self.inner.get(
            ApiKind::Explorer,
            &format!("v1/transactions/{}", tx_hash.to_string()),
        );
        if let serde_json::Value::Object(mut info) = info {
            let tx_status = info.remove("status").unwrap();
            assert_eq!(tx_status, *expected_status);
        } else {
            panic!("Invalid transaction info format, object expected");
        }
    }
}

/// Creates a testkit together with the API wrapper defined above.
fn create_testkit() -> (TestKit, EmployeesApi) {
    let testkit = TestKitBuilder::validator()
        .with_service(EmployeesService::new(SUPERUSER_PKEY.to_string()))
        .create();

    let api = EmployeesApi {
        inner: testkit.api(),
    };

    (testkit, api)
}

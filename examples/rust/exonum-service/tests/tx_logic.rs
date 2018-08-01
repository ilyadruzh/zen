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

//! These are tests concerning the business logic of transactions in the employees service.
//! See `api.rs` for tests focused on the API of the service.
//!
//! Note how business logic tests use `TestKit::create_block*` methods to send transactions,
//! the service schema to make assertions about the storage state.

extern crate exonum;
extern crate exonum_employees as employees;
#[macro_use]
extern crate exonum_testkit;
extern crate rand;

// Import data types used in tests from the crate where the service is defined.
use employees::schema::{Account, EmployeesSchema};
use employees::service::EmployeesService;
use employees::transactions::{TxCreateAccount, TxDeleteAccount, TxEditAccount, TxSetCustomData};
use exonum::blockchain::Transaction;
use exonum::crypto::{self, PublicKey, SecretKey};
use exonum::encoding::serialize::FromHex;
use exonum_testkit::{TestKit, TestKitBuilder};

/// Superuser public key
pub const SUPERUSER_PKEY: &str = "77471db2db2e2f0c1bde8cc222f790c707413447cebead035cde49a44a417188";

/// Superuser private key. Used for signing off a transactions done by a superuser.
pub const SUPERUSER_KEY: &str = "c79237c63ddb93f478e2b4dcc003c2241daa7c87277c308b1d1689a2cd928e1077471db2db2e2f0c1bde8cc222f790c707413447cebead035cde49a44a417188";

#[test]
fn test_create_account() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();
    let (tx, _, _) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Check that the user indeed is persisted by the service
    let account = get_account(&testkit, tx.pub_key());

    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), FIRST_NAME);
    assert_eq!(account.last_name(), LAST_NAME);
    assert_eq!(account.id_number(), ID_NUMBER);
}

#[test]
fn test_create_account_invalid_superuser() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    // Deliberately invalid public key
    let superuser_pkey = PublicKey::from_hex("77471db2db2e2f0c1bde8cc222f790c707413447cebead035cde49a44a417180").unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();
    let (tx, _, _) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Check that account is not exist
    let account = try_get_account(&testkit, tx.pub_key());
    assert_eq!(account, None);
}

#[test]
fn test_edit_account_superuser() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    const NEW_FIRST_NAME: &str = "Ivan";
    const NEW_LAST_NAME: &str = "Pupkin";
    const NEW_ID_NUMBER: u64 = 32;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to edit
    let (tx, pkey, skey) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Check that the user indeed is persisted by the service
    let account = get_account(&testkit, tx.pub_key());

    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), FIRST_NAME);
    assert_eq!(account.last_name(), LAST_NAME);
    assert_eq!(account.id_number(), ID_NUMBER);

    // Edit account by superuser
    let tx = edit_account(&mut testkit, &superuser_pkey, &pkey, &superuser_key, NEW_FIRST_NAME, NEW_LAST_NAME, NEW_ID_NUMBER);

    // Check that the user info is changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_eq!(account.first_name(), NEW_FIRST_NAME);
    assert_eq!(account.last_name(), NEW_LAST_NAME);
    assert_eq!(account.id_number(), NEW_ID_NUMBER);
}

#[test]
fn test_edit_account_myself() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    const NEW_FIRST_NAME: &str = "Ivan";
    const NEW_LAST_NAME: &str = "Pupkin";
    const NEW_ID_NUMBER: u64 = 32;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to edit
    let (tx, pkey, skey) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Check that the user indeed is persisted by the service
    let account = get_account(&testkit, tx.pub_key());

    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), FIRST_NAME);
    assert_eq!(account.last_name(), LAST_NAME);
    assert_eq!(account.id_number(), ID_NUMBER);

    // Edit account by superuser
    let tx = edit_account(&mut testkit, &pkey, &pkey, &skey, NEW_FIRST_NAME, NEW_LAST_NAME, NEW_ID_NUMBER);

    // Check that the user info is changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_eq!(account.first_name(), NEW_FIRST_NAME);
    assert_eq!(account.last_name(), NEW_LAST_NAME);
    assert_eq!(account.id_number(), NEW_ID_NUMBER);
}

#[test]
fn test_edit_account_nonexistent() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    const NEW_FIRST_NAME: &str = "Ivan";
    const NEW_LAST_NAME: &str = "Pupkin";
    const NEW_ID_NUMBER: u64 = 32;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to edit
    let (tx, pkey, _) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Generate a "test" user key which will represent a nonexistent account
    let (pk_other, skey) = crypto::gen_keypair();

    // Edit account by a regular user (but account doesn't exist)
    let tx = edit_account(&mut testkit, &superuser_pkey, &pk_other, &superuser_key, NEW_FIRST_NAME, NEW_LAST_NAME, NEW_ID_NUMBER);

    // Check that the user info hasn't changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_ne!(account.first_name(), NEW_FIRST_NAME);
    assert_ne!(account.last_name(), NEW_LAST_NAME);
    assert_ne!(account.id_number(), NEW_ID_NUMBER);
}

#[test]
fn test_edit_account_reject() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    const NEW_FIRST_NAME: &str = "Ivan";
    const NEW_LAST_NAME: &str = "Pupkin";
    const NEW_ID_NUMBER: u64 = 32;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to edit
    let (tx, pkey, _) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Generate a "test" user key which isn't a superuser and will try to edit another account
    let (pk_other, skey) = crypto::gen_keypair();

    // Edit account by a regular user (which has no rights to do so)
    let tx = edit_account(&mut testkit, &pk_other, &pkey, &skey, NEW_FIRST_NAME, NEW_LAST_NAME, NEW_ID_NUMBER);

    // Check that the user info hasn't changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_ne!(account.first_name(), NEW_FIRST_NAME);
    assert_ne!(account.last_name(), NEW_LAST_NAME);
    assert_ne!(account.id_number(), NEW_ID_NUMBER);
}

#[test]
fn test_set_custom_data_superuser() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;
    const CUSTOM_DATA: &str = "test";

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to set custom data on
    let (tx, pkey, skey) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Check that the user indeed is persisted by the service
    let account = get_account(&testkit, tx.pub_key());

    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), FIRST_NAME);
    assert_eq!(account.last_name(), LAST_NAME);
    assert_eq!(account.id_number(), ID_NUMBER);

    // Set custom data by superuser
    let tx = set_custom_data(&mut testkit, &superuser_pkey, &pkey, &superuser_key, CUSTOM_DATA);

    // Check that the custom data is changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_eq!(account.custom_data(), CUSTOM_DATA)
}

#[test]
fn test_set_custom_data_myself() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;
    const CUSTOM_DATA: &str = "test";

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to set custom data on
    let (tx, pkey, skey) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Check that the user indeed is persisted by the service
    let account = get_account(&testkit, tx.pub_key());

    assert_eq!(account.pub_key(), tx.pub_key());
    assert_eq!(account.first_name(), FIRST_NAME);
    assert_eq!(account.last_name(), LAST_NAME);
    assert_eq!(account.id_number(), ID_NUMBER);

    // Edit account by superuser
    let tx = set_custom_data(&mut testkit, &pkey, &pkey, &skey, CUSTOM_DATA);

    // Check that the custom data is changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_eq!(account.custom_data(), CUSTOM_DATA)
}

#[test]
fn test_set_custom_data_nonexistent() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;
    const CUSTOM_DATA: &str = "test";

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to set custom data on
    let (tx, pkey, _) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Generate a "test" user key which will represent a nonexistent account
    let (pk_other, skey) = crypto::gen_keypair();

    // Edit account by a regular user (but account doesn't exist)
    let tx = set_custom_data(&mut testkit, &superuser_pkey, &pk_other, &superuser_key, CUSTOM_DATA);

    // Check that the custom data hasn't changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_ne!(account.custom_data(), CUSTOM_DATA)
}

#[test]
fn test_set_custom_data_reject() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;
    const CUSTOM_DATA: &str = "test";

    // Create an account to edit
    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();
    let (tx, pkey, _) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Generate a "test" user key which isn't a superuser and will try to edit another account
    let (pk_other, skey) = crypto::gen_keypair();

    // Edit account by a regular user (which has no rights to do so)
    let tx = set_custom_data(&mut testkit, &pk_other, &pkey, &skey, CUSTOM_DATA);

    // Check that the custom data hasn't changed
    let account = get_account(&testkit, &pkey);

    assert_eq!(account.pub_key(), &pkey);
    assert_ne!(account.custom_data(), CUSTOM_DATA)
}

#[test]
fn test_delete_account_superuser() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to delete
    let (tx, pkey, skey) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Delete an account by a superuser
    let tx = delete_account(&mut testkit, &superuser_pkey, &pkey, &superuser_key);

    // Check that account is deleted
    let account = try_get_account(&testkit, &pkey);
    assert_eq!(account, None);
}

#[test]
fn test_delete_account_superuser_nonexistent() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to delete
    let (tx, pkey, skey) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Generate a "test" user key which will represent a nonexistent account
    let (pk_other, skey) = crypto::gen_keypair();

    // Delete an account by a superuser
    let tx = delete_account(&mut testkit, &superuser_pkey, &pk_other, &superuser_key);

    // Check that account isn't deleted
    let account = try_get_account(&testkit, &pkey);
    assert!(account.is_some());
}

#[test]
fn test_delete_account_rejected() {
    const FIRST_NAME: &str = "John";
    const LAST_NAME: &str = "Smith";
    const ID_NUMBER: u64 = 42;

    let superuser_pkey = PublicKey::from_hex(SUPERUSER_PKEY).unwrap();
    let superuser_key = SecretKey::from_hex(SUPERUSER_KEY).unwrap();

    let mut testkit = init_testkit();

    // Create an account to delete
    let (tx, pkey, _) = create_account(&mut testkit, &superuser_pkey, &superuser_key, FIRST_NAME, LAST_NAME, ID_NUMBER);

    // Generate a "test" user key which isn't a superuser and will try to delete another account
    let (pk_other, skey) = crypto::gen_keypair();

    // Delete an account by a regular user
    let tx = delete_account(&mut testkit, &pk_other, &pkey, &skey);

    // Check that account isn't deleted
    let account = try_get_account(&testkit, &pkey);
    assert!(account.is_some());
}

/// Initializes testkit with `EmployeesService`.
fn init_testkit() -> TestKit {
    TestKitBuilder::validator()
        .with_service(EmployeesService::new(SUPERUSER_PKEY.to_string()))
        .create()
}

/// Creates a account with the given name and a random key.
fn create_account(testkit: &mut TestKit, pkey_by_who: &PublicKey, key_by_who: &SecretKey, first_name: &str, last_name: &str, id_number: u64) -> (TxCreateAccount, PublicKey, SecretKey) {
    let (pubkey, key) = crypto::gen_keypair();

    let tx = TxCreateAccount::new(pkey_by_who, &pubkey, first_name, last_name, id_number, &key_by_who);
    testkit.create_block_with_transaction(tx.clone());

    (tx, pubkey, key)
}

/// Changes account data by given public key.
fn edit_account(testkit: &mut TestKit, pkey_by_who: &PublicKey, pkey: &PublicKey, key_by_who: &SecretKey, first_name: &str, last_name: &str, id_number: u64) -> TxEditAccount {
    let tx = TxEditAccount::new(pkey_by_who, &pkey, first_name, last_name, id_number, /* seed */ 0, &key_by_who);
    testkit.create_block_with_transaction(tx.clone());

    tx
}

/// Sets account's custom data by given public key.
fn set_custom_data(testkit: &mut TestKit, pkey_by_who: &PublicKey, pkey: &PublicKey, key_by_who: &SecretKey, custom_data: &str) -> TxSetCustomData {
    let tx = TxSetCustomData::new(pkey_by_who, &pkey, custom_data, /* seed */ 0, &key_by_who);
    testkit.create_block_with_transaction(tx.clone());

    tx
}

/// Deletes account by a given public key.
fn delete_account(testkit: &mut TestKit, pkey_by_who: &PublicKey, pkey: &PublicKey, key_by_who: &SecretKey) -> TxDeleteAccount {
    let tx = TxDeleteAccount::new(pkey_by_who, &pkey, /* seed */ 0, &key_by_who);
    testkit.create_block_with_transaction(tx.clone());

    tx
}

/// Returns the account identified by the given public key or `None` such account doesn't exist.
fn try_get_account(testkit: &TestKit, pubkey: &PublicKey) -> Option<Account> {
    let snapshot = testkit.snapshot();

    EmployeesSchema::new(&snapshot).account(pubkey)
}

/// Returns the account identified by the given public key.
fn get_account(testkit: &TestKit, pubkey: &PublicKey) -> Account {
    try_get_account(testkit, pubkey).expect("No account persisted")
}
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

//! Employees API.

use bodyparser;
use exonum::api::{Api, ApiError};
use exonum::blockchain::*;
use exonum::crypto::{Hash, PublicKey};
use exonum::helpers::Height;
use exonum::node::TransactionSend;
use exonum::storage::Snapshot;
use iron::prelude::*;
use router::Router;
use schema::*;
use schema::EmployeeId;
use serde_json;
use std::fmt;
use transactions::EmployeesTransactions;

/// The structure returned by the REST API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Hash of the transaction.
    pub tx_hash: Hash,
}

/// Account custom data.
#[derive(Debug, Serialize)]
pub struct AccountInfo {
    custom_data: String,
}

/// Employees service API description.
#[derive(Clone)]
pub struct EmployeesApi<T: TransactionSend + Clone> {
    /// Exonum blockchain.
    pub blockchain: Blockchain,
    /// Channel for transactions.
    pub channel: T,
}

impl<T> EmployeesApi<T>
    where
        T: TransactionSend + Clone + 'static,
{
    /// Checks that given `Block` has transactions that did modified a specific `Account`
    fn helper_check_tx_modifies_account(&self, schema: &Schema<&Box<Snapshot>>, block: &Block, account: &Account) -> bool {
        let block_transactions = schema.block_transactions(block.height());

        let mut found = false;

        // Get transaction bytes by its hash and convert it into an actual `Transaction`
        for tx_hash in block_transactions.iter() {
            let transaction = schema.transactions().get(&tx_hash);

            if let Some(tx) = transaction {
                let tx =  EmployeesTransactions::tx_from_raw(tx);

                if let Ok(tx) = tx {
                    // Is there any of possible transactions that modifies a specific account?
                    found = match tx {
                        EmployeesTransactions::TxCreateAccount(tx) => {
                            tx.pub_key() == account.pub_key()
                        }

                        EmployeesTransactions::TxEditAccount(tx) => {
                            tx.pkey_account() == account.pub_key()
                        }

                        EmployeesTransactions::TxSetCustomData(tx) => {
                            tx.pkey_account() == account.pub_key()
                        }

                        EmployeesTransactions::TxDeleteAccount(tx) => {
                            tx.pkey_account() == account.pub_key()
                        }
                    };

                    if found {
                        break
                    }
                }
            }
        }

        found
    }

    /// Finds number (height) of the block which contains a latest transaction that modifies
    /// a given account.
    ///
    /// Complexity: `O(n)`, where: `n` is number of blocks in the chain, since lookup in the `Map`s are `O(1)`
    fn block_by_account(&self, account: &Account) -> Option<Height> {
        use std::cmp::max;

        let view = self.blockchain.snapshot();
        let schema = Schema::new(&view);
        let blocks = schema.blocks();

        // Look for a related blocks that contains transactions that are changing given account
        // and take the latest (with max. height)
        let mut max_height = None;

        blocks.iter()
            // Look for related blocks
            .filter(|&(_, ref block)| self.helper_check_tx_modifies_account(&schema, block, &account))

            // Find the latest one (with max. height)
            .for_each( | (_, block) | max_height = Some(max(block.height(), max_height.unwrap_or(Height(0)))));

        max_height
    }

    /// Same as `block_by_account` but firstly looks up for an account
    /// that is associated with the specific `EmployeeId`
    fn block_by_employee_id(&self, id: &EmployeeId) -> Option<Height> {
        let view = self.blockchain.snapshot();
        let employees_schema = EmployeesSchema::new(view);

        if let Some(account) = employees_schema.account_by_id(id) {
            return self.block_by_account(&account);
        }

        None
    }

    fn transaction(&self, req: &mut Request) -> IronResult<Response> {
        match req.get::<bodyparser::Struct<EmployeesTransactions>>() {
            Ok(Some(transaction)) => {
                let transaction: Box<Transaction> = transaction.into();
                let tx_hash = transaction.hash();
                self.channel.send(transaction).map_err(ApiError::from)?;
                let json = TransactionResponse { tx_hash };
                self.ok_response(&serde_json::to_value(&json).unwrap())
            }

            Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        }
    }

    fn account(&self, req: &mut Request) -> IronResult<Response> {
        let pub_key: PublicKey = self.url_fragment(req, "pubkey")?;
        let view = self.blockchain.snapshot();
        let schema = EmployeesSchema::new(view);
        if let Some(account) = schema.account(&pub_key) {
            self.ok_response(&serde_json::to_value(&account).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("Account not found").unwrap())
        }
    }

    fn block_by_id(&self, req: &mut Request) -> IronResult<Response> {
        let employee_id: EmployeeId = self.url_fragment(req, "id")?;

        if let Some(height) = self.block_by_employee_id(&employee_id) {
            self.ok_response(&serde_json::to_value(&height).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("Account not found").unwrap())
        }
    }

    /// Endpoint for dumping all accounts from the storage.
    fn accounts(&self, _: &mut Request) -> IronResult<Response> {
        let snapshot = self.blockchain.snapshot();
        let schema = EmployeesSchema::new(snapshot);
        let idx = schema.accounts();
        let accounts: Vec<Account> = idx.values().collect();

        self.ok_response(&serde_json::to_value(&accounts).unwrap())
    }
}

impl<T: TransactionSend + Clone> fmt::Debug for EmployeesApi<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmployeesApi {{}}")
    }
}

impl<T> Api for EmployeesApi<T>
    where
        T: 'static + TransactionSend + Clone,
{
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let get_accounts =move |req: &mut Request| self_.accounts(req);

        let self_ = self.clone();
        let get_block_by_id = move |req: &mut Request| self_.block_by_id(req);

        let self_ = self.clone();
        let get_account = move |req: &mut Request| self_.account(req);

        let self_ = self.clone();
        let post_transaction = move |req: &mut Request| self_.transaction(req);


        router.get("/v1/accounts", get_accounts, "get_accounts");
        router.post("/v1/accounts/transaction", post_transaction, "post_transaction");
        router.get("/v1/accounts/:pubkey", get_account, "account");
        router.get("/v1/blocks/:id", get_block_by_id, "block_by_id");
    }
}
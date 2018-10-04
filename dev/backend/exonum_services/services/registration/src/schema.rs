use exonum::crypto::{Hash, PublicKey};
use exonum::storage::{Fork, MapIndex, ProofMapIndex, Snapshot, ValueSetIndex};

// Employee ID type alias.
// pub type EmployeeId = u64;

// Declare the data to be stored in the blockchain, namely accounts with data.
encoding_struct! {
    /// Account struct used to persist data within the service.
    struct Account {
        pub_key: &PublicKey,

        /// UTF-8 string with the owner's username.
        username: &str,

        /// password_encrypted_by_secret_key
        encrypted_password: &str,

        /// user's photo stored in IPFS
        photo_ipfs: &Hash,

        is_active: &bool,

        /// Custom data.
        custom_data: &str,
    }
}

/// Additional methods for managing account in immutable fashion.
impl Account {
    // backlog - Returns a copy of this account with the changed info.
    pub fn set_photo(self, photo_ipfs: &Hash) -> Self {
        Self::new(
            self.pub_key(),
            self.username(),
            self.encrypted_password(),
            photo_ipfs,
            self.is_active(),
            self.custom_data(),
        )
    }

    /// Returns a copy of this account with changed custom data.
    pub fn set_custom_data(self, custom_data: &str) -> Self {
        Self::new(
            self.pub_key(),
            self.username(),
            self.encrypted_password(),
            self.photo_ipfs(),
            self.is_active(),
            custom_data,
        )
    }
}

/// Schema of the key-value storage used by the demo employees service.
pub struct RoleSystemSchema<T> {
    view: T,
}

/// Declare the layout of data managed by the service. An instance of [`MapIndex`] is used
/// to keep accounts in the storage. Index values are serialized [`Account`] structs.
impl<T: AsRef<Snapshot>> RoleSystemSchema<T> {
    /// Creates a new schema instance.
    pub fn new(view: T) -> Self {
        RoleSystemSchema { view }
    }

    /// Returns an immutable version of the accounts table.
    pub fn accounts(&self) -> ProofMapIndex<&Snapshot, PublicKey, Account> {
        ProofMapIndex::new("rolesystem.accounts", self.view.as_ref())
    }

    /// Gets a specific account from the storage.
    pub fn account(&self, pub_key: &PublicKey) -> Option<Account> {
        self.accounts().get(pub_key)
    }

    /// Method to get state hash.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.accounts().merkle_root()]
    }

    // pub fn is_activated(&self, pub_key: &PublicKey) -> bool {
    //     self.superuser_pk().contains(&pub_key.to_string())
    // }

    // backlog - Returns an immutable version of employee ID to account PK table.
    // fn employee_id_to_account_pk(&self) -> MapIndex<&Snapshot, EmployeeId, PublicKey> {
    //     MapIndex::new("employees.accounts_by_id", self.view.as_ref())
    // }

    // TODO:  Searches an account by given username
    // pub fn pub_key_by_username(&self, username: &str) -> Option<Account> {
    //     self.employee_id_to_account_pk()
    //         .get(id)
    //         .map_or(None, |pk| self.accounts().get(&pk))
    // }

    /// Returns an immutable version of a superuser public keys set.
    // pub fn superuser_pk(&self) -> ValueSetIndex<&Snapshot, String> {
    //     ValueSetIndex::new("employees.superuser_pks", self.view.as_ref())
    // }

    // Checks that specific public key is belongs to a superuser set.
    // pub fn is_superuser(&self, pub_key: &PublicKey) -> bool {
    //     self.superuser_pk().contains(&pub_key.to_string())
    // }
}

/// A mutable version of the schema with an additional method to persist accounts
/// to the storage.
impl<'a> RoleSystemSchema<&'a mut Fork> {
    /// Returns a mutable version of the accounts table.
    pub fn accounts_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Account> {
        ProofMapIndex::new("rolesystem.accounts", &mut self.view)
    }

    /// Creates a new account with specified credentials.
    pub fn account_create(
        &mut self,
        pub_key: &PublicKey,
        username: &str,
        encrypted_password: &str,
        photo_ipfs: &Hash,
        is_active: &bool,
        custom_data: &str,
    ) {
        let account = Account::new(pub_key, username, encrypted_password, photo_ipfs, is_active,custom_data,);

        // Save account and ID into the DB
        self.accounts_mut().put(&pub_key, account);
        // self.account_pk_by_employee_id().put(&id_number, *pub_key);
    }

    /// Changes account basic information.
    pub fn account_edit(
        &mut self,
        pub_key: &PublicKey,
        username: &str,
        encrypted_password: &str,
        photo_ipfs: &Hash,
        is_active: &bool,
        custom_data: &str,
    ) {
        let account = account.set_info(username, encrypted_password, photo_ipfs, is_active, custom_data);

        self.accounts_mut().put(pub_key, account);
    }

    /// Changes account basic information.
    pub fn account_set_custom_data(
        &mut self,
        account: Account,
        pub_key: &PublicKey,
        custom_data: &str,
    ) {
        let account = account.set_custom_data(custom_data);
        self.accounts_mut().put(pub_key, account);
    }

    // Changes account basic information.
    // pub fn account_delete(&mut self, id: &EmployeeId, pub_key: &PublicKey) {
    //     self.account_pk_by_employee_id().remove(id);
    //     self.accounts_mut().remove(pub_key);
    // }

    // Adds a new superuser public key.
    // pub fn add_superuser_key(&mut self, key: &str) {
    //     self.superusers_pk_mut().insert(key.to_string());
    // }

    /// Returns a mutable version of employee ID to account PK table.
    // pub fn account_pk_by_employee_id(&mut self) -> MapIndex<&mut Fork, EmployeeId, PublicKey> {
    //     MapIndex::new("rolesystem.accounts_by_id", &mut self.view)
    // }

    // fn superusers_pk_mut(&mut self) -> ValueSetIndex<&mut Fork, String> {
    //     ValueSetIndex::new("rolesystem.superuser_pks", &mut self.view)
    // }
}

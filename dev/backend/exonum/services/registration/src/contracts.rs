use exonum::blockchain::{ExecutionError, ExecutionResult, Transaction};
use exonum::{messages::Message, storage::Fork};
use num_traits::ToPrimitive;
use schema::*;
use transactions::{TxCreateAccount, TxDeleteAccount, TxEditAccount, TxSetCustomData};

impl Transaction for TxCreateAccount {
    /// Verifies integrity of the transaction by checking the transaction
    /// signatures.
    fn verify(&self) -> bool {
        self.verify_signature(self.pkey_by_who())
    }

    /// If a account with the specified public key is not registered and ID number is unique,
    /// then creates a new account, with the specified public key and name, last name, id number.
    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = RoleSystemSchema::new(view);

        // if !schema.is_superuser(&self.pkey_by_who()) {
        //     return Err(ErrorKind::NotASuperUser.into())
        // }

        if schema.account(self.pub_key()).is_some() {
            return Err(ErrorKind::AccountAlreadyExists.into());
        }

        // TODO - Запосить аккаунт по юзернейму
        if schema
            .account_pk_by_username()
            .get(&self.id_number())
            .is_some()
        {
            return Err(ErrorKind::EmployeeIdAlreadyExists.into());
        }

        schema.account_create(
            self.pub_key(),
            self.username(),
            self.encrypted_password(),
            self.photo_ipfs(),
            self.is_active(),
            self.custom_data,
        );

        Ok(())
    }
}

impl Transaction for TxEditAccount {
    /// Verifies integrity of the transaction by checking the transaction
    /// signatures.
    fn verify(&self) -> bool {
        self.verify_signature(self.pkey_by_who())
    }

    /// Retrieves two accounts to apply the edition; they should be previously registered
    /// with the help of [`TxCreateAccount`] transactions. Checks the editor's
    /// authorship and superuser rights and applies changes to edited account if the editor's permissions
    /// are sufficient. Otherwise, performs no op.
    ///
    /// [`TxCreateAccount`]: ../transactions/struct.TxCreateAccount.html
    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = EmployeesSchema::new(view);

        // Someone is trying to edit an account that doesn't belong to him
        if *self.pkey_by_who() != *self.pkey_account() {
            // It must be a superuser, otherwise do nothing
            // if !schema.is_superuser(&self.pkey_by_who()) {
            return Err(ErrorKind::NotASuperUser.into());
            // }
        }

        if let Some(account) = schema.account(self.pkey_account()) {
            let username = self.first_name();
            let encrypted_password = self.last_name();
            let photo_ipfs = self.photo_ipfs();
            let is_active = self.is_active();
            let custom_data = self.custom_data();

            // If there's an account with the same ID number exists,
            // check that it's the same account as we've been editing now.
            //
            // Otherwise, do nothing since an ID number supposed to be unique
            // if let Some(pk) = schema.account_pk_by_employee_id().get(&id_number) {
            //     if pk != *self.pkey_account() {
            //         return Err(ErrorKind::EmployeeIdAlreadyExists.into());
            //     }
            // }

            schema.account_edit(
                account,
                self.pkey_account(),
                username,
                encrypted_password,
                photo_ipfs,
                is_active,
                custom_data,
            );
        }

        Ok(())
    }
}

// impl Transaction for TxSetCustomData {
    /// Verifies integrity of the transaction by checking the transaction
    /// signatures.
    // fn verify(&self) -> bool {
    //     self.verify_signature(self.pkey_by_who())
    // }

    /// Retrieves two accounts to apply the edition; they should be previously registered
    /// with the help of [`TxCreateAccount`] transactions. Checks the editor's
    /// authorship and superuser rights and applies changes to edited account if the editor's permissions
    /// are sufficient. Otherwise, performs no op.
    ///
    /// [`TxCreateAccount`]: ../transactions/struct.TxCreateAccount.html
    // fn execute(&self, view: &mut Fork) -> ExecutionResult {
    //     let mut schema = EmployeesSchema::new(view);

        // Someone is trying to edit an account that doesn't belong to him
        // if *self.pkey_by_who() != *self.pkey_account() {
        //     // It must be a superuser, otherwise do nothing
        //     if !schema.is_superuser(&self.pkey_by_who()) {
        //         return Err(ErrorKind::NotASuperUser.into());
        //     }
        // }

        // let no_account = schema.account(self.pkey_account()).is_none();

        // Account must exist
        // if no_account {
        //     return Err(ErrorKind::AccountNotFound.into());
        // }

        // Change custom data in the account
        // let account = schema.account(self.pkey_account()).unwrap(); // Guaranteed to be `Some` by `if no_account` clause above
        // schema.account_set_custom_data(account, self.pkey_account(), self.custom_data());

        // Ok(())
    // }
// }

// impl Transaction for TxDeleteAccount {
//     /// Verifies integrity of the transaction by checking the transaction
//     /// signatures.
//     fn verify(&self) -> bool {
//         self.verify_signature(self.pkey_by_who())
//     }

//     /// Retrieves two accounts to apply the edition; they should be previously registered
//     /// with the help of [`TxCreateAccount`] transactions. Checks the editor's
//     /// authorship and superuser rights and deletes account if the editor's permissions
//     /// are sufficient. Otherwise, performs no op.
//     ///
//     /// [`TxCreateAccount`]: ../transactions/struct.TxCreateAccount.html
//     fn execute(&self, view: &mut Fork) -> ExecutionResult {
//         let mut schema = EmployeesSchema::new(view);

//         let superuser = schema.is_superuser(&self.pkey_by_who());
//         let account = schema.account(self.pkey_account());

//         if let Some(ref account) = account {
//             if superuser {
//                 // Remove the account
//                 schema.account_delete(&account.id_number(), self.pkey_account());
//             }
//         } else {
//             return Err(ErrorKind::AccountNotFound.into());
//         }

//         Ok(())
//     }
// }

/// Error codes emitted by `TxCreateAccount`, `TxEditAccount`, `TxSetCustomData`, `TxDeleteAccount` transactions during execution.
#[derive(Display, Primitive)]
pub enum ErrorKind {
    /// Error can be emitted by `TxCreateAccount`
    #[display(fmt = "Account already exists.")]
    AccountAlreadyExists = 1,

    /// Error can be emitted by `TxEditAccount`, `TxSetCustomData`
    #[display(fmt = "Editor not found.")]
    EditorNotFound = 2,

    /// Error can be emitted by any transaction except `TxCreateAccount`
    #[display(fmt = "Account not found.")]
    AccountNotFound = 3,

    /// Can be emitted by `TxCreateAccount`, `TxEditAccount`, `TxSetCustomData`, `TxDeleteAccount`
    #[display(fmt = "Not a superuser.")]
    NotASuperUser = 4,

    /// Can be emitted by `TxEditAccount`, `TxCreateAccount`
    #[display(fmt = "Employee ID already exists")]
    EmployeeIdAlreadyExists = 5,
}

impl ErrorKind {
    /// Converts error to the raw code
    pub fn into_code(self) -> u8 {
        self.to_u8().unwrap()
    }
}

impl From<ErrorKind> for ExecutionError {
    fn from(value: ErrorKind) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

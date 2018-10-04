use exonum::crypto::PublicKey;
use schema::EmployeeId;
use service::SERVICE_ID;

transactions! {
    pub EmployeesTransactions {
        const SERVICE_ID = SERVICE_ID;

        /// Transaction type for creating a new account.
        ///
        /// See [the `Transaction` trait implementation](#impl-Transaction) for details how
        /// `TxCreateAccount` transactions are processed.
        struct TxCreateAccount {
            /// Public key of the editor. Must be a superuser.
            pkey_by_who: &PublicKey,

            /// Public key of the account's owner.
            pub_key: &PublicKey,

            /// UTF-8 string with the owner's name.
            first_name: &str,

            /// UTF-8 string with the owner's surname.
            last_name: &str,

            /// 64-bit ID document number of an author.
            id_number: EmployeeId,
        }

        /// Transaction type for editing an account in database.
        ///
        /// See [the `Transaction` trait implementation](#impl-Transaction) for details how
        /// `TxEditAccount` transactions are processed.
        struct TxEditAccount {
            /// Public key of the editor.
            pkey_by_who: &PublicKey,

            /// Public key of the account owner.
            pkey_account: &PublicKey,

            /// UTF-8 string with the owner's name.
            first_name: &str,

            /// UTF-8 string with the owner's surname.
            last_name: &str,

            /// 64-bit ID document number of an author.
            id_number: EmployeeId,

            /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
            ///
            /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
            seed: u64,
        }

        /// Transaction type editing an account's custom data in database.
        ///
        /// See [the `Transaction` trait implementation](#impl-Transaction) for details how
        /// `TxSetCustomData` transactions are processed.
        struct TxSetCustomData {
            /// Public key of the editor.
            pkey_by_who: &PublicKey,

            /// Public key of the account owner.
            pkey_account: &PublicKey,

            /// Optional set of custom data bytes.
            custom_data: &str,

            /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
            ///
            /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
            seed: u64,
        }

        /// Transaction type for account's deletion.
        ///
        /// See [the `Transaction` trait implementation](#impl-Transaction) for details how
        /// `TxDeleteAccount` transactions are processed.
        struct TxDeleteAccount {
            /// Public key of the editor.
            pkey_by_who: &PublicKey,

            /// Public key of the account owner.
            pkey_account: &PublicKey,

            /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
            ///
            /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
            seed: u64,
        }
    }
}
use exonum::crypto::{ PublicKey, Hash };
// use schema::EmployeeId;
use service::SERVICE_ID;

transactions! {
    pub RoleSystemTransactions {
        const SERVICE_ID = SERVICE_ID;

- хэш пароля с шифрованием открытым ключём

        struct TxCreateAccount {
            /// Public key of the account's owner.
            pub_key: &PublicKey,

            /// UTF-8 string with the owner's username.
            username: &str,

            /// password_encrypted_by_secret_key
            encrypted_password: &str,

            /// user's photo stored in IPFS 
            photo_ipfs: &Hash,

        }

        /// backlog - не сейчас
        struct TxEditAccount {

            /// Public key of the account owner.
            pkey_account: &PublicKey,

            /// UTF-8 string with the owner's username.
            username: &str,

            /// password_encrypted_by_secret_key
            encrypted_password: &str,

            /// user's photo stored in IPFS 
            photo_ipfs: &Hash,

            seed: u64,
        }

        /// Transaction type editing an account's custom data in database.
        struct TxSetCustomData {

            /// Public key of the account owner.
            pkey_account: &PublicKey,

            /// Optional set of custom data bytes.
            custom_data: &str,

            seed: u64,
        }

        /// Transaction type for account's deactivation.
        struct TxDeactivateAccount {

            /// Public key of the account owner.
            pkey_account: &PublicKey,

            seed: u64,
        }
    }
}
use types;

trait Extract {
    fn extract(script: chain::script, mainnet_p2kh: u8, mainnet_p2sh: u8) -> list_payment_address;
    fn extract_input(script: chain::script, mainnet_p2kh: u8, mainnet_p2sh: u8) -> list_payment_address;
    fn extract_output(script: chain::script, mainnet_p2kh: u8, mainnet_p2sh: u8) -> list_payment_address;
}


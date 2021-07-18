use anchor_client::solana_sdk::signature::Keypair;
use rand::rngs::OsRng;

/// generates a keypair and returns its base58 encoded self
pub fn generate_keypair_with_base58() -> (Keypair, String) {
    let mut csprng = OsRng {};
    let kp = Keypair::generate(&mut csprng);
    let kp_58 = kp.to_base58_string();
    (kp, kp_58)
}

pub fn keypair_from_base58(input: &str) -> Keypair {
    Keypair::from_base58_string(input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_keypair() {
        let (kp, kp_58) = generate_keypair_with_base58();
        let kp2 = keypair_from_base58(kp_58.as_str());
        let kp2_58 = kp2.to_base58_string();
        assert!(kp_58 == kp2_58);
        assert!(kp == kp2);
    }
}

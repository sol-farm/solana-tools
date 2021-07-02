use crate::hashmap::PUBKEY_MAP;
use anchor_client::solana_sdk::pubkey::Pubkey;

pub const USDC_TOKEN_DECIMALS: u8 = 6;
pub const USDT_TOKEN_DECIMALS: u8 = 6;
pub const WSOL_TOKEN_DECIMALS: u8 = 9;
pub const RAY_TOKEN_DECIMALS: u8 = 6;
pub const SRM_TOKEN_DECIMALS: u8 = 6;
pub const RAY_USDC_LP_TOKEN_DECIMALS: u8 = 6;
pub const RAY_USDT_LP_TOKEN_DECIMALS: u8 = 6;
pub const RAY_SOL_LP_TOKEN_DECIMALS: u8 = 6;
pub const RAY_SRM_LP_TOKEN_DECIMALS: u8 = 6;

pub fn usdc_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("usdc_token_mint").unwrap()
}

pub fn usdt_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("usdt_token_mint").unwrap()
}

pub fn wsol_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("wsol_token_mint").unwrap()
}

pub fn ray_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("ray_token_mint").unwrap()
}

pub fn srm_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("srm_token_mint").unwrap()
}

pub fn ray_usdc_lp_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("ray_usdc_lp_token_mint").unwrap()
}

pub fn ray_usdt_lp_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("ray_usdt_lp_token_mint").unwrap()
}

pub fn ray_sol_lp_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("ray_sol_lp_token_mint").unwrap()
}

pub fn ray_srm_lp_token_mint() -> Pubkey {
    *PUBKEY_MAP.get("ray_srm_lp_token_mint").unwrap()
}

#[cfg(test)]
mod tests {
    use crate::token_mints::{
        RAY_SOL_LP_TOKEN_DECIMALS, RAY_SRM_LP_TOKEN_DECIMALS, RAY_TOKEN_DECIMALS,
        RAY_USDC_LP_TOKEN_DECIMALS, RAY_USDT_LP_TOKEN_DECIMALS, SRM_TOKEN_DECIMALS,
        USDC_TOKEN_DECIMALS, USDT_TOKEN_DECIMALS, WSOL_TOKEN_DECIMALS,
    };

    use super::{
        ray_sol_lp_token_mint, ray_srm_lp_token_mint, ray_token_mint, ray_usdc_lp_token_mint,
        ray_usdt_lp_token_mint, srm_token_mint, usdc_token_mint, usdt_token_mint, wsol_token_mint,
    };

    #[test]
    fn test_token_decimals() {
        assert!(USDC_TOKEN_DECIMALS == 6);
        assert!(USDT_TOKEN_DECIMALS == 6);
        assert!(WSOL_TOKEN_DECIMALS == 9);
        assert!(RAY_TOKEN_DECIMALS == 6);
        assert!(SRM_TOKEN_DECIMALS == 6);
        assert!(RAY_USDC_LP_TOKEN_DECIMALS == 6);
        assert!(RAY_USDT_LP_TOKEN_DECIMALS == 6);
        assert!(RAY_SOL_LP_TOKEN_DECIMALS == 6);
        assert!(RAY_SRM_LP_TOKEN_DECIMALS == 6);
    }

    #[test]
    fn test_token_mints() {
        assert!(usdc_token_mint().to_string() == "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
        assert!(usdt_token_mint().to_string() == "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
        assert!(wsol_token_mint().to_string() == "So11111111111111111111111111111111111111112");
        assert!(ray_token_mint().to_string() == "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
        assert!(srm_token_mint().to_string() == "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt");
        assert!(
            ray_usdc_lp_token_mint().to_string() == "BZFGfXMrjG2sS7QT2eiCDEevPFnkYYF7kzJpWfYxPbcx"
        );
        assert!(
            ray_usdt_lp_token_mint().to_string() == "C3sT1R3nsw4AVdepvLTLKr5Gvszr7jufyBWUCvy4TUvT"
        );
        assert!(
            ray_sol_lp_token_mint().to_string() == "F5PPQHGcznZ2FxD9JaxJMXaf7XkaFFJ6zzTBcW8osQjw"
        );
        assert!(
            ray_srm_lp_token_mint().to_string() == "DSX5E21RE9FB9hM8Nh8xcXQfPK6SzRaJiywemHBSsfup"
        );
    }
}

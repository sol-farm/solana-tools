use std::collections::HashMap;

use anchor_client::solana_sdk::pubkey::Pubkey;
use once_cell::sync::Lazy;
use std::str::FromStr;

// used for a static hashmap that prevents repeated parsing of strings into Pubkey's
pub static PUBKEY_MAP: Lazy<HashMap<&str, Pubkey>> = Lazy::new(|| {
    let mut m: HashMap<&str, Pubkey> = HashMap::new();
    m.insert(
        "usdc_token_mint",
        Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
            .expect("failed to parse usdc token mint"),
    );
    m.insert(
        "usdt_token_mint",
        Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB")
            .expect("failed to parse usdt token mint"),
    );
    m.insert(
        "wsol_token_mint",
        Pubkey::from_str("So11111111111111111111111111111111111111112")
            .expect("failed to parse wsol token mint"),
    );
    m.insert(
        "ray_token_mint",
        Pubkey::from_str("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R")
            .expect("failed to parse ray token mint"),
    );
    m.insert(
        "srm_token_mint",
        Pubkey::from_str("SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt")
            .expect("failed to parse srm token mint"),
    );
    m.insert(
        "ray_usdc_lp_token_mint",
        Pubkey::from_str("BZFGfXMrjG2sS7QT2eiCDEevPFnkYYF7kzJpWfYxPbcx")
            .expect("failed to parse ray-usdc lp token mint"),
    );
    m.insert(
        "ray_usdt_lp_token_mint",
        Pubkey::from_str("C3sT1R3nsw4AVdepvLTLKr5Gvszr7jufyBWUCvy4TUvT")
            .expect("failed to parse ray-usdt lp token mint"),
    );
    m.insert(
        "ray_sol_lp_token_mint",
        Pubkey::from_str("F5PPQHGcznZ2FxD9JaxJMXaf7XkaFFJ6zzTBcW8osQjw")
            .expect("failed to parse ray-sol lp token mint"),
    );
    m.insert(
        "ray_srm_lp_token_mint",
        Pubkey::from_str("DSX5E21RE9FB9hM8Nh8xcXQfPK6SzRaJiywemHBSsfup")
            .expect("failed to parse ray-srm token mint"),
    );
    m.insert(
        "ray_sol_market",
        Pubkey::from_str("C6tp2RVZnxBPFbnAsfTjis8BN9tycESAT4SgDQgbbrsA")
            .expect("failed to parse ray-sol market"),
    );
    m.insert(
        "ray_srm_market",
        Pubkey::from_str("Cm4MmknScg7qbKqytb1mM92xgDxv3TNXos4tKbBqTDy7")
            .expect("failed to parse ray srm market"),
    );
    m.insert(
        "ray_usdc_market",
        Pubkey::from_str("2xiv8A5xrJ7RnGdxXB42uFEkYHJjszEhaJyKKt4WaLep")
            .expect("failed to parse ray-usdc market"),
    );
    m.insert(
        "ray_usdt_market",
        Pubkey::from_str("teE55QrL4a4QSfydR9dnHF97jgCfptpuigbb53Lo95g")
            .expect("failed to parse ray-usdt market"),
    );
    m.insert(
        "devnet_serum_program_id",
        Pubkey::from_str("DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY")
            .expect("failed to parse devnet serum program id"),
    );
    m.insert(
        "mainnet_serum_program_id",
        Pubkey::from_str("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")
            .expect("failed to parse mainnet serum program id"),
    );
    m.insert(
        "ray_sol_open_orders",
        Pubkey::from_str("JQEY8R9frhxuvcsewGfgkCVdGWztpHLx4P9zmTAsZFM")
            .expect("failed to parse ray-sol open orders"),
    );
    m.insert(
        "ray_srm_open_orders",
        Pubkey::from_str("6CVRtzecMaPZ1pdfT2ZzJ1qf89yuFsD7MKYGwvjYsy6w")
            .expect("failed to parse ray-srm open orders"),
    );
    m.insert(
        "ray_usdc_open_orders",
        Pubkey::from_str("3Xq4vBd5EWs45v9YwG1Mpfr8Xjng23pDovVUbnAaPce9")
            .expect("failed to parse ray-usdc open orders"),
    );
    m.insert(
        "ray_usdt_open_orders",
        Pubkey::from_str("7UF3m8hDGZ6bNnHzaT2YHrhp7A7n9qFfBj6QEpHPv5S8")
            .expect("failed to parse ray-usdt open orders"),
    );
    m.insert(
        "ray_sol_amm_id",
        Pubkey::from_str("HeRUVkQyPuJAPFXUkTaJaWzimBopWbJ54q5DCMuPpBY4")
            .expect("failed to parse ray-sol amm id"),
    );
    m.insert(
        "ray_srm_amm_id",
        Pubkey::from_str("EGhB6FdyHtJPbPMRoBC8eeUVnVh2iRgnQ9HZBKAw46Uy")
            .expect("failed to parse ray-srm amm id"),
    );
    m.insert(
        "ray_usdc_amm_id",
        Pubkey::from_str("5NMFfbccSpLdre6anA8P8vVy35n2a52AJiNPpQn8tJnE")
            .expect("failed to pary ray-usdc amm id"),
    );
    m.insert(
        "ray_usdt_amm_id",
        Pubkey::from_str("DVa7Qmb5ct9RCpaU7UTpSaf3GVMYz17vNVU67XpdCRut")
            .expect("failed to parse ray-usdt amm id"),
    );
    m.insert(
        "sol_usdc_market",
        Pubkey::from_str("9wFFyRfZBsuAha4YcuxcXLKwMxJR43S7fPfQLusDBzvT")
            .expect("failed to parse sol-usdc market"),
    );
    m.insert(
        "sol_usdc_open_orders",
        Pubkey::from_str("HRk9CMrpq7Jn9sh7mzxE8CChHG8dneX9p475QKz4Fsfc")
            .expect("failed to pares sol-usdc open orders"),
    );
    m.insert(
        "sol_usdc_lp_token_mint",
        Pubkey::from_str("8HoQnePLqPj4M7PUDzfw8e3Ymdwgc7NLGnaTUapubyvu")
            .expect("failed to parse sol-usdc lp token mint"),
    );
    m.insert(
        "sol_usdc_amm_id",
        Pubkey::from_str("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2")
            .expect("failed to parse sol-usdc amm id"),
    );
    m.insert(
        "srm_usdc_market",
        Pubkey::from_str("ByRys5tuUWDgL73G8JBAEfkdFf8JWBzPBDHsBVQ5vbQA")
            .expect("failed to parse srm-usdc market"),
    );
    m.insert(
        "srm_usdc_open_orders",
        Pubkey::from_str("GJwrRrNeeQKY2eGzuXGc3KBrBftYbidCYhmA6AZj2Zur")
            .expect("failed to parse srm-usdc open orders"),
    );
    m.insert(
        "srm_usdc_lp_token_mint",
        Pubkey::from_str("9XnZd82j34KxNLgQfz29jGbYdxsYznTWRpvZE3SRE7JG")
            .expect("failed to parse srm-usd lp token mint"),
    );
    m.insert(
        "srm_usdc_amm_id",
        Pubkey::from_str("8tzS7SkUZyHPQY7gLqsMCXZ5EDCgjESUHcB17tiR1h3Z")
            .expect("failed to parse srm usdc amm id"),
    );
    m.insert(
        "usdt_usdc_market",
        Pubkey::from_str("77quYg4MGneUdjgXCunt9GgM1usmrxKY31twEy3WHwcS")
        .expect("failed to parse usdt-usdc market")
    );
    m.insert(
        "usdt_usdc_amm_id",
        Pubkey::from_str("7TbGqz32RsuwXbXY7EyBCiAnMbJq1gm1wKmfjQjuwoyF")
        .expect("failed to parse usdt-usdc amm id")
    );
    m.insert(
        "usdt_usdc_lp_token_mint",
        Pubkey::from_str("HqbxvyDnod2zTrhRJ5sSJn4CNnake6M9ksQjHxBcHBZj")
        .expect("failed to parse usdt-usdc lp token mint")
    );
    m.insert(
        "usdt_usdc_open_orders",
        Pubkey::from_str("6XXvXS3meWqnftEMUgdY8hDWGJfrb8t22x2k1WyVYwhF")
        .expect("failed to parse usdt-usdc open orders")
    );
    m
});

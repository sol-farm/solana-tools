use crate::errors;
use crate::errors::UtilsError;
use crate::hashmap::PUBKEY_MAP;
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{account_info::IntoAccountInfo, pubkey::Pubkey, sysvar},
};
use anyhow::Result;
use num_traits::pow::Pow;
use serum_dex::critbit::SlabView;
use serum_dex::matching::OrderBookState;
use serum_dex::state::{MarketState, OpenOrders};
use std::sync::Arc;

pub fn ray_sol_market() -> Pubkey {
    *PUBKEY_MAP.get("ray_sol_market").unwrap()
}

pub fn ray_srm_market() -> Pubkey {
    *PUBKEY_MAP.get("ray_srm_market").unwrap()
}

pub fn ray_usdc_market() -> Pubkey {
    *PUBKEY_MAP.get("ray_usdc_market").unwrap()
}

pub fn ray_usdt_market() -> Pubkey {
    *PUBKEY_MAP.get("ray_usdt_market").unwrap()
}

pub fn sol_usdc_market() -> Pubkey {
    *PUBKEY_MAP.get("sol_usdc_market").unwrap()
}

pub fn srm_usdc_market() -> Pubkey {
    *PUBKEY_MAP.get("srm_usdc_market").unwrap()
}

pub fn usdt_usdc_market() -> Pubkey {
    *PUBKEY_MAP.get("usdt_usdc_market").unwrap()
}

pub fn devnet_serum_program_id() -> Pubkey {
    *PUBKEY_MAP.get("devnet_serum_program_id").unwrap()
}

pub fn mainnet_serum_program_id() -> Pubkey {
    *PUBKEY_MAP.get("mainnet_serum_program_id").unwrap()
}

/// helper function that calls load_serum_market -> load_open_orders
///
pub fn load_serum_open_orders_order_book_state(
    rpc: &Arc<RpcClient>,
    market_key: Pubkey,
    serum_program_id: Pubkey,
    open_orders_key: Pubkey,
) -> Result<(OpenOrders, MarketState)> {
    let rent_key = sysvar::rent::id();
    let mut accounts = rpc.get_multiple_accounts(&[market_key, open_orders_key, rent_key])?;
    if accounts.len() != 3 {
        return Err(UtilsError::InsufficientAccounts.into());
    }

    let market_account = std::mem::take(&mut accounts[0]);
    if market_account.is_none() {
        return Err(UtilsError::MarketAccountISNone.into());
    }
    let market_account = market_account.unwrap();

    let open_orders_account = std::mem::take(&mut accounts[1]);
    if open_orders_account.is_none() {
        return Err(UtilsError::OpenOrdersAccountIsNone.into()); 
    }
    let open_orders_account = open_orders_account.unwrap();

    let rent_account = std::mem::take(&mut accounts[2]);
    if rent_account.is_none() {
        return Err(UtilsError::RentAccountIsNone.into());
    }

    let rent_account = rent_account.unwrap();
    let mut rent_tuple = (rent_key, rent_account);
    let rent_sysvar_account = rent_tuple.into_account_info();
    let rent_sysvar = sysvar::Sysvar::from_account_info(&rent_sysvar_account)?;

    let mut market_tuple = (market_key, market_account);
    let serum_market = market_tuple.into_account_info();
    let serum_market_state = MarketState::load(&serum_market, &serum_program_id)?;

    let mut orders_tuple = (open_orders_key, open_orders_account);
    let open_orders = orders_tuple.into_account_info();
    let open_orders_state = serum_market_state.load_orders_mut(
        &open_orders,
        None,
        &serum_program_id,
        Some(rent_sysvar),
    )?;

    Ok((open_orders_state.to_owned(), serum_market_state.to_owned()))
}

/// loads a serum dex MarketState object
pub fn load_serum_market(
    rpc: &Arc<RpcClient>,
    market_key: Pubkey,
    serum_program_id: Pubkey,
) -> Result<MarketState> {
    let serum_market_account = rpc.get_account(&market_key)?;
    let mut account_tuple = (market_key, serum_market_account);
    let serum_market = account_tuple.into_account_info();
    let serum_market_state = MarketState::load(&serum_market, &serum_program_id)?;
    Ok(serum_market_state.to_owned())
}

/// loads a serum dex OpenOrders object
pub fn load_open_orders(
    rpc: &Arc<RpcClient>,
    market_state: MarketState,
    open_orders_key: Pubkey,
    serum_program_id: Pubkey,
) -> Result<OpenOrders> {
    let open_orders_account = rpc.get_account(&open_orders_key)?;
    let mut account_tuple = (open_orders_key, open_orders_account);
    let open_orders = account_tuple.into_account_info();

    let rent_key = sysvar::rent::id();
    let rent_account = rpc.get_account(&rent_key)?;
    let mut rent_tuple = (rent_key, rent_account);
    let rent_sysvar_account = rent_tuple.into_account_info();
    let rent_sysvar = sysvar::Sysvar::from_account_info(&rent_sysvar_account)?;

    let open_orders_state =
        market_state.load_orders_mut(&open_orders, None, &serum_program_id, Some(rent_sysvar))?;
    Ok(open_orders_state.to_owned())
}

/// returns the bbo prices for ask and bid
pub fn find_best_ask_bid_price(order_book: &OrderBookState) -> Result<(u64, u64)> {
    let ask_node = order_book.asks.find_min();
    if ask_node.is_none() {
        return Err(errors::UtilsError::FindAsksMin.into());
    }
    let ask_node = ask_node.unwrap();

    let bid_node = order_book.bids.find_max();
    if bid_node.is_none() {
        return Err(errors::UtilsError::FindBidsMax.into());
    }
    let bid_node = bid_node.unwrap();

    let asks_price = {
        let any_node = order_book.asks.get(ask_node);
        if any_node.is_none() {
            return Err(errors::UtilsError::GetAsksNode.into());
        }
        let leaf_node = any_node.unwrap().as_leaf();
        if leaf_node.is_none() {
            return Err(errors::UtilsError::AsksNodeAsLeaf.into());
        }
        let leaf_node = leaf_node.unwrap();
        u64::from(leaf_node.price())
    };
    let bids_price = {
        let any_node = order_book.bids.get(bid_node);
        if any_node.is_none() {
            return Err(errors::UtilsError::GetBidsNode.into());
        }
        let leaf_node = any_node.unwrap().as_leaf();
        if leaf_node.is_none() {
            return Err(errors::UtilsError::BidsNodeAsLeaf.into());
        }
        let leaf_node = leaf_node.unwrap();
        u64::from(leaf_node.price())
    };
    Ok((asks_price, bids_price))
}

pub fn tick_size(
    base_lot_size: u64,
    quote_lot_size: u64,
    base_token_decimals: u8,
    quote_token_decimals: u8,
) -> f64 {
    // a hard coded value
    // see https://github.com/project-serum/serum-ts/blob/e28c020cc2d5fca18f50ac1b484799cfab9f23b2/packages/serum/src/market.ts#L1148
    const PRICE: f64 = 1_f64;
    // a hard coded value
    // see https://github.com/project-serum/serum-ts/blob/e28c020cc2d5fca18f50ac1b484799cfab9f23b2/packages/serum/src/market.ts#L1078-L1084
    const TEN: f64 = 10_f64;

    let base_multiplier = TEN.pow(base_token_decimals as f64);
    let quote_multiplier = TEN.pow(quote_token_decimals as f64);

    let a = (PRICE * (quote_lot_size as f64)) * (base_multiplier as f64);

    let b = (base_lot_size as f64) * (quote_multiplier as f64);

    a / b
}

#[cfg(test)]
mod test {
    use super::*;
    use anchor_client::{
        solana_client::rpc_client::RpcClient, solana_sdk::pubkey::Pubkey, Cluster,
    };
    use std::str::FromStr;

    #[test]
    pub fn test_market_keys() {
        assert!(ray_sol_market().to_string() == "C6tp2RVZnxBPFbnAsfTjis8BN9tycESAT4SgDQgbbrsA");
        assert!(ray_srm_market().to_string() == "Cm4MmknScg7qbKqytb1mM92xgDxv3TNXos4tKbBqTDy7");
        assert!(ray_usdc_market().to_string() == "2xiv8A5xrJ7RnGdxXB42uFEkYHJjszEhaJyKKt4WaLep");
        assert!(ray_usdt_market().to_string() == "teE55QrL4a4QSfydR9dnHF97jgCfptpuigbb53Lo95g");
        assert!(sol_usdc_market().to_string() == "9wFFyRfZBsuAha4YcuxcXLKwMxJR43S7fPfQLusDBzvT");
        assert!(usdt_usdc_market().to_string() == "77quYg4MGneUdjgXCunt9GgM1usmrxKY31twEy3WHwcS");
    }

    #[test]
    pub fn test_load_serum_market_open_orders() {
        let ray_usdt_open_orders =
            Pubkey::from_str("7UF3m8hDGZ6bNnHzaT2YHrhp7A7n9qFfBj6QEpHPv5S8").unwrap();
        let serum_market = Pubkey::from_str("teE55QrL4a4QSfydR9dnHF97jgCfptpuigbb53Lo95g").unwrap();
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));

        let market_state =
            load_serum_market(&rpc, serum_market, mainnet_serum_program_id()).unwrap();
        println!("market state {:#?}", market_state);

        let open_orders = load_open_orders(
            &rpc,
            market_state,
            ray_usdt_open_orders,
            mainnet_serum_program_id(),
        )
        .unwrap();

        // https://github.com/rust-lang/rust/issues/82523
        println!("open_orders account flags {}", {
            open_orders.account_flags
        });
        println!("open orders market {:#?}", { open_orders.market });
        println!("open orders owner {:#?}", { open_orders.owner });
        println!("open orders native_coin_free {}", {
            open_orders.native_coin_free
        });
        println!("open orders native_coin_total {}", {
            open_orders.native_coin_total
        });
        println!("open orders native_pc_free {}", {
            open_orders.native_pc_free
        });
        println!("open order native_pc_total {}", {
            open_orders.native_pc_total
        });
        println!("open order free_slot_bits {}", {
            open_orders.free_slot_bits
        });
        println!("open order is_bid_bits {}", { open_orders.is_bid_bits });
    }
}

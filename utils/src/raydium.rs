use crate::errors::UtilsError;
use crate::serum;
use crate::serum::mainnet_serum_program_id;
use crate::serum::ray_sol_market;
use crate::serum::ray_srm_market;
use crate::serum::ray_usdc_market;
use crate::serum::ray_usdt_market;
use crate::serum::sol_usdc_market;
use crate::serum::srm_usdc_market;
use crate::serum::usdt_usdc_market;
use crate::token_mints::RAY_TOKEN_DECIMALS;
use crate::token_mints::SRM_TOKEN_DECIMALS;
use crate::token_mints::USDC_TOKEN_DECIMALS;
use crate::token_mints::USDT_TOKEN_DECIMALS;
use crate::token_mints::WSOL_TOKEN_DECIMALS;
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{account_info::IntoAccountInfo, program_pack::Pack, pubkey::Pubkey},
};
use anchor_lang::__private::bytemuck::cast_slice;
use anyhow::{Result, anyhow};
use arrayref::{array_ref, array_refs};
use serum_dex::matching::OrderBookState;
use spl_token::amount_to_ui_amount;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint as MintAccount;
use std::convert::identity;
use std::ops::DerefMut;
use std::sync::Arc;

use crate::hashmap::PUBKEY_MAP;

#[derive(Debug, Clone, Default)]
pub struct AmmInfoLayoutV3 {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub coin_decimals: u64,
    pub pc_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub fee: u64,
    pub min_separate: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub pnl_ratio: u64,
    pub amount_wave_ratio: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub need_take_pnl_coin: u64,
    pub need_take_pnl_pc: u64,
    pub total_pnl_x: u64,
    pub total_pnl_y: u64,
    pub system_decimals_value: u64,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub coin_mint_address: Pubkey,
    pub pc_mint_address: Pubkey,
    pub lp_mint_address: Pubkey,
    pub amm_open_orders: Pubkey,
    pub serum_market: Pubkey,
    pub serum_program_id: Pubkey,
    pub amm_target_orders: Pubkey,
    pub amm_quantities: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub amm_owner: Pubkey,
    pub pnl_owner: Pubkey,
    pub srm_token_account: Pubkey,
}

#[derive(Debug, Clone, Default)]
pub struct AmmInfoLayoutV4 {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub coin_decimals: u64,
    pub pc_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave_ratio: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub system_decimals_value: u64,
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
    pub need_take_pnl_coin: u64,
    pub need_take_pnl_pc: u64,
    pub total_pnl_pc: u64,
    pub total_pnl_coin: u64,
    pub pool_total_deposit_pc: u128,
    pub pool_total_deposit_coin: u128,
    pub swap_coin_in_amount: u128,
    pub swap_pc_out_amount: u128,
    pub swap_coin_2_pc_fee: u64,
    pub swap_pc_in_amount: u128,
    pub swap_coin_out_amount: u128,
    pub swap_pc_2_coin_fee: u64,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub coin_mint_address: Pubkey,
    pub pc_mint_address: Pubkey,
    pub lp_mint_address: Pubkey,
    pub amm_open_orders: Pubkey,
    pub serum_market: Pubkey,
    pub serum_program_id: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub amm_owner: Pubkey,
    pub pnl_owner: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub enum AMMs {
    RAYSOL,
    RAYSRM,
    RAYUSDT,
    RAYUSDC,
    SOLUSDC,
    SRMUSDC,
    USDTUSDC,
}

impl AMMs {
    pub fn open_orders(self) -> Pubkey {
        match self {
            AMMs::RAYSOL => *PUBKEY_MAP.get("ray_sol_open_orders").unwrap(),
            AMMs::RAYSRM => *PUBKEY_MAP.get("ray_srm_open_orders").unwrap(),
            AMMs::RAYUSDC => *PUBKEY_MAP.get("ray_usdc_open_orders").unwrap(),
            AMMs::RAYUSDT => *PUBKEY_MAP.get("ray_usdt_open_orders").unwrap(),
            AMMs::SOLUSDC => *PUBKEY_MAP.get("sol_usdc_open_orders").unwrap(),
            AMMs::SRMUSDC => *PUBKEY_MAP.get("srm_usdc_open_orders").unwrap(),
            AMMs::USDTUSDC => *PUBKEY_MAP.get("usdt_usdc_open_orders").unwrap(),
        }
    }
    pub fn lp_token_mint(self) -> Pubkey {
        match self {
            AMMs::RAYSOL => *PUBKEY_MAP.get("ray_sol_lp_token_mint").unwrap(),
            AMMs::RAYSRM => *PUBKEY_MAP.get("ray_srm_lp_token_mint").unwrap(),
            AMMs::RAYUSDC => *PUBKEY_MAP.get("ray_usdc_lp_token_mint").unwrap(),
            AMMs::RAYUSDT => *PUBKEY_MAP.get("ray_usdt_lp_token_mint").unwrap(),
            AMMs::SOLUSDC => *PUBKEY_MAP.get("sol_usdc_lp_token_mint").unwrap(),
            AMMs::SRMUSDC => *PUBKEY_MAP.get("srm_usdc_lp_token_mint").unwrap(),
            AMMs::USDTUSDC => *PUBKEY_MAP.get("usdt_usdc_lp_token_mint").unwrap(),
        }
    }
    pub fn amm_id(self) -> Pubkey {
        match self {
            AMMs::RAYSOL => *PUBKEY_MAP.get("ray_sol_amm_id").unwrap(),
            AMMs::RAYSRM => *PUBKEY_MAP.get("ray_srm_amm_id").unwrap(),
            AMMs::RAYUSDC => *PUBKEY_MAP.get("ray_usdc_amm_id").unwrap(),
            AMMs::RAYUSDT => *PUBKEY_MAP.get("ray_usdt_amm_id").unwrap(),
            AMMs::SOLUSDC => *PUBKEY_MAP.get("sol_usdc_amm_id").unwrap(),
            AMMs::SRMUSDC => *PUBKEY_MAP.get("srm_usdc_amm_id").unwrap(),
            AMMs::USDTUSDC => *PUBKEY_MAP.get("usdt_usdc_amm_id").unwrap(),
        }
    }
    pub fn serum_market(&self) -> Pubkey {
        match self {
            AMMs::RAYSOL => ray_sol_market(),
            AMMs::RAYSRM => ray_srm_market(),
            AMMs::RAYUSDC => ray_usdc_market(),
            AMMs::RAYUSDT => ray_usdt_market(),
            AMMs::SOLUSDC => sol_usdc_market(),
            AMMs::SRMUSDC => srm_usdc_market(),
            AMMs::USDTUSDC => usdt_usdc_market(),
        }
    }
    pub fn base_token_decimals(&self) -> u8 {
        match self {
            AMMs::RAYSOL | AMMs::RAYSRM | AMMs::RAYUSDC | AMMs::RAYUSDT => RAY_TOKEN_DECIMALS,
            AMMs::SOLUSDC => WSOL_TOKEN_DECIMALS,
            AMMs::SRMUSDC => SRM_TOKEN_DECIMALS,
            AMMs::USDTUSDC => USDT_TOKEN_DECIMALS,
        }
    }
    pub fn quote_token_decimals(&self) -> u8 {
        match self {
            AMMs::RAYSOL => WSOL_TOKEN_DECIMALS,
            AMMs::RAYSRM => SRM_TOKEN_DECIMALS,
            AMMs::RAYUSDC | AMMs::SOLUSDC | AMMs::SRMUSDC  => USDC_TOKEN_DECIMALS,
            AMMs::RAYUSDT => USDT_TOKEN_DECIMALS,
            AMMs::USDTUSDC => USDC_TOKEN_DECIMALS,
        }
    }
    pub fn calculate_lp_token_price(self, rpc: &Arc<RpcClient>) -> Result<f64> {
        AMMs::lp_token_price(self, rpc)
    }
    pub fn quote_token_usd_price(self, rpc: &Arc<RpcClient>) -> Result<f64> {
        match self {
            AMMs::USDTUSDC => {
                let (open_orders, mut market_state) = serum::load_serum_open_orders_order_book_state(rpc, self.serum_market(), mainnet_serum_program_id(), self.open_orders())?;
                let coin_lot_size = market_state.coin_lot_size;
                let pc_lot_size = market_state.pc_lot_size;
        
                let asks_key = Pubkey::new(cast_slice(&identity(market_state.asks) as &[_]));
                let bids_key = Pubkey::new(cast_slice(&identity(market_state.bids) as &[_]));
                let mut account_datas = rpc.get_multiple_accounts(&[asks_key, bids_key])?;
                let asks_acct = std::mem::take(&mut account_datas[0]);
                let bids_acct = std::mem::take(&mut account_datas[1]);
                if asks_acct.is_none() {
                    return Err(UtilsError::AsksAccountIsNone.into());
                }
                if bids_acct.is_none() {
                    return Err(UtilsError::BidsAccountIsNone.into());
                }
                let asks_acct = asks_acct.unwrap();
                let bids_acct = bids_acct.unwrap();
        
                let mut asks_tuple = (asks_key, asks_acct);
                let asks_account = asks_tuple.into_account_info();
        
                let mut bids_tuple = (bids_key, bids_acct);
                let bids_account = bids_tuple.into_account_info();

                let mut asks = market_state.load_asks_mut(&asks_account)?;
                let mut bids = market_state.load_bids_mut(&bids_account)?;
                let order_book_state = OrderBookState {
                    market_state: &mut market_state,
                    asks: &mut asks,
                    bids: &mut bids,
                };
                let tick_size = serum::tick_size(
                    coin_lot_size,
                    pc_lot_size,
                    self.base_token_decimals(),
                    self.quote_token_decimals(),
                );
                let (asks_price, bids_price) = serum::find_best_ask_bid_price(&order_book_state)?;
                let asks_price = asks_price as f64 * tick_size;
                let _bids_price = bids_price as f64 * tick_size;
                let quote_price = 1_f64 / asks_price;
                
                let base_usd_price = self.base_token_usd_price(rpc)?;
                let quote_usd_price = quote_price * base_usd_price;
                Ok(quote_usd_price)
            },
            _ => Err(anyhow!("quote token price not support for pair {:#?}", self).into()),
        }
    }
    pub fn base_token_usd_price(self, rpc: &Arc<RpcClient>) -> Result<f64> {
        match self {
            // just fetch price for RAY_USDC
            AMMs::RAYSOL | AMMs::RAYSRM | AMMs::RAYUSDC | AMMs::RAYUSDT => {
                let mut market_state = serum::load_serum_market(
                    rpc,
                    serum::ray_usdc_market(),
                    serum::mainnet_serum_program_id(),
                )?;
                let coin_lot_size = market_state.coin_lot_size;
                let pc_lot_size = market_state.pc_lot_size;

                let asks_key = Pubkey::new(cast_slice(&identity(market_state.asks) as &[_]));
                let bids_key = Pubkey::new(cast_slice(&identity(market_state.bids) as &[_]));

                let mut accounts = rpc.get_multiple_accounts(&[asks_key, bids_key])?;
                let asks_acct = std::mem::take(&mut accounts[0]);
                if asks_acct.is_none() {
                    return Err(UtilsError::AsksAccountIsNone.into());
                }
                let asks_acct = asks_acct.unwrap();
                let bids_acct = std::mem::take(&mut accounts[1]);
                if bids_acct.is_none() {
                    return Err(UtilsError::BidsAccountIsNone.into());
                }
                let bids_acct = bids_acct.unwrap();
                let mut asks_tuple = (asks_key, asks_acct);
                let asks_account = asks_tuple.into_account_info();

                let mut bids_tuple = (bids_key, bids_acct);
                let bids_account = bids_tuple.into_account_info();

                let mut asks = market_state.load_asks_mut(&asks_account)?;
                let mut bids = market_state.load_bids_mut(&bids_account)?;
                let order_book_state = OrderBookState {
                    market_state: &mut market_state,
                    asks: &mut asks,
                    bids: &mut bids,
                };

                let ray_usdc_tick_size = serum::tick_size(
                    coin_lot_size,
                    pc_lot_size,
                    RAY_TOKEN_DECIMALS,
                    USDC_TOKEN_DECIMALS,
                );
                let (asks_price, _bids_price) = serum::find_best_ask_bid_price(&order_book_state)?;
                Ok(asks_price as f64 * ray_usdc_tick_size)
            }
            AMMs::USDTUSDC => {
                let mut market_state = serum::load_serum_market(
                    rpc,
                    serum::usdt_usdc_market(),
                    serum::mainnet_serum_program_id(),
                )?;
                let coin_lot_size = market_state.coin_lot_size;
                let pc_lot_size = market_state.pc_lot_size;

                let asks_key = Pubkey::new(cast_slice(&identity(market_state.asks) as &[_]));
                let bids_key = Pubkey::new(cast_slice(&identity(market_state.bids) as &[_]));

                let mut accounts = rpc.get_multiple_accounts(&[asks_key, bids_key])?;
                let asks_acct = std::mem::take(&mut accounts[0]);
                if asks_acct.is_none() {
                    return Err(UtilsError::AsksAccountIsNone.into());
                }
                let asks_acct = asks_acct.unwrap();
                let bids_acct = std::mem::take(&mut accounts[1]);
                if bids_acct.is_none() {
                    return Err(UtilsError::BidsAccountIsNone.into());
                }
                let bids_acct = bids_acct.unwrap();


                let mut asks_tuple = (asks_key, asks_acct);
                let asks_account = asks_tuple.into_account_info();

                let mut bids_tuple = (bids_key, bids_acct);
                let bids_account = bids_tuple.into_account_info();

                let mut asks = market_state.load_asks_mut(&asks_account)?;
                let mut bids = market_state.load_bids_mut(&bids_account)?;
                let order_book_state = OrderBookState {
                    market_state: &mut market_state,
                    asks: &mut asks,
                    bids: &mut bids,
                };

                let usdt_usdc_tick_size = serum::tick_size(
                    coin_lot_size,
                    pc_lot_size,
                    USDT_TOKEN_DECIMALS,
                    USDC_TOKEN_DECIMALS,
                );
                let (asks_price, _bids_price) = serum::find_best_ask_bid_price(&order_book_state)?;
                Ok(asks_price as f64 * usdt_usdc_tick_size)
            }
            AMMs::SOLUSDC => {
                let mut market_state = serum::load_serum_market(
                    rpc,
                    serum::sol_usdc_market(),
                    serum::mainnet_serum_program_id(),
                )?;
                let coin_lot_size = market_state.coin_lot_size;
                let pc_lot_size = market_state.pc_lot_size;

                let asks_key = Pubkey::new(cast_slice(&identity(market_state.asks) as &[_]));
                let bids_key = Pubkey::new(cast_slice(&identity(market_state.bids) as &[_]));

                let mut accounts = rpc.get_multiple_accounts(&[asks_key, bids_key])?;
                let asks_acct = std::mem::take(&mut accounts[0]);
                if asks_acct.is_none() {
                    return Err(UtilsError::AsksAccountIsNone.into());
                }
                let asks_acct = asks_acct.unwrap();
                let bids_acct = std::mem::take(&mut accounts[1]);
                if bids_acct.is_none() {
                    return Err(UtilsError::BidsAccountIsNone.into());
                }
                let bids_acct = bids_acct.unwrap();


                let mut asks_tuple = (asks_key, asks_acct);
                let asks_account = asks_tuple.into_account_info();

                let mut bids_tuple = (bids_key, bids_acct);
                let bids_account = bids_tuple.into_account_info();

                let mut asks = market_state.load_asks_mut(&asks_account)?;
                let mut bids = market_state.load_bids_mut(&bids_account)?;
                let order_book_state = OrderBookState {
                    market_state: &mut market_state,
                    asks: &mut asks,
                    bids: &mut bids,
                };

                let sol_usdc_tick_size = serum::tick_size(
                    coin_lot_size,
                    pc_lot_size,
                    WSOL_TOKEN_DECIMALS,
                    USDC_TOKEN_DECIMALS,
                );
                let (asks_price, _bids_price) = serum::find_best_ask_bid_price(&order_book_state)?;
                Ok(asks_price as f64 * sol_usdc_tick_size)
            }
            AMMs::SRMUSDC => {
                let mut market_state = serum::load_serum_market(
                    rpc,
                    serum::srm_usdc_market(),
                    serum::mainnet_serum_program_id(),
                )?;
                let coin_lot_size = market_state.coin_lot_size;
                let pc_lot_size = market_state.pc_lot_size;

                let asks_key = Pubkey::new(cast_slice(&identity(market_state.asks) as &[_]));
                let bids_key = Pubkey::new(cast_slice(&identity(market_state.bids) as &[_]));

                let mut accounts = rpc.get_multiple_accounts(&[asks_key, bids_key])?;
                let asks_acct = std::mem::take(&mut accounts[0]);
                if asks_acct.is_none() {
                    return Err(UtilsError::AsksAccountIsNone.into());
                }
                let asks_acct = asks_acct.unwrap();
                let bids_acct = std::mem::take(&mut accounts[1]);
                if bids_acct.is_none() {
                    return Err(UtilsError::BidsAccountIsNone.into());
                }
                let bids_acct = bids_acct.unwrap();

                let mut asks_tuple = (asks_key, asks_acct);
                let asks_account = asks_tuple.into_account_info();

                let mut bids_tuple = (bids_key, bids_acct);
                let bids_account = bids_tuple.into_account_info();

                let mut asks = market_state.load_asks_mut(&asks_account)?;
                let mut bids = market_state.load_bids_mut(&bids_account)?;
                let order_book_state = OrderBookState {
                    market_state: &mut market_state,
                    asks: &mut asks,
                    bids: &mut bids,
                };

                let srm_usdc_tick_size = serum::tick_size(
                    coin_lot_size,
                    pc_lot_size,
                    SRM_TOKEN_DECIMALS,
                    USDC_TOKEN_DECIMALS,
                );
                let (asks_price, _bids_price) = serum::find_best_ask_bid_price(&order_book_state)?;
                Ok(asks_price as f64 * srm_usdc_tick_size)
            }
        }
    }
    fn lp_token_price(token: AMMs, rpc: &Arc<RpcClient>) -> Result<f64> {
        let serum_market = AMMs::serum_market(&token);
        let (base_token_total, quote_token_total, mut market_state) = {
            let (open_orders, market_state) =
                serum::load_serum_open_orders_order_book_state(
                    rpc,
                    serum_market,
                    serum::mainnet_serum_program_id(),
                    AMMs::open_orders(token),
                )?;
            (
                open_orders.native_coin_total,
                open_orders.native_pc_total,
                market_state,
            )
        };
        let coin_lot_size = market_state.coin_lot_size;
        let pc_lot_size = market_state.pc_lot_size;

        let asks_key = Pubkey::new(cast_slice(&identity(market_state.asks) as &[_]));
        let bids_key = Pubkey::new(cast_slice(&identity(market_state.bids) as &[_]));
        let mut account_datas = rpc.get_multiple_accounts(&[asks_key, bids_key])?;
        let asks_acct = std::mem::take(&mut account_datas[0]);
        let bids_acct = std::mem::take(&mut account_datas[1]);
        if asks_acct.is_none() {
            return Err(UtilsError::AsksAccountIsNone.into());
        }
        if bids_acct.is_none() {
            return Err(UtilsError::BidsAccountIsNone.into());
        }
        let asks_acct = asks_acct.unwrap();
        let bids_acct = bids_acct.unwrap();

        let mut asks_tuple = (asks_key, asks_acct);
        let asks_account = asks_tuple.into_account_info();

        let mut bids_tuple = (bids_key, bids_acct);
        let bids_account = bids_tuple.into_account_info();
        let mut asks = market_state.load_asks_mut(&asks_account)?;
        let mut bids = market_state.load_bids_mut(&bids_account)?;
        let order_book_state = OrderBookState {
            market_state: &mut market_state,
            asks: &mut asks,
            bids: &mut bids,
        };
        let (
            need_take_pnl_coin, 
            need_take_pnl_pc, 
            lp_mint_address, 
            pool_coin_token_account,
            pool_pc_token_account,
        ) = match AmmLayouts::from(token) {
            AmmLayouts::V3 => {
                let layout = AmmInfoLayoutV3::load(rpc, AMMs::amm_id(token))?;
                (
                    layout.need_take_pnl_coin,
                    layout.need_take_pnl_pc,
                    layout.lp_mint_address,
                    layout.pool_coin_token_account,
                    layout.pool_pc_token_account
                )
            },
            AmmLayouts::V4 => {
                let layout = AmmInfoLayoutV4::load(rpc, AMMs::amm_id(token))?;
                (
                    layout.need_take_pnl_coin,
                    layout.need_take_pnl_pc,
                    layout.lp_mint_address,
                    layout.pool_coin_token_account,
                    layout.pool_pc_token_account
                )
            }
        };
        let tick_size = serum::tick_size(
            coin_lot_size,
            pc_lot_size,
            AMMs::base_token_decimals(&token),
            AMMs::quote_token_decimals(&token),
        );

        let (asks_price, bids_price) = serum::find_best_ask_bid_price(&order_book_state)?;
        let asks_price = asks_price as f64 * tick_size;
        let _bids_price = bids_price as f64 * tick_size;
        let quote_price = 1_f64 / asks_price;

        let base_usd_price = AMMs::base_token_usd_price(token, rpc)?;
        let quote_usd_price = quote_price * base_usd_price;

        let mut token_accounts = rpc.get_multiple_accounts(&[
            lp_mint_address,
            pool_coin_token_account,
            pool_pc_token_account,
        ])?;
        if token_accounts.len() != 3 {
            return Err(UtilsError::InsufficientAccounts.into());
        }
        let lp_mint_account = std::mem::take(&mut token_accounts[0]);
        let pool_coin_token_account = std::mem::take(&mut token_accounts[1]);
        let pool_pc_token_account = std::mem::take(&mut token_accounts[2]);
        if lp_mint_account.is_none() {
            return Err(UtilsError::TokenAccountISNone("failed to get lp token mint").into());
        }
        let lp_mint_account = lp_mint_account.unwrap();
        if pool_coin_token_account.is_none() {
            return Err(UtilsError::TokenAccountISNone("failed to get coin token account").into());
        }
        let pool_coin_token_account = pool_coin_token_account.unwrap();
        if pool_pc_token_account.is_none() {
            return Err(UtilsError::TokenAccountISNone("failed to get pc token account").into());
        }
        let pool_pc_token_account = pool_pc_token_account.unwrap();
        let lp_mint_account = MintAccount::unpack_unchecked(&lp_mint_account.data[..])?;
        let pool_coin_account =
            TokenAccount::unpack_unchecked(&pool_coin_token_account.data[..])?;
        let pool_pc_account =
            TokenAccount::unpack_unchecked(&pool_pc_token_account.data[..])?;
        let pool_pc_amount = pool_pc_account.amount;
        let pool_pc_amount = pool_pc_amount + quote_token_total;
        let pool_pc_amount = pool_pc_amount - need_take_pnl_pc;
        let pool_pc_amount = pool_pc_amount as f64 * quote_usd_price;
        let pool_pc_ui_amount =
        amount_to_ui_amount(pool_pc_amount as u64, AMMs::quote_token_decimals(&token));

        let pool_coin_amount = pool_coin_account.amount;
        let pool_coin_amount = pool_coin_amount + base_token_total;
        let pool_coin_amount = pool_coin_amount - need_take_pnl_coin;
        let pool_coin_amount = pool_coin_amount as f64 * base_usd_price;
        let pool_coin_ui_amount =
            amount_to_ui_amount(pool_coin_amount as u64, AMMs::base_token_decimals(&token));        

        let lp_token_supply_ui_amount =
            amount_to_ui_amount(lp_mint_account.supply, lp_mint_account.decimals);

        let lp_token_price =
            (pool_coin_ui_amount + pool_pc_ui_amount) / lp_token_supply_ui_amount;

        Ok(lp_token_price)
    }
    
}

#[derive(Debug, Clone, Copy)]
pub enum AmmLayouts {
    V3,
    V4,
}

impl From<AMMs> for AmmLayouts {
    fn from(amm: AMMs) -> AmmLayouts {
        match amm {
            AMMs::RAYUSDT | AMMs::SOLUSDC | AMMs::SRMUSDC | AMMs::USDTUSDC => AmmLayouts::V4,
            AMMs::RAYUSDC | AMMs::RAYSRM | AMMs::RAYSOL => AmmLayouts::V3,
        }
    }
}


impl AmmInfoLayoutV3 {
    pub fn load(rpc: &Arc<RpcClient>, amm_key: Pubkey) -> Result<AmmInfoLayoutV3> {
        let account_data = rpc.get_account_data(&amm_key)?;
        let layout = AmmInfoLayoutV3::unpack_from_slice(&account_data[..]);
        Ok(layout)
    }
    pub fn unpack_from_slice(src: &[u8]) -> AmmInfoLayoutV3 {
        const LEN: usize = 680;
        let input = array_ref![src, 0, LEN];
        let (
            status,
            nonce,
            order_num,
            depth,
            coin_decimals,
            pc_decimals,
            state,
            reset_flag,
            fee,
            min_separate,
            min_size,
            vol_max_cut_ratio,
            pnl_ratio,
            amount_wave_ratio,
            coin_lot_size,
            pc_lot_size,
            min_price_multiplier,
            max_price_multiplier,
            need_take_pnl_coin,
            need_take_pnl_pc,
            total_pnl_x,
            total_pnl_y,
            system_decimals_value,
            _blob,
            pool_coin_token_account,
            pool_pc_token_account,
            coin_mint_address,
            pc_mint_address,
            lp_mint_address,
            amm_open_orders,
            serum_market,
            serum_program_id,
            amm_target_orders,
            amm_quantities,
            pool_withdraw_queue,
            pool_temp_lp_token_account,
            amm_owner,
            pnl_owner,
            srm_token_account,
        ) = array_refs![
            input, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 16, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32
        ];
        let status = u64::from_le_bytes(*status);
        let nonce = u64::from_le_bytes(*nonce);
        let order_num = u64::from_le_bytes(*order_num);
        let depth = u64::from_le_bytes(*depth);
        let coin_decimals = u64::from_le_bytes(*coin_decimals);
        let pc_decimals = u64::from_le_bytes(*pc_decimals);
        let state = u64::from_le_bytes(*state);
        let reset_flag = u64::from_le_bytes(*reset_flag);
        let fee = u64::from_le_bytes(*fee);
        let min_separate = u64::from_le_bytes(*min_separate);
        let min_size = u64::from_le_bytes(*min_size);
        let vol_max_cut_ratio = u64::from_le_bytes(*vol_max_cut_ratio);
        let pnl_ratio = u64::from_le_bytes(*pnl_ratio);
        let amount_wave_ratio = u64::from_le_bytes(*amount_wave_ratio);
        let coin_lot_size = u64::from_le_bytes(*coin_lot_size);
        let pc_lot_size = u64::from_le_bytes(*pc_lot_size);
        let min_price_multiplier = u64::from_le_bytes(*min_price_multiplier);
        let max_price_multiplier = u64::from_le_bytes(*max_price_multiplier);
        let need_take_pnl_coin = u64::from_le_bytes(*need_take_pnl_coin);
        let need_take_pnl_pc = u64::from_le_bytes(*need_take_pnl_pc);
        let total_pnl_x = u64::from_le_bytes(*total_pnl_x);
        let total_pnl_y = u64::from_le_bytes(*total_pnl_y);
        let system_decimals_value = u64::from_le_bytes(*system_decimals_value);
        let pool_coin_token_account = Pubkey::new_from_array(*pool_coin_token_account);
        let pool_pc_token_account = Pubkey::new_from_array(*pool_pc_token_account);
        let coin_mint_address = Pubkey::new_from_array(*coin_mint_address);
        let pc_mint_address = Pubkey::new_from_array(*pc_mint_address);
        let lp_mint_address = Pubkey::new_from_array(*lp_mint_address);
        let amm_open_orders = Pubkey::new_from_array(*amm_open_orders);
        let serum_market = Pubkey::new_from_array(*serum_market);
        let serum_program_id = Pubkey::new_from_array(*serum_program_id);
        let amm_target_orders = Pubkey::new_from_array(*amm_target_orders);
        let amm_quantities = Pubkey::new_from_array(*amm_quantities);
        let pool_withdraw_queue = Pubkey::new_from_array(*pool_withdraw_queue);
        let pool_temp_lp_token_account = Pubkey::new_from_array(*pool_temp_lp_token_account);
        let amm_owner = Pubkey::new_from_array(*amm_owner);
        let pnl_owner = Pubkey::new_from_array(*pnl_owner);
        let srm_token_account = Pubkey::new_from_array(*srm_token_account);
        AmmInfoLayoutV3 {
            status,
            nonce,
            order_num,
            depth,
            coin_decimals,
            pc_decimals,
            state,
            reset_flag,
            fee,
            min_separate,
            min_size,
            vol_max_cut_ratio,
            pnl_ratio,
            amount_wave_ratio,
            coin_lot_size,
            pc_lot_size,
            min_price_multiplier,
            max_price_multiplier,
            need_take_pnl_coin,
            need_take_pnl_pc,
            total_pnl_x,
            total_pnl_y,
            system_decimals_value,
            pool_coin_token_account,
            pool_pc_token_account,
            coin_mint_address,
            pc_mint_address,
            lp_mint_address,
            amm_open_orders,
            serum_market,
            serum_program_id,
            amm_target_orders,
            amm_quantities,
            pool_withdraw_queue,
            pool_temp_lp_token_account,
            amm_owner,
            pnl_owner,
            srm_token_account,
        }
    }
}
impl AmmInfoLayoutV4 {
    pub fn load(rpc: &Arc<RpcClient>, amm_key: Pubkey) -> Result<AmmInfoLayoutV4> {
        let account_data = rpc.get_account_data(&amm_key)?;
        let layout = AmmInfoLayoutV4::unpack_from_slice(&account_data[..]);
        Ok(layout)
    }
    pub fn unpack_from_slice(src: &[u8]) -> AmmInfoLayoutV4 {
        const LEN: usize = 752;
        let input = array_ref![src, 0, LEN];
        let (
            status,
            nonce,
            order_num,
            depth,
            coin_decimals,
            pc_decimals,
            state,
            reset_flag,
            min_size,
            vol_max_cut_ratio,
            amount_wave_ratio,
            coin_lot_size,
            pc_lot_size,
            min_price_multiplier,
            max_price_multiplier,
            system_decimals_value,
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
            need_take_pnl_coin,
            need_take_pnl_pc,
            total_pnl_pc,
            total_pnl_coin,
            pool_total_deposit_pc,
            pool_total_deposit_coin,
            swap_coin_in_amount,
            swap_pc_out_amount,
            swap_coin_2_pc_fee,
            swap_pc_in_amount,
            swap_coin_out_amount,
            swap_pc_2_coin_fee,
            pool_coin_token_account,
            pool_pc_token_account,
            coin_mint_address,
            pc_mint_address,
            lp_mint_address,
            amm_open_orders,
            serum_market,
            serum_program_id,
            amm_target_orders,
            pool_withdraw_queue,
            pool_temp_lp_token_account,
            amm_owner,
            pnl_owner,
        ) = array_refs![
            input, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 16, 16, 16, 16, 8, 16, 16, 8, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32
        ];
        let status = u64::from_le_bytes(*status);
        let nonce = u64::from_le_bytes(*nonce);
        let order_num = u64::from_le_bytes(*order_num);
        let depth = u64::from_le_bytes(*depth);
        let coin_decimals = u64::from_le_bytes(*coin_decimals);
        let pc_decimals = u64::from_le_bytes(*pc_decimals);
        let state = u64::from_le_bytes(*state);
        let reset_flag = u64::from_le_bytes(*reset_flag);
        let min_size = u64::from_le_bytes(*min_size);
        let vol_max_cut_ratio = u64::from_le_bytes(*vol_max_cut_ratio);
        let amount_wave_ratio = u64::from_le_bytes(*amount_wave_ratio);
        let coin_lot_size = u64::from_le_bytes(*coin_lot_size);
        let pc_lot_size = u64::from_le_bytes(*pc_lot_size);
        let min_price_multiplier = u64::from_le_bytes(*min_price_multiplier);
        let max_price_multiplier = u64::from_le_bytes(*max_price_multiplier);
        let system_decimals_value = u64::from_le_bytes(*system_decimals_value);
        let min_separate_numerator = u64::from_le_bytes(*min_separate_numerator);
        let min_separate_denominator = u64::from_le_bytes(*min_separate_denominator);
        let trade_fee_numerator = u64::from_le_bytes(*trade_fee_numerator);
        let trade_fee_denominator = u64::from_le_bytes(*trade_fee_denominator);
        let pnl_numerator = u64::from_le_bytes(*pnl_numerator);
        let pnl_denominator = u64::from_le_bytes(*pnl_denominator);
        let swap_fee_numerator = u64::from_le_bytes(*swap_fee_numerator);
        let swap_fee_denominator = u64::from_le_bytes(*swap_fee_denominator);
        let need_take_pnl_coin = u64::from_le_bytes(*need_take_pnl_coin);
        let need_take_pnl_pc = u64::from_le_bytes(*need_take_pnl_pc);
        let total_pnl_pc = u64::from_le_bytes(*total_pnl_pc);
        let total_pnl_coin = u64::from_le_bytes(*total_pnl_coin);
        let pool_total_deposit_pc = u128::from_le_bytes(*pool_total_deposit_pc);
        let pool_total_deposit_coin = u128::from_le_bytes(*pool_total_deposit_coin);
        let swap_coin_in_amount = u128::from_le_bytes(*swap_coin_in_amount);
        let swap_coin_2_pc_fee = u64::from_le_bytes(*swap_coin_2_pc_fee);
        let swap_pc_in_amount = u128::from_le_bytes(*swap_pc_in_amount);
        let swap_coin_out_amount = u128::from_le_bytes(*swap_coin_out_amount);
        let swap_pc_out_amount = u128::from_le_bytes(*swap_pc_out_amount);
        let swap_pc_2_coin_fee = u64::from_le_bytes(*swap_pc_2_coin_fee);
        let pool_coin_token_account = Pubkey::new_from_array(*pool_coin_token_account);
        let pool_pc_token_account = Pubkey::new_from_array(*pool_pc_token_account);
        let coin_mint_address = Pubkey::new_from_array(*coin_mint_address);
        let pc_mint_address = Pubkey::new_from_array(*pc_mint_address);
        let lp_mint_address = Pubkey::new_from_array(*lp_mint_address);
        let amm_open_orders = Pubkey::new_from_array(*amm_open_orders);
        let serum_market = Pubkey::new_from_array(*serum_market);
        let serum_program_id = Pubkey::new_from_array(*serum_program_id);
        let amm_target_orders = Pubkey::new_from_array(*amm_target_orders);
        let pool_withdraw_queue = Pubkey::new_from_array(*pool_withdraw_queue);
        let pool_temp_lp_token_account = Pubkey::new_from_array(*pool_temp_lp_token_account);
        let amm_owner = Pubkey::new_from_array(*amm_owner);
        let pnl_owner = Pubkey::new_from_array(*pnl_owner);
        AmmInfoLayoutV4 {
            status,
            nonce,
            order_num,
            depth,
            coin_decimals,
            pc_decimals,
            state,
            reset_flag,
            min_size,
            vol_max_cut_ratio,
            amount_wave_ratio,
            coin_lot_size,
            pc_lot_size,
            min_price_multiplier,
            max_price_multiplier,
            system_decimals_value,
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
            need_take_pnl_coin,
            need_take_pnl_pc,
            total_pnl_pc,
            total_pnl_coin,
            pool_total_deposit_pc,
            pool_total_deposit_coin,
            swap_coin_in_amount,
            swap_pc_out_amount,
            swap_coin_2_pc_fee,
            swap_pc_in_amount,
            swap_coin_out_amount,
            swap_pc_2_coin_fee,
            pool_coin_token_account,
            pool_pc_token_account,
            coin_mint_address,
            pc_mint_address,
            lp_mint_address,
            amm_open_orders,
            serum_market,
            serum_program_id,
            amm_target_orders,
            pool_withdraw_queue,
            pool_temp_lp_token_account,
            amm_owner,
            pnl_owner,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anchor_client::{solana_client::rpc_client::RpcClient, Cluster};

    #[test]
    pub fn test_ray_sol_lp_token_price() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let err = AMMs::calculate_lp_token_price(AMMs::RAYSOL, &rpc);
        assert!(err.is_err() == false, "{:#?}", err.err());
        println!("RAY-SOL lp token price {}", err.unwrap());
    }

    #[test]
    pub fn test_ray_srm_lp_token_price() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let err = AMMs::calculate_lp_token_price(AMMs::RAYSRM, &rpc);
        assert!(err.is_err() == false, "{:#?}", err.err());
        println!("RAY-SRM lp token price {}", err.unwrap());
    }

    #[test]
    pub fn test_ray_usdc_lp_token_price() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let err = AMMs::calculate_lp_token_price(AMMs::RAYUSDC, &rpc);
        assert!(err.is_err() == false, "{:#?}", err.err());
        println!("RAY-USDC lp token price {}", err.unwrap());
    }

    #[test]
    pub fn test_ray_usdt_lp_token_price() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let err = AMMs::calculate_lp_token_price(AMMs::RAYUSDT, &rpc);
        assert!(err.is_err() == false, "{:#?}", err.err());
        println!("RAY-USDT lp token price {}", err.unwrap());
    }

    #[test]
    pub fn test_ray_sol_amm_v3() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let _v3 =
            AmmInfoLayoutV3::load(&rpc, AMMs::amm_id(AMMs::RAYSOL)).expect("failed to load layout");
    }

    #[test]
    pub fn test_ray_srm_amm_v3() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let _v3 =
            AmmInfoLayoutV3::load(&rpc, AMMs::amm_id(AMMs::RAYSRM)).expect("failed to load layout");
    }

    #[test]
    pub fn test_ray_usdc_amm_v3() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let _v3 = AmmInfoLayoutV3::load(&rpc, AMMs::amm_id(AMMs::RAYUSDC))
            .expect("failed to load layout");
    }

    #[test]
    pub fn test_ray_usdt_amm_v4() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let _v4 = AmmInfoLayoutV4::load(&rpc, AMMs::amm_id(AMMs::RAYUSDT))
            .expect("failed to load layour");
    }

    #[test]
    pub fn test_sol_usdc_amm_v4() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let _v4 = AmmInfoLayoutV4::load(&rpc, AMMs::amm_id(AMMs::SOLUSDC))
            .expect("failed to load layour");
    }

    #[test]
    pub fn test_sol_usdc_base_token_price() {
        let cluster = Cluster::Custom(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ws://api.mainnet-beta.solana.com".to_string(),
        );
        let rpc = Arc::new(RpcClient::new(cluster.url().to_string()));
        let sol_usd_price =
            AMMs::base_token_usd_price(AMMs::SOLUSDC, &rpc).expect("failed to parse sol usd price");
        println!("sol usd price {}", sol_usd_price);
    }
}

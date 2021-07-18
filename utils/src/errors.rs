use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError<'a> {
    #[error("retrieve token account is none {0}")]
    TokenAccountISNone(&'a str),
    #[error("failed to find asks min")]
    FindAsksMin,
    #[error("failed to find bids max")]
    FindBidsMax,
    #[error("failed to get asks node")]
    GetAsksNode,
    #[error("failed to get asks node as leaf node")]
    AsksNodeAsLeaf,
    #[error("failed to get bids node")]
    GetBidsNode,
    #[error("failed to get bids node as leaf node")]
    BidsNodeAsLeaf,
    #[error("asks account is none")]
    AsksAccountIsNone,
    #[error("bids account is none")]
    BidsAccountIsNone,
    #[error("insufficient number of accounts")]
    InsufficientAccounts,
    #[error("serum market account is none")]
    MarketAccountISNone,
    #[error("serum market open orders account is none")]
    OpenOrdersAccountIsNone,
    #[error("sysvar rent account is none")]
    RentAccountIsNone,
}

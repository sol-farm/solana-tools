use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("retrieve token account is none")]
    TokenAccountISNone,
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
}

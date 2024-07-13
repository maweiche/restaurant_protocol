use anchor_lang::error_code;

#[error_code]
pub enum SetupError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
}
#[error_code]
pub enum ProtocolError {
    #[msg("The Protocol is locked, you can't perform this action")]
    ProtocolLocked,
    #[msg("You are not authorized to perform this action")]
    UnauthorizedAdmin,
    #[msg("Airdrop instructions not correct")]
    InstructionsNotCorrect,
    #[msg("Invalid Sale Time")]
    InvalidSaleTime,
    #[msg("Invalid Max Supply")]
    InvalidMaxSupply,
    #[msg("Invalid Price")]
    InvalidPrice,
    #[msg("Mint Count did not increment")]
    InvalidMintCount,
    #[msg("Invalid Balance of Token Pre Mint")]
    InvalidBalancePreMint,
    #[msg("Invalid Balance of Token Post Mint")]
    InvalidBalancePostMint,
    #[msg("Total Supply not increased")]
    TotalSupplyNotIncreased,
    #[msg("Invalid balance pre burn")]
    InvalidBalancePreBurn,
    #[msg("Invalid balance post burn")]
    InvalidBalancePostBurn,
}
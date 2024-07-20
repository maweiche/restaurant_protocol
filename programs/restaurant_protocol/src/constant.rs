use anchor_lang::declare_id;

pub mod multisig_wallet {
    use super::*;
    declare_id!("6KuX26FZqzqpsHDLfkXoBXbQRPEDEbstqNiPBKHNJQ9e");
}

pub mod admin_wallet {
    use super::*;
    declare_id!("ADM12HQ5G2EzSwWy2nN1xXMyGjaBULuuX9GTgW2FPwZK");
}

pub const ED25519_PROGRAM_ID: &str = "Ed25519SigVerify111111111111111111111111111";
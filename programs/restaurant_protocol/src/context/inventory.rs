use anchor_lang::prelude::*;
use crate::{
    state::{
        Admin,
        Inventory,
        Protocol
    },
    constant,
    errors::{SetupError, ProtocolError},
};

impl<'info> InventoryAdd<'info> {
    pub fn add(
        &mut self,
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        stock: f64,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin_state.is_some() || self.admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
        
        self.inventory_state.set_inner(Inventory {
            sku,
            category,
            name,
            price,
            stock,
            last_order: 0,
        });

        Ok(())
    }
}

impl<'info> InventoryRemove<'info> {
    pub fn remove(
        &mut self
    ) -> Result<()> {

        /*
        
            Remove Admin Ix:

            Some security check:
            - Check if the account signing is the primary admin from the multisig wallet.

            What the Instruction does:
            - Closes the Admin_State account which is necessary for Admin rights, this is intended to only be used when the admin is compromised.
            - Returns any account rent of the Admin_State account to the multisig wallet.   

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
    
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct InventoryAdd<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", admin.key().as_ref()],
        bump
    )]
    pub admin_state: Option<Account<'info, Admin>>,
    pub inventory_item: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = Inventory::INIT_SPACE + 5,
        seeds = [b"inventory_state", inventory_item.key().as_ref()],
        bump
    )]
    pub inventory_state: Account<'info, Inventory>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InventoryRemove<'info> {
    /// CHECK: This is the inventory being removed, it's ok because the signer will be required to be the overall authority on program
    #[account(mut)]
    pub inventory_item: AccountInfo<'info>,
    #[account(
        mut,
        close = admin, // this is where the account rent funds will be sent to after the admin is removed
        seeds = [b"inventory_state", inventory_item.key().as_ref()],
        bump
    )]
    pub inventory_state: Account<'info, Inventory>,
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
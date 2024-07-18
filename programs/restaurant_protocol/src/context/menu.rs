use anchor_lang::prelude::*;
use crate::{
    state::{
        MenuItem,
        Protocol
    },
    constant,
    errors::{SetupError, ProtocolError},
};

impl<'info> Menu<'info> {
    pub fn add(
        &mut self,
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        ingredients: Vec<String>,
        active: bool,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
        
        self.menu_state.set_inner(MenuItem {
            sku,
            category,
            name,
            price,
            ingredients,
            active,
        });

        Ok(())
    }

    pub fn update(
        &mut self,
        active: bool,
    ) -> Result<()> {

        /*
        
            Update Menu Item Ix:

            Some security check:
            - Check if the account signing is the primary admin from the multisig wallet.

            What the Instruction does:
            - Updates the active status of the menu item.   

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
        
        self.menu_state.active = active;

        Ok(())
    }

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
pub struct Menu<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    pub menu_item: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = MenuItem::INIT_SPACE + 5,
        seeds = [b"menu_state", menu_item.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub menu_state: Account<'info, MenuItem>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
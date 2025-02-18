use anchor_lang::prelude::*;
use crate::{
    state::{
        RestaurantAdmin,
        InventoryItem,
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
        require!(self.restaurant_admin_state.restaurant.key() == *self.restaurant.key, SetupError::Unauthorized);
        
        self.inventory_state.set_inner(InventoryItem {
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

impl<'info> InventoryUpdate<'info> {
    pub fn update(
        &mut self,
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        stock: f64,
        last_order: u64
    ) -> Result<()> {

        /*
        

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.restaurant_admin_state.restaurant.key() == *self.restaurant.key, SetupError::Unauthorized);
        
        self.inventory_state.set_inner(InventoryItem {
            sku,
            category,
            name,
            price,
            stock,
            last_order
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
        require!(self.restaurant_admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
    
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct InventoryAdd<'info> {
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", restaurant_admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    pub inventory_item: SystemAccount<'info>,
    #[account(
        init,
        payer = restaurant_admin,
        space = InventoryItem::INIT_SPACE + 5,
        seeds = [b"inventory_state", inventory_item.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub inventory_state: Account<'info, InventoryItem>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InventoryUpdate<'info> {
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", restaurant_admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    pub inventory_item: SystemAccount<'info>,
    #[account(
        seeds = [b"inventory_state", inventory_item.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub inventory_state: Account<'info, InventoryItem>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InventoryRemove<'info> {
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    pub inventory_item: SystemAccount<'info>,
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"inventory_state", inventory_item.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub inventory_state: Account<'info, InventoryItem>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
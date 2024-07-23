use anchor_lang::prelude::*;
use crate::{
    state::{
        RestaurantAdmin,
        MenuItem,
        Protocol
    },
    constant,
    errors::{SetupError, ProtocolError},
};

impl<'info> MenuInit<'info> {
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
        require!(self.restaurant_admin_state.restaurant.key() == *self.restaurant.key, SetupError::Unauthorized);
        
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
}

impl<'info> MenuUpdate<'info> {
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
        require!(self.restaurant_admin_state.restaurant.key() == *self.restaurant.key, SetupError::Unauthorized);
        
        self.menu_state.active = active;

        Ok(())
    }
}

impl<'info> MenuRemove<'info> {
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
        require!(self.restaurant_admin_state.restaurant.key() == *self.restaurant.key, SetupError::Unauthorized);
    
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct MenuInit<'info> {
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
    pub menu_item: SystemAccount<'info>,
    #[account(
        init,
        payer = restaurant_admin,
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

#[derive(Accounts)]
pub struct MenuUpdate<'info> {
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
    pub menu_item: SystemAccount<'info>,
    #[account(
        mut,
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

#[derive(Accounts)]
pub struct MenuRemove<'info> {
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
    pub menu_item: SystemAccount<'info>,
    #[account(
        mut,
        close = restaurant_admin,
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
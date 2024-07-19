use anchor_lang::prelude::*;
use crate::{
    state::{
        Admin,
        Protocol,
        RestaurantAdmin
    },
    constant,
    errors::{SetupError, ProtocolError},
};

impl<'info> AdminInit<'info> {
    pub fn initialize_admin(
        &mut self,
        username: String,
    ) -> Result<()> {

        /*
        
            Create a new Admin Ix:

            Some security check:
            - Check if the account that is initializing the admin is the admin itself, if not 
            it should be the mutlisig account that is the admin of the enitre protocol.
            - Save the Time of initialization to render it useless for the first 12h of initialization.

            What the Instruction does:
            - Initialize the new admin account with the username (so we can monitor who are the admin
            account atm in an easy way) and the publickey of the new admin.

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin_state.is_some() || self.admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
        
        self.new_admin_state.set_inner(Admin {
            publickey: self.new_admin.key(),
            username,
            initialized: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

impl<'info> AdminRemove<'info> {
    pub fn remove_admin(
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
        require!(self.primary_admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
    
        
        Ok(())
    }
}

impl<'info> RestaurantAdminInit<'info> {
    pub fn initialize_admin(
        &mut self,
        username: String,
    ) -> Result<()> {

        /*
        
            Create a new Admin Ix:

            Some security check:
            - Check if the account that is initializing the admin is the admin itself, if not 
            it should be the mutlisig account that is the admin of the enitre protocol.
            - Save the Time of initialization to render it useless for the first 12h of initialization.

            What the Instruction does:
            - Initialize the new admin account with the username (so we can monitor who are the admin
            account atm in an easy way) and the publickey of the new admin.

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.restaurant_owner.key() != self.restaurant.owner.key(), SetupError::Unauthorized);
        
        self.restaurant_admin_state.set_inner(RestaurantAdmin {
            publickey: self.restaurant_admin.key(),
            restaurant: *self.restaurant.key,
            username,
            initialized: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

impl<'info> RestaurantAdminRemove<'info> {
    pub fn remove_admin(
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
        require!(self.restaurant_owner.key() == self.restaurant.owner.key(), SetupError::Unauthorized);
    
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct AdminInit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", admin.key().as_ref()],
        bump
    )]
    pub admin_state: Option<Account<'info, Admin>>,
    pub new_admin: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = Admin::INIT_SPACE + 5,
        seeds = [b"admin_state", new_admin.key().as_ref()],
        bump
    )]
    pub new_admin_state: Account<'info, Admin>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminRemove<'info> {
    /// CHECK: This is the admin being removed, it's ok because the signer will be required to be the overall authority on program
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(
        mut,
        close = primary_admin, // this is where the account rent funds will be sent to after the admin is removed
        seeds = [b"admin_state", admin.key().as_ref()],
        bump
    )]
    pub admin_state: Account<'info, Admin>,
    pub primary_admin: Signer<'info>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct RestaurantAdminInit<'info> {
    #[account(mut)]
    pub restaurant_owner: Signer<'info>,
    #[account(mut)]
    pub restaurant: AccountInfo<'info>,
    #[account(mut)]
    pub restaurant_admin: AccountInfo<'info>,
    #[account(
        init,
        payer = restaurant_owner,
        space = Admin::INIT_SPACE + 5,
        seeds = [b"admin_state", restaurant_admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RestaurantAdminRemove<'info> {
    #[account(mut)]
    pub restaurant_owner: Signer<'info>,
    #[account(mut)]
    pub restaurant: AccountInfo<'info>,
    #[account(mut)]
    pub restaurant_admin: AccountInfo<'info>,
    #[account(
        mut,
        close = restaurant_owner,
        seeds = [b"admin_state", restaurant_admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
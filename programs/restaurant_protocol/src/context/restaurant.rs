use anchor_lang::{
    solana_program::{
        sysvar::rent::ID as RENT_ID,
        program::{invoke, invoke_signed}
    },
    prelude::*
};
pub use anchor_spl::token_2022::Token2022;
use crate::state::{Restaurant, Protocol, Admin};
use crate::errors::ProtocolError;
pub use spl_token_2022::{
    extension::ExtensionType,
    extension::group_pointer::instruction::initialize as initialize_group_pointer,
};

impl<'info> RestaurantInit<'info> {
    pub fn add(
        &mut self,
        reference: Pubkey,
        name: String,
        symbol: String,
        url: String,
        bumps: RestaurantInitBumps,
    ) -> Result<()> {

        /*
        
            Create Collection Ix:

            Some security check:
            - The admin_state.publickey must match the signing admin.

            What these Instructions do:
            - Creates a Collection that can be used to mint NFTs.
        */

        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin_state.publickey == *self.admin.key, ProtocolError::UnauthorizedAdmin);

        // sanity check

       

        self.restaurant.set_inner(
            Restaurant {
                reference,
                name,
                symbol,
                owner: *self.owner.key,
                url,
                customer_count: 0,
            }
        );

        // Step 1: Initialize Account
        let size = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
            &[
                ExtensionType::GroupPointer
            ],
        ).unwrap();

        let rent = &Rent::from_account_info(&self.rent.to_account_info())?;
        let lamports = rent.minimum_balance(size );

        let collection_key = self.restaurant.key();
        let seeds: &[&[u8]; 3] = &[
            b"mint",
            collection_key.as_ref(),
            &[bumps.mint],
        ];
        let signer_seeds = &[&seeds[..]];

        invoke_signed(
            &solana_program::system_instruction::create_account(
                &self.admin.key(),
                &self.mint.key(),
                lamports,
                (size).try_into().unwrap(),
                &spl_token_2022::id(),
            ),
            &vec![
                self.admin.to_account_info(),
                self.mint.to_account_info(),
            ],
            signer_seeds
        )?;

        invoke(
            &initialize_group_pointer(
                &self.token_2022_program.key(),
                &self.mint.key(),
                Some(self.admin.key()),
                Some(self.mint.key()),
            )?,
            &vec![
                self.mint.to_account_info(),
            ],  
        )?;

        Ok(())
    }
}

impl<'info>RestaurantClose <'info> {
    pub fn close(&mut self) -> Result<()> {
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin_state.publickey == *self.admin.key, ProtocolError::UnauthorizedAdmin);


        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
    reference: Pubkey,
    name: String,
    symbol: String,
    owner: Pubkey,
    url: String,
)]
pub struct RestaurantInit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", admin.key().as_ref()],
        bump
    )]
    pub admin_state: Account<'info, Admin>,
    /// CHECK: this is ok because admin is setting up on owner behalf
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    #[account(
        init,
        seeds = [b"restaurant", restaurant.key().as_ref()],
        bump,
        payer = admin,
        space = Restaurant::INIT_SPACE + 54 + url.len() + name.len() + symbol.len() + 4 + 4
    )] 
    pub restaurant: Account<'info, Restaurant>,
    /// CHECK: this is fine since we are handling all the checks and creation in the program.
    #[account(
        mut,
        seeds = [b"mint", restaurant.key().as_ref()],
        bump
    )]
    pub mint: UncheckedAccount<'info>,
    #[account(address = RENT_ID)]
    /// CHECK: this is fine since we are hard coding the rent sysvar.
    pub rent: UncheckedAccount<'info>,
    pub token_2022_program: Program<'info, Token2022>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RestaurantClose<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", admin.key().as_ref()],
        bump
    )]
    pub admin_state: Account<'info, Admin>,
    /// CHECK: this is ok because admin is setting up on owner behalf
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    #[account(
        mut,
        close = admin,
        seeds = [b"restaurant", restaurant.key().as_ref()],
        bump,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token_2022::{
            Token2022, 
            spl_token_2022::{
                instruction::AuthorityType,
                state::Account as TokenAccount,
                extension::StateWithExtensions,
            }},
        associated_token::{AssociatedToken, Create, create},
        token::Token,  
        token_interface::{MintTo, mint_to, set_authority, SetAuthority},
    },
    solana_program::{system_instruction, program::invoke},
};
use crate::{
    state::{
        Customer,
        CustomerOrder,
        CustomerNft,
        Protocol,
        RestaurantAdmin
    },
    errors::ProtocolError,
};

impl<'info> OrderInit<'info> {
    pub fn add(
        &mut self,
        order_id: u64,
        total: f32,
        items: Vec<u64>,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);

        let amount_in_lamports = total as u64 * 1000000000;
        let transfer_instruction = system_instruction::transfer(
            &self.customer.key(),
            &self.restaurant.owner.key(),
            amount_in_lamports,
        );

        invoke(
            &transfer_instruction,
            &[
                self.customer.to_account_info(),
                self.restaurant_owner.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        let new_reward_amount = (total * 10.0) + self.customer_nft.reward_points as f32;

        self.customer_nft.reward_points = new_reward_amount as u64;
        
        self.order_state.set_inner(CustomerOrder {
            order_id,
            customer: self.customer.key(),
            items,
            total, // highest f16 number is 65504.0
            status: 0,
            created_at: Clock::get()?.unix_timestamp,
            updated_at: 0,
        });

        Ok(())
    }
}

impl<'info> OrderUpdate<'info> {
    pub fn update(
        &mut self,
        status: u8,
    ) -> Result<()> {

        /*
        

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.restaurant_admin_state.restaurant == self.restaurant.key(), ProtocolError::UnauthorizedAdmin);
        
        self.order_state.status = status;
        self.order_state.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

impl<'info> OrderCancel<'info> {
    pub fn cancel(
        &mut self,
    ) -> Result<()> {

        /*
        

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.customer.key() == self.order_state.customer, ProtocolError::UnauthorizedAdmin);
        
        self.order_state.status = 4;
        self.order_state.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

impl<'info> OrderClose<'info> {
    pub fn close(
        &mut self,
    ) -> Result<()> {

        /*
        

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.restaurant_admin_state.restaurant == self.restaurant.key(), ProtocolError::UnauthorizedAdmin);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
    order_id: u64,
    items: Vec<u64>,
)]
pub struct OrderInit<'info> {
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK
    pub restaurant_owner: AccountInfo<'info>,
    #[account(mut)]
    pub customer: Signer<'info>,
    #[account(
        seeds = [b"customer", customer.key().as_ref(), restaurant.key().as_ref()],
        bump,
    )]
    pub customer_profile: Account<'info, Customer>,
    #[account(
        mut,
        seeds = [b"member_nft", customer.key().as_ref(), restaurant.key().as_ref()],
        bump,
    )] 
    pub customer_nft: Account<'info, CustomerNft>,
    /// CHECK
    pub order: AccountInfo<'info>,
    #[account(
        init,
        payer = customer,
        space = CustomerOrder::INIT_SPACE + 5,
        seeds = [b"order_state", order.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub order_state: Account<'info, CustomerOrder>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OrderUpdate<'info> {
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"restaurant", restaurant_admin.key.as_ref(), restaurant.key().as_ref()],
        bump,
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    /// CHECK
    pub order: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"order_state", order.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub order_state: Account<'info, CustomerOrder>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OrderCancel<'info> {
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    #[account(mut)]
    pub customer: Signer<'info>,
    #[account(
        seeds = [b"customer", customer.key().as_ref(), restaurant.key().as_ref()],
        bump,
    )]
    pub customer_profile: Account<'info, Customer>,
    /// CHECK
    pub order: AccountInfo<'info>,
    #[account(
        mut,
        close = customer,
        seeds = [b"order_state", order.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub order_state: Account<'info, CustomerOrder>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OrderClose<'info> {
    #[account(mut)]
    /// CHECK
    pub restaurant: AccountInfo<'info>,
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"restaurant", restaurant_admin.key.as_ref(), restaurant.key().as_ref()],
        bump,
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    /// CHECK
    pub order: AccountInfo<'info>,
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"order_state", order.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub order_state: Account<'info, CustomerOrder>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
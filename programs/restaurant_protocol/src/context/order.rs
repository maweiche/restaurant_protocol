use anchor_lang::prelude::*;
use crate::{
    state::{
        Customer,
        CustomerOrder,
        Protocol
    },
    errors::ProtocolError,
};

impl<'info> Order<'info> {
    pub fn add(
        &mut self,
        order_id: u64,
        items: Vec<u64>,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        
        self.order_state.set_inner(CustomerOrder {
            order_id,
            customer: self.customer.key(),
            items,
            total: 0.0,
            status: 0,
            created_at: Clock::get()?.unix_timestamp,
            updated_at: 0,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
    order_id: u64,
    items: Vec<u64>,
)]
pub struct Order<'info> {
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
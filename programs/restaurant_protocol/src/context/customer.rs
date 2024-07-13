use anchor_lang::prelude::*;
use crate::{
    state::{
        Admin,
        Customer,
        Protocol
    },
    constant,
    errors::{SetupError, ProtocolError},
};

impl<'info> CustomerInit<'info> {
    pub fn add(
        &mut self,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        
        self.customer_profile.set_inner(Customer {
            publickey: self.customer.key(),
            reward_points: 0,
            initialized: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct CustomerInit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub customer: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = Customer::INIT_SPACE + 5,
        seeds = [b"customer", customer.key().as_ref()],
        bump
    )]
    pub customer_profile: Account<'info, Customer>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

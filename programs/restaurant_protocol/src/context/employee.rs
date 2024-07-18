use anchor_lang::prelude::*;
use crate::{
    state::{
        Admin,
        Employee,
        Protocol
    },
    constant,
    errors::{SetupError, ProtocolError},
};

impl<'info> EmployeeInit<'info> {
    pub fn initialize_employee(
        &mut self,
        username: String,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.admin_state.is_some() || self.admin.key() == constant::multisig_wallet::id(), SetupError::Unauthorized);
        
        self.employee_state.set_inner(Employee {
            publickey: self.employee.key(),
            restaurant: *self.restaurant.key,
            username,
            initialized: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

impl<'info> EmployeeRemove<'info> {
    pub fn remove_employee(
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
pub struct EmployeeInit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub admin_state: Option<Account<'info, Admin>>,
    pub employee: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = Employee::INIT_SPACE + 5,
        seeds = [b"employee_state", employee.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub employee_state: Account<'info, Employee>,
    #[account(mut)]
    /// CHECK: this is fine since we are hard coding the collection sysvar.
    pub restaurant: AccountInfo<'info>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmployeeRemove<'info> {
    /// CHECK: This is the employee being removed, it's ok because the signer will be required to be the overall authority on program
    #[account(mut)]
    pub employee: AccountInfo<'info>,
    #[account(
        mut,
        close = admin, // this is where the account rent funds will be sent to after the admin is removed
        seeds = [b"admin_state", admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub admin_state: Account<'info, Admin>,
    #[account(mut)]
    /// CHECK
    restaurant: AccountInfo<'info>,
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
use anchor_lang::prelude::*;
mod state;
mod errors;
mod constant;
mod context;
use context::*;

declare_id!("99vazimgMSgqzg3zLBGXhZjSZVBRvThChw49VVp9U39T");

#[program]
pub mod restaurant_protocol {
    use super::*;

    pub fn initialize_protocol_account(ctx: Context<ProtocolSetting>) -> Result<()> {
        ctx.accounts.initialize_protocol()
    }

    pub fn lock_protocol(ctx: Context<ProtocolSetting>) -> Result<()> {
        ctx.accounts.change_locked_setting()
    }

    pub fn initialize_admin_account(ctx: Context<AdminInit>, 
        username: String
    ) -> Result<()> {
        ctx.accounts.initialize_admin(username)
    }

    pub fn remove_admin_account(ctx: Context<AdminRemove>) -> Result<()> {
        ctx.accounts.remove_admin()
    }

    pub fn initialize_employee_account(ctx: Context<EmployeeInit>, 
        username: String
    ) -> Result<()> {
        ctx.accounts.initialize_employee(username)
    }

    pub fn remove_employee_account(ctx: Context<EmployeeRemove>) -> Result<()> {
        ctx.accounts.remove_employee()
    }

    pub fn add_inventory(ctx: Context<InventoryAdd>, 
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        stock: f64,
    ) -> Result<()> {
        ctx.accounts.add(sku, category, name, price, stock)
    }

    pub fn remove_inventory(ctx: Context<InventoryRemove>) -> Result<()> {
        ctx.accounts.remove()
    }
}
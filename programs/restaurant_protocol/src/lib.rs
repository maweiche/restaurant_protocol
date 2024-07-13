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

    pub fn add_inventory(ctx: Context<Inventory>, 
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        stock: f64,
    ) -> Result<()> {
        ctx.accounts.add(sku, category, name, price, stock)
    }

    pub fn remove_inventory(ctx: Context<Inventory>) -> Result<()> {
        ctx.accounts.remove()
    }

    pub fn add_menu_item(ctx: Context<Menu>, 
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        ingredients: Vec<String>,
        active: bool,
    ) -> Result<()> {
        ctx.accounts.add(sku, category, name, price, ingredients, active)
    }

    pub fn update_menu_item(ctx: Context<Menu>, 
        active: bool,
    ) -> Result<()> {
        ctx.accounts.update(active)
    }

    pub fn remove_menu_item(ctx: Context<Menu>) -> Result<()> {
        ctx.accounts.remove()
    }

    pub fn add_customer(ctx: Context<CustomerInit>) -> Result<()> {
        ctx.accounts.add()
    }

    pub fn add_order(ctx: Context<Order>, 
        order_id: u64,
        items: Vec<u64>,
    ) -> Result<()> {
        ctx.accounts.add(order_id, items)
    }
}
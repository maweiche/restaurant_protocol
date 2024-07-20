use anchor_lang::prelude::*;
mod state;
mod errors;
mod constant;
mod context;
use context::*;
use state::*;

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

    pub fn add_restaurant(ctx: Context<RestaurantInit>, 
        reference: Pubkey,
        name: String,
        symbol: String,
        url: String,
    ) -> Result<()> {
        ctx.accounts.add(reference, name, symbol, url, ctx.bumps)
    }

    pub fn initialize_restaurant_admin(ctx: Context<RestaurantAdminInit>, 
        username: String
    ) -> Result<()> {
        ctx.accounts.initialize_admin(username)
    }

    pub fn remove_restaurant_admin(ctx: Context<RestaurantAdminRemove>) -> Result<()> {
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

    pub fn update_inventory(ctx: Context<InventoryUpdate>, 
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        stock: f64,
        last_order: u64
    ) -> Result<()> {
        ctx.accounts.update(
            sku,
            category,
            name,
            price,
            stock,
            last_order
        )
    }

    pub fn remove_inventory(ctx: Context<InventoryRemove>) -> Result<()> {
        ctx.accounts.remove()
    }

    pub fn add_menu_item(ctx: Context<MenuInit>, 
        sku: u64,
        category: Pubkey,
        name: String,
        price: f64,
        ingredients: Vec<String>,
        active: bool,
    ) -> Result<()> {
        ctx.accounts.add(sku, category, name, price, ingredients, active)
    }

    pub fn update_menu_item(ctx: Context<MenuUpdate>, 
        active: bool,
    ) -> Result<()> {
        ctx.accounts.update(active)
    }

    pub fn remove_menu_item(ctx: Context<MenuRemove>) -> Result<()> {
        ctx.accounts.remove()
    }

    pub fn add_reward(ctx: Context<RewardInit>, 
        category: Pubkey,
        restaurant: Pubkey,
        reward_points: u64,
        reward_item: Pubkey,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.add(
            category,
            restaurant,
            reward_points,
            reward_item,
            uri,
            ctx.bumps
        )
    }

    pub fn remove_reward(ctx: Context<RewardRemove>) -> Result<()> {
        ctx.accounts.remove()
    }

    pub fn buy_reward(ctx: Context<RewardBuy>) -> Result<()> {
        ctx.accounts.buy(ctx.bumps)
    }

    pub fn airdrop_reward(ctx: Context<RewardAirdrop>) -> Result<()> {
        ctx.accounts.airdrop(ctx.bumps)
    }

    pub fn add_customer(ctx: Context<CustomerInit>,
        id: u64,
        uri: String,
        attributes: Vec<Attributes>
    ) -> Result<()> {
        ctx.accounts.add(id, uri, attributes, ctx.bumps)
    }

    pub fn add_order(ctx: Context<OrderInit>, 
        order_id: u64,
        total: f32,
        items: Vec<u64>,
        
    ) -> Result<()> {
        ctx.accounts.add(order_id, total, items)
    }

    pub fn update_order(ctx: Context<OrderUpdate>, 
        status: u8,
    ) -> Result<()> {
        ctx.accounts.update(status)
    }

    pub fn cancel_order(ctx: Context<OrderCancel>) -> Result<()> {
        ctx.accounts.cancel()
    }

    pub fn close_order(ctx: Context<OrderClose>) -> Result<()> {
        ctx.accounts.close()
    }
}
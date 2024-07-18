use anchor_lang::prelude::*;

// Setup State
#[account]
pub struct Protocol {
    pub locked: bool,
}

impl Space for Protocol {
    const INIT_SPACE: usize = 8 + 1;
}

#[account]
pub struct Restaurant {
    pub reference: Pubkey, // how we will sort
    pub name: String,
    pub symbol: String,
    pub owner: Pubkey,
    pub url: String,
    pub customer_count: u32,
}

impl Space for Restaurant {
    const INIT_SPACE: usize = 8 + 32 + 4 + 32 + 4 + 32 + 4 + 32 + 4 + 4;
}

#[account]
pub struct Admin {
    pub publickey: Pubkey,
    pub username: String,
    pub initialized: i64,
}

impl Space for Admin {
    const INIT_SPACE: usize = 8 + 32 + 4 + 8;
}

#[account]
pub struct Employee {
    pub publickey: Pubkey,
    pub username: String,
    pub initialized: i64,
}

impl Space for Employee {
    const INIT_SPACE: usize = 8 + 32 + 4 + 8;
}

#[account]
pub struct InventoryItem {
    pub sku: u64,              // Stock Keeping Unit -- how we identify the product
    pub category: Pubkey,      // Category of the product -- stored as public key for easy sorting and filtering
    pub name: String,          // Name of the product -- what the product is called
    pub price: f64,            // Price of the product -- how much it costs for ordering
    pub stock: f64,            // Stock of the product -- how many units are available, will be updated as orders are made
    pub last_order: u64,       // Last time the product was ordered -- stored as unix timestamp
}

impl Space for InventoryItem {
    const INIT_SPACE: usize = 8 + 32 + 4 + 8 + 8 + 8;
}

#[account]
pub struct MenuItem {
    pub sku: u64,              // Stock Keeping Unit -- how we identify the product
    pub category: Pubkey,      // Category of the product -- stored as public key for easy sorting and filtering
    pub name: String,          // Name of the product -- what the product is called
    pub price: f64,            // Price of the product -- how much it costs for ordering
    pub ingredients: Vec<String>, // Ingredients of the product -- what is used to make the product, this is what will be used to deduct from the inventory
    pub active: bool,          // Active status of the product -- whether it is available for ordering
}

impl Space for MenuItem {
    const INIT_SPACE: usize = 8 + 32 + 4 + 8 + 8 + 8 + 8;
}

#[account]
pub struct Customer {
    pub id: u64,
    pub restaurant: Pubkey,
    pub publickey: Pubkey,
    pub customer_nft: Pubkey,
    pub member_since: i64,
}

impl Space for Customer {
    const INIT_SPACE: usize = 8 + 8 + 8;
}

#[account]
pub struct CustomerNft {
    pub id: u64,
    pub reward_points: u64,
}

impl Space for CustomerNft {
    const INIT_SPACE: usize = 8 + 8 + 32 + 4 + 2 + 32 + 2 + 8;
}

#[account]
pub struct CustomerOrder {
    pub order_id: u64,         // Order ID -- unique identifier for the order
    pub customer: Pubkey,      // Customer of the order -- who made the order
    pub items: Vec<u64>,       // Items in the order -- what products were ordered, skus of the products
    pub total: f64,            // Total of the order -- how much the order costs
    pub status: u8,            // Status of the order -- what state the order is in (0: pending, 1: completed, 2: cancelled)
    pub created_at: i64,       // Created at -- when the order was made, stored as unix timestamp
    pub updated_at: i64,       // Updated at -- when the order was last updated, stored as unix timestamp
}

impl Space for CustomerOrder {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 8 + 8;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Attributes {
    pub key: String,
    pub value: String,
}
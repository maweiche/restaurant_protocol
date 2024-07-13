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
pub struct Inventory {
    pub sku: u64,              // Stock Keeping Unit -- how we identify the product
    pub category: Pubkey,      // Category of the product -- stored as public key for easy sorting and filtering
    pub name: String,          // Name of the product -- what the product is called
    pub price: f64,            // Price of the product -- how much it costs for ordering
    pub stock: f64,            // Stock of the product -- how many units are available, will be updated as orders are made
    pub last_order: u64,       // Last time the product was ordered -- stored as unix timestamp
}

impl Space for Inventory {
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
use crate::{
    state::{
        Admin,
        Restaurant,
        Customer,
        Protocol,
        CustomerNft,
        Attributes
    },
    constant,
    errors::{SetupError, ProtocolError},
};

pub use anchor_lang::{
    solana_program::{
        sysvar::rent::ID as RENT_ID,
        program::{invoke, invoke_signed}
    },
    prelude::*
};
pub use anchor_spl::{
    token_2022::{
        Token2022, 
        spl_token_2022::{
            instruction::AuthorityType,
            state::Account as TokenAccount,
            extension::StateWithExtensions,
        }},
    associated_token::{AssociatedToken, Create, create},  
    token::Token,
    token_interface::{MintTo, mint_to, set_authority, SetAuthority}
};
pub use spl_token_2022::{
    extension::ExtensionType,
    instruction::initialize_mint2,
    extension::metadata_pointer::instruction::initialize as initialize_metadata_pointer,
    extension::group_member_pointer::instruction::initialize as initialize_group_member_pointer,
};
pub use spl_token_metadata_interface::{
    state::{TokenMetadata, Field},
    instruction::{initialize as initialize_metadata_account, update_field as update_metadata_account},
};


impl<'info> CustomerInit<'info> {
    pub fn add(
        &mut self,
        id: u64, 
        uri: String, 
        attributes: Vec<Attributes>,
        bumps: CustomerInitBumps,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        
        self.customer_profile.set_inner(Customer {
            id,
            restaurant: self.restaurant.key(),
            publickey: self.customer.key(),
            customer_nft: self.mint.key(),
            member_since: Clock::get()?.unix_timestamp,
        });

        // Step 1: Initialize Account
        let size = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
            &[
                ExtensionType::GroupMemberPointer,
                ExtensionType::PermanentDelegate,
                ExtensionType::MetadataPointer,
            ],
        ).unwrap();

        // let attributes: Vec<Attributes> = vec![
        //     Attributes {
        //         key: "id".to_string(),
        //         value: self.customer.key().to_string(), // bytes: 4 + 32
        //     },
        //     Attributes {
        //         key: "restaurant".to_string(),
        //         value: self.restaurant.name.to_string(),
        //     },
        //     Attributes {
        //         key: "owner".to_string(),
        //         value: self.customer.key().to_string(),
        //     },
        //     Attributes {
        //         key: "username".to_string(),
        //         value: username,
        //     },
        //     Attributes {
        //         key: "member since".to_string(),
        //         value: Clock::get()?.unix_timestamp.to_string(),
        //     },
        //     Attributes {
        //         key: "lifetime reward points".to_string(),
        //         value: "0".to_string(),
        //     },
        //     Attributes {
        //         key: "current reward points".to_string(),
        //         value: "0".to_string(),
        //     }
        // ];

        let metadata = TokenMetadata {
            update_authority: spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(Some(self.auth.key())).unwrap(),
            mint: self.mint.key(),
            name: self.restaurant.name.to_string() + " Customer Membership",
            symbol: self.restaurant.symbol.to_string(),
            uri,
            additional_metadata: attributes.into_iter().map(|attr| (attr.key, attr.value)).collect(),
        };

        let extension_extra_space = metadata.tlv_size_of().unwrap();
        let rent = &Rent::from_account_info(&self.rent.to_account_info())?;
        let lamports = rent.minimum_balance(size + extension_extra_space);

        let nft_key = self.customer_nft.key();
        let seeds: &[&[u8]; 3] = &[
            b"mint",
            nft_key.as_ref(),
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

         // 2.3: Add group member pointer
         invoke(
            &initialize_group_member_pointer(
                &self.token_2022_program.key(),
                &self.mint.key(),
                Some(self.auth.key()),
                Some(self.restaurant_mint.key()), 
            )?,
            &vec![
                self.mint.to_account_info(),
            ],  
        )?;
        
        // 2.4: Metadata Pointer
        invoke(
            &initialize_metadata_pointer(
            &self.token_2022_program.key(),
            &self.mint.key(),
            Some(self.auth.key()),
            Some(self.mint.key()),
            )?,
            &vec![
            self.mint.to_account_info(),
            ],
        )?;

        // Step 3: Initialize Mint & Metadata Account
        invoke_signed(
            &initialize_mint2(
                &self.token_2022_program.key(),
                &self.mint.key(),
                &self.auth.key(),
                None,
                0,
            )?,
            &vec![
                self.mint.to_account_info(),
            ],
            signer_seeds
        )?;

        let seeds: &[&[u8]; 2] = &[
            b"auth",
            &[bumps.auth],
        ];
        let signer_seeds = &[&seeds[..]];

        invoke_signed(
            &initialize_metadata_account(
                &self.token_2022_program.key(),
                &self.mint.key(),
                &self.auth.key(),
                &self.mint.key(),
                &self.auth.key(),
                metadata.name,
                metadata.symbol,
                metadata.uri,
            ),
            &vec![
                self.mint.to_account_info(),
                self.auth.to_account_info(),
            ],
            signer_seeds
        )?;

        for (field, value) in metadata.additional_metadata.into_iter() {
            invoke_signed(
                &update_metadata_account(
                    &self.token_2022_program.key(),
                    &self.mint.key(),
                    &self.auth.key(),
                    Field::Key(field),
                    value,
                ),
                &vec![
                    self.mint.to_account_info(),
                    self.auth.to_account_info(),
                ],
                signer_seeds
            )?;
        }

        // Initialize ATA if it doesn't exist
        if self.customer_mint_ata.owner != &self.customer.key() {
            create(
            CpiContext::new(
                self.token_2022_program.to_account_info(),
                Create {
                payer: self.admin.to_account_info(), // payer
                associated_token: self.customer_mint_ata.to_account_info(),
                authority: self.customer.to_account_info(), // owner
                mint: self.mint.to_account_info(),
                system_program: self.system_program.to_account_info(),
                token_program: self.token_2022_program.to_account_info(),
                }
            ),
            )?;
        }
        
        // balance before minting
        {
            let _before_data = self.customer_mint_ata.data.borrow();
            let _before_state = StateWithExtensions::<TokenAccount>::unpack(&_before_data)?;
            
            // msg!("before mint balance={}", _before_state.base.amount);

            require!(
                _before_state.base.amount == 0,
                ProtocolError::InvalidBalancePreMint
            );
        }
        

        // Mint the mint
        mint_to(
            CpiContext::new_with_signer(
                self.token_2022_program.to_account_info(),
                MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.customer_mint_ata.to_account_info(),
                    authority: self.auth.to_account_info(),
                },
                signer_seeds
            ),
            1,
        )?;

        set_authority(
            CpiContext::new_with_signer(
                self.token_2022_program.to_account_info(), 
                SetAuthority {
                    current_authority: self.auth.to_account_info(),
                    account_or_mint: self.mint.to_account_info(),
                }, 
                signer_seeds
            ), 
            AuthorityType::MintTokens, 
            None
        )?;

        // balance after minting, reload the data
        {
            let _after_data = self.customer_mint_ata.data.borrow();
            let _after_state = StateWithExtensions::<TokenAccount>::unpack(&_after_data)?;

            // msg!("after mint balance={}", _after_state.base.amount);

            require!(_after_state.base.amount == 1, ProtocolError::InvalidBalancePostMint);
        }

        self.restaurant.customer_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64, uri: String, attributes: Vec<Attributes>)]
pub struct CustomerInit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is ok, we are creating everything on the customer behalf
    pub customer: AccountInfo<'info>,
    #[account(
        init,
        payer = admin,
        space = Customer::INIT_SPACE + 5,
        seeds = [b"customer", customer.key().as_ref()],
        bump
    )]
    pub customer_profile: Account<'info, Customer>,
    #[account(
        init,
        payer = admin,
        seeds = [b"ainft", customer.key().as_ref(), id.to_le_bytes().as_ref()],
        bump,
        space = CustomerNft::INIT_SPACE + attributes.iter().map(|attr| attr.key.len() + attr.value.len()).sum::<usize>(),
    )] 
    pub customer_nft: Account<'info, CustomerNft>,
    #[account(
        mut,
        seeds = [b"mint", customer_nft.key().as_ref()],
        bump
    )]
    /// CHECK
    pub mint: UncheckedAccount<'info>,
    #[account(
        seeds = [b"auth"],
        bump
    )]
    /// CHECK:
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            customer.key().as_ref(),
            token_2022_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = associated_token_program.key(),
        bump
    )]
    /// CHECK
    pub customer_mint_ata: AccountInfo<'info>,
    pub restaurant_mint: AccountInfo<'info>,
    #[account(
        seeds = [b"restaurant", restaurant_mint.key().as_ref()],
        bump
    )]
    /// CHECK: this is fine since we are hard coding the collection sysvar.
    pub restaurant: Account<'info, Restaurant>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(address = RENT_ID)]
    /// CHECK: this is fine since we are hard coding the rent sysvar.
    pub rent: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_2022_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

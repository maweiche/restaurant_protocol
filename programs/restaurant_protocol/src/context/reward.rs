use crate::{
    state::{
        RestaurantAdmin,
        Restaurant,
        Protocol,
        Reward,
        Customer,
        CustomerNft,
    },
    constant::{ ED25519_PROGRAM_ID, admin_wallet },
    errors::ProtocolError,
};
use std::str::FromStr;
pub use anchor_lang::{
    solana_program::{
        sysvar::{
            rent::ID as RENT_ID,
            instructions::{
                self,
                load_current_index_checked,
                load_instruction_at_checked,
            }
        },
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
    token_interface::{MintTo, mint_to, set_authority, SetAuthority}
};
pub use spl_token_2022::{
    extension::ExtensionType,
    instruction::{initialize_mint_close_authority, initialize_permanent_delegate, initialize_mint2},
    extension::metadata_pointer::instruction::initialize as initialize_metadata_pointer,
};
pub use spl_token_metadata_interface::{
    state::{TokenMetadata, Field},
    instruction::{initialize as initialize_metadata_account, update_field as update_metadata_account},
};

impl<'info> RewardInit<'info> {
    pub fn add(
        &mut self,
        category: Pubkey,
        restaurant: Pubkey,
        reward_points: u64,
        reward_item: Pubkey,
        uri: String,
        bumps: RewardInitBumps,
    ) -> Result<()> {

        /*
        
            Create a new Employee Ix:

            Some security check:
            

        */
        
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);
        require!(self.restaurant_admin_state.restaurant.key() == self.restaurant.key(), ProtocolError::UnauthorizedAdmin);
        
        self.reward.set_inner(
            Reward {
                category,
                restaurant,
                reward_points,
                reward_item,
            }
        );

        // Step 1: Initialize Account
        let size = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
            &[
                ExtensionType::PermanentDelegate,
                ExtensionType::MetadataPointer,
            ],
        ).unwrap();
        let metadata = TokenMetadata {
            update_authority: spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(Some(self.auth.key())).unwrap(),
            mint: self.mint.key(),
            name: "Reward for ".to_string() + &self.restaurant.name,
            symbol: "TREAT".to_string(),
            uri,
            additional_metadata: vec![
                ("category".to_string(), category.to_string()),
                ("restaurant".to_string(), restaurant.to_string()),
                ("reward_points".to_string(), reward_points.to_string()),
                ("reward_item".to_string(), reward_item.to_string()),
            ]
        };

        let extension_extra_space = metadata.tlv_size_of().unwrap();
        let rent = &Rent::from_account_info(&self.rent.to_account_info())?;
        let lamports = rent.minimum_balance(size + extension_extra_space);

        let reward_key = self.reward.key();
        let seeds: &[&[u8]; 3] = &[
            b"mint",
            reward_key.as_ref(),
            &[bumps.mint],
        ];
        let signer_seeds = &[&seeds[..]];

        invoke_signed(
            &solana_program::system_instruction::create_account(
                &self.restaurant_admin.key(),
                &self.mint.key(),
                lamports,
                (size).try_into().unwrap(),
                &spl_token_2022::id(),
            ),
            &vec![
                self.restaurant_admin.to_account_info(),
                self.mint.to_account_info(),
            ],
            signer_seeds
        )?;

        // Step 2: Initialize Extension needed: 

        // 2.1: Permanent Delegate, 
        invoke(
            &initialize_permanent_delegate(
                &self.token_2022_program.key(),
                &self.mint.key(),
                &self.auth.key(),
            )?,
            &vec![
                self.mint.to_account_info(),
            ],
        )?;
        
        // 2.2: Close Mint Authority, 
        invoke(
            &initialize_mint_close_authority(
                &self.token_2022_program.key(),
                &self.mint.key(),
                Some(&self.auth.key()),
            )?,
            &vec![
                self.mint.to_account_info(),
            ],
        )?;
        
        // 2.3: Metadata Pointer
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

        Ok(())
    }
}

impl<'info> RewardRemove<'info> {
    pub fn remove(&mut self) -> Result<()> {
        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);

        Ok(())
    }
}

impl<'info> RewardBuy<'info> {
    pub fn buy(
        &mut self,
        bumps: RewardBuyBumps,
    ) -> Result<()> {

        /*


            STILL NEED TO ADD SECURITY CHECKS

        */

        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);

        let seeds: &[&[u8]; 2] = &[
            b"auth",
            &[bumps.auth],
        ];
        let signer_seeds = &[&seeds[..]];

        // Initialize ATA
        create(
            CpiContext::new(
                self.token_2022_program.to_account_info(),
                Create {
                    payer: self.customer.to_account_info(), // payer
                    associated_token: self.customer_mint_ata.to_account_info(),
                    authority: self.customer.to_account_info(), // owner
                    mint: self.mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_2022_program.to_account_info(),
                }
            ),
        )?;

        // balance before minting
        {
            let _before_data = self.customer_mint_ata.data.borrow();
            let _before_state = StateWithExtensions::<TokenAccount>::unpack(&_before_data)?;
        
            // msg!("before mint balance={}", _before_state.base.amount);
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

        // check the post balance of the mint
        {
            let _data = self.customer_mint_ata.data.borrow();
            let _state = StateWithExtensions::<TokenAccount>::unpack(&_data)?;
        
            // msg!("after mint balance={}", _state.base.amount);
            require!(
                _state.base.amount == 1,
                ProtocolError::InvalidBalancePostMint
            );
        }



        let cost_of_reward = self.reward.reward_points;
        let current_reward_points = self.customer_nft.reward_points;
        let new_reward_points = current_reward_points - cost_of_reward;

        self.customer_nft.reward_points = new_reward_points;
        
        invoke_signed(
            &update_metadata_account(
                &self.token_2022_program.key(),
                &self.mint.key(),
                &self.auth.key(),
                Field::Key("reward_points".to_string()),
                new_reward_points.to_string(),
            ),
            &vec![
                self.mint.to_account_info(),
                self.auth.to_account_info(),
            ],
            signer_seeds
        )?;

        Ok(())
    }

}

impl<'info> RewardAirdrop<'info> {
    pub fn airdrop(
        &mut self,
        bumps: RewardAirdropBumps,
    ) -> Result<()> {

        /*

        
            STILL NEED TO ADD SECURITY CHECKS

        */

        require!(!self.protocol.locked, ProtocolError::ProtocolLocked);

        // Instruction Check
        let ixs = self.instructions.to_account_info();
        let current_index = load_current_index_checked(&ixs)? as usize;


        let seeds: &[&[u8]; 2] = &[
            b"auth",
            &[bumps.auth],
        ];
        let signer_seeds = &[&seeds[..]];

        // If the current index is greater than 0, then we can check for the airdrop instructions
        if current_index > 0 {
            match load_instruction_at_checked(current_index - 1, &ixs) {
               Ok(signature_ix) => {
                   if Pubkey::from_str(ED25519_PROGRAM_ID).unwrap() == signature_ix.program_id {
                        // Ensure signing authority is correct
                      require!(
                        admin_wallet::id()
                            .to_bytes()
                            .eq(&signature_ix.data[16..48]),
                        ProtocolError::UnauthorizedAdmin,
                      );

                      let mut message_data: [u8; 32] = [0; 32];
                      message_data.copy_from_slice(&signature_ix.data[112..144]);
                      let _buyer = Pubkey::from(message_data);

                      require!(
                        _buyer == *self.customer.key,
                        ProtocolError::UnauthorizedAdmin,
                      );

                   //    invoke(
                   //     &transfer_instruction_two,
                   //     &[
                   //         self.collection_owner.to_account_info(),
                   //         self.payer.to_account_info(),
                   //         self.system_program.to_account_info(),
                   //     ],
                   //     )?;
           
                       // Initialize ATA
                       create(
                       CpiContext::new(
                           self.token_2022_program.to_account_info(),
                           Create {
                               payer: self.restaurant_admin.to_account_info(), // payer
                               associated_token: self.customer_mint_ata.to_account_info(),
                               authority: self.customer.to_account_info(), // owner
                               mint: self.mint.to_account_info(),
                               system_program: self.system_program.to_account_info(),
                               token_program: self.token_2022_program.to_account_info(),
                           }
                       ),
                       )?;

                       // balance before minting
                       {
                           let _before_data = self.customer_mint_ata.data.borrow();
                           let _before_state = StateWithExtensions::<TokenAccount>::unpack(&_before_data)?;
                       
                           // msg!("before mint balance={}", _before_state.base.amount);
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

                       // check the post balance of the mint
                       {
                           let _after_data = self.customer_mint_ata.data.borrow();
                           let _after_state = StateWithExtensions::<TokenAccount>::unpack(&_after_data)?;

                           // msg!("after mint balance={}", _after_state.base.amount);

                           require!(_after_state.base.amount == 1, ProtocolError::InvalidBalancePostMint);
                       }
                   } else {
                       // NO ED25519 instruction
                       Err(ProtocolError::InstructionsNotCorrect)?;
                   }
               }
               Err(_) => {
                   // NO ED25519 instruction
                   Err(ProtocolError::InstructionsNotCorrect)?;
               }
           }
       }

        Ok(())

    }


}

#[derive(Accounts)]
#[instruction(item: Pubkey)]
pub struct RewardInit<'info> {
    /// CHECK: this is ok because admin is setting up on owner behalf
    #[account(mut)]
    pub restaurant_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"restaurant", restaurant_owner.key().as_ref()],
        bump,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", restaurant_admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    #[account(
        init,
        payer = restaurant_admin,
        space = Reward::INIT_SPACE + 5,
        seeds = [b"reward", reward.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub reward: Account<'info, Reward>,
    /// CHECK: this is fine since we are handling all the checks and creation in the program.
    #[account(
        mut,
        seeds = [b"mint", reward.key().as_ref()],
        bump
    )]
    pub mint: UncheckedAccount<'info>,
    /// CHECK:
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(address = RENT_ID)]
    /// CHECK: this is fine since we are hard coding the rent sysvar.
    pub rent: UncheckedAccount<'info>,
    pub token_2022_program: Program<'info, Token2022>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RewardRemove<'info> {
    /// CHECK: this is ok because admin is setting up on owner behalf
    #[account(mut)]
    pub restaurant_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"restaurant", restaurant_owner.key().as_ref()],
        bump,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", restaurant_admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"reward", reward.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub reward: Account<'info, Reward>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RewardBuy<'info> {
    /// CHECK: this is ok because admin is setting up on owner behalf
    #[account(mut)]
    pub restaurant_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"restaurant", restaurant_owner.key().as_ref()],
        bump,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(mut)]
    pub customer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"customer", customer.key().as_ref()],
        bump
    )]
    pub customer_profile: Account<'info, Customer>,
    #[account(
        mut,
        seeds = [b"member_nft", customer.key().as_ref(), restaurant.key().as_ref()],
        bump,
    )] 
    pub customer_nft: Account<'info, CustomerNft>,
    #[account(
        seeds = [b"reward", reward.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub reward: Account<'info, Reward>,
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
    pub customer_mint_ata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"mint", reward.key().as_ref()],
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
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_2022_program: Program<'info, Token2022>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct RewardAirdrop<'info> {
    /// CHECK: this is ok because admin is setting up on owner behalf
    #[account(mut)]
    pub restaurant_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"restaurant", restaurant_owner.key().as_ref()],
        bump,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"admin_state", restaurant_admin.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub restaurant_admin_state: Account<'info, RestaurantAdmin>,
    /// CHECK: this is ok since it is restaurant_admin minting/sending nft
    #[account(mut)]
    pub customer: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"customer", customer.key().as_ref()],
        bump
    )]
    pub customer_profile: Account<'info, Customer>,
    #[account(
        mut,
        seeds = [b"member_nft", customer.key().as_ref(), restaurant.key().as_ref()],
        bump,
    )] 
    pub customer_nft: Account<'info, CustomerNft>,
    #[account(
        seeds = [b"reward", reward.key().as_ref(), restaurant.key().as_ref()],
        bump
    )]
    pub reward: Account<'info, Reward>,
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
    pub customer_mint_ata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"mint", reward.key().as_ref()],
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
    /// CHECK: InstructionsSysvar account
    instructions: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_2022_program: Program<'info, Token2022>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
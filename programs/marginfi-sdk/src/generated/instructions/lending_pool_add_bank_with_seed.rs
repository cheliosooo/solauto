//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::generated::types::BankConfigCompact;

/// Accounts.
pub struct LendingPoolAddBankWithSeed {
      
              
          pub marginfi_group: solana_program::pubkey::Pubkey,
          
              
          pub admin: solana_program::pubkey::Pubkey,
          
              
          pub fee_payer: solana_program::pubkey::Pubkey,
          
              
          pub bank_mint: solana_program::pubkey::Pubkey,
          
              
          pub bank: solana_program::pubkey::Pubkey,
          
              
          pub liquidity_vault_authority: solana_program::pubkey::Pubkey,
          
              
          pub liquidity_vault: solana_program::pubkey::Pubkey,
          
              
          pub insurance_vault_authority: solana_program::pubkey::Pubkey,
          
              
          pub insurance_vault: solana_program::pubkey::Pubkey,
          
              
          pub fee_vault_authority: solana_program::pubkey::Pubkey,
          
              
          pub fee_vault: solana_program::pubkey::Pubkey,
          
              
          pub rent: solana_program::pubkey::Pubkey,
          
              
          pub token_program: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl LendingPoolAddBankWithSeed {
  pub fn instruction(&self, args: LendingPoolAddBankWithSeedInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: LendingPoolAddBankWithSeedInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.marginfi_group,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.fee_payer,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.bank_mint,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.bank,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.liquidity_vault_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.liquidity_vault,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.insurance_vault_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.insurance_vault,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.fee_vault_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.fee_vault,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = LendingPoolAddBankWithSeedInstructionData::new().try_to_vec().unwrap();
          let mut args = args.try_to_vec().unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::MARGINFI_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct LendingPoolAddBankWithSeedInstructionData {
              }

impl LendingPoolAddBankWithSeedInstructionData {
  fn new() -> Self {
    Self {
                                      }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LendingPoolAddBankWithSeedInstructionArgs {
            pub bank_config: BankConfigCompact,
                pub bank_seed: u64,
      }


/// Instruction builder for `LendingPoolAddBankWithSeed`.
///
/// ### Accounts:
///
          ///   0. `[]` marginfi_group
                      ///   1. `[writable, signer]` admin
                      ///   2. `[writable, signer]` fee_payer
          ///   3. `[]` bank_mint
                ///   4. `[writable]` bank
          ///   5. `[]` liquidity_vault_authority
                ///   6. `[writable]` liquidity_vault
          ///   7. `[]` insurance_vault_authority
                ///   8. `[writable]` insurance_vault
          ///   9. `[]` fee_vault_authority
                ///   10. `[writable]` fee_vault
                ///   11. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
                ///   12. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
                ///   13. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Default)]
pub struct LendingPoolAddBankWithSeedBuilder {
            marginfi_group: Option<solana_program::pubkey::Pubkey>,
                admin: Option<solana_program::pubkey::Pubkey>,
                fee_payer: Option<solana_program::pubkey::Pubkey>,
                bank_mint: Option<solana_program::pubkey::Pubkey>,
                bank: Option<solana_program::pubkey::Pubkey>,
                liquidity_vault_authority: Option<solana_program::pubkey::Pubkey>,
                liquidity_vault: Option<solana_program::pubkey::Pubkey>,
                insurance_vault_authority: Option<solana_program::pubkey::Pubkey>,
                insurance_vault: Option<solana_program::pubkey::Pubkey>,
                fee_vault_authority: Option<solana_program::pubkey::Pubkey>,
                fee_vault: Option<solana_program::pubkey::Pubkey>,
                rent: Option<solana_program::pubkey::Pubkey>,
                token_program: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                  bank_config: Option<BankConfigCompact>,
                bank_seed: Option<u64>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl LendingPoolAddBankWithSeedBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn marginfi_group(&mut self, marginfi_group: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.marginfi_group = Some(marginfi_group);
                    self
    }
            #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.admin = Some(admin);
                    self
    }
            #[inline(always)]
    pub fn fee_payer(&mut self, fee_payer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.fee_payer = Some(fee_payer);
                    self
    }
            #[inline(always)]
    pub fn bank_mint(&mut self, bank_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.bank_mint = Some(bank_mint);
                    self
    }
            #[inline(always)]
    pub fn bank(&mut self, bank: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.bank = Some(bank);
                    self
    }
            #[inline(always)]
    pub fn liquidity_vault_authority(&mut self, liquidity_vault_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.liquidity_vault_authority = Some(liquidity_vault_authority);
                    self
    }
            #[inline(always)]
    pub fn liquidity_vault(&mut self, liquidity_vault: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.liquidity_vault = Some(liquidity_vault);
                    self
    }
            #[inline(always)]
    pub fn insurance_vault_authority(&mut self, insurance_vault_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.insurance_vault_authority = Some(insurance_vault_authority);
                    self
    }
            #[inline(always)]
    pub fn insurance_vault(&mut self, insurance_vault: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.insurance_vault = Some(insurance_vault);
                    self
    }
            #[inline(always)]
    pub fn fee_vault_authority(&mut self, fee_vault_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.fee_vault_authority = Some(fee_vault_authority);
                    self
    }
            #[inline(always)]
    pub fn fee_vault(&mut self, fee_vault: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.fee_vault = Some(fee_vault);
                    self
    }
            /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
#[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.rent = Some(rent);
                    self
    }
            /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
#[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program = Some(token_program);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
              #[inline(always)]
      pub fn bank_config(&mut self, bank_config: BankConfigCompact) -> &mut Self {
        self.bank_config = Some(bank_config);
        self
      }
                #[inline(always)]
      pub fn bank_seed(&mut self, bank_seed: u64) -> &mut Self {
        self.bank_seed = Some(bank_seed);
        self
      }
        /// Add an aditional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = LendingPoolAddBankWithSeed {
                              marginfi_group: self.marginfi_group.expect("marginfi_group is not set"),
                                        admin: self.admin.expect("admin is not set"),
                                        fee_payer: self.fee_payer.expect("fee_payer is not set"),
                                        bank_mint: self.bank_mint.expect("bank_mint is not set"),
                                        bank: self.bank.expect("bank is not set"),
                                        liquidity_vault_authority: self.liquidity_vault_authority.expect("liquidity_vault_authority is not set"),
                                        liquidity_vault: self.liquidity_vault.expect("liquidity_vault is not set"),
                                        insurance_vault_authority: self.insurance_vault_authority.expect("insurance_vault_authority is not set"),
                                        insurance_vault: self.insurance_vault.expect("insurance_vault is not set"),
                                        fee_vault_authority: self.fee_vault_authority.expect("fee_vault_authority is not set"),
                                        fee_vault: self.fee_vault.expect("fee_vault is not set"),
                                        rent: self.rent.unwrap_or(solana_program::pubkey!("SysvarRent111111111111111111111111111111111")),
                                        token_program: self.token_program.unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
          let args = LendingPoolAddBankWithSeedInstructionArgs {
                                            bank_config: self.bank_config.clone().expect("bank_config is not set"),
                                                                  bank_seed: self.bank_seed.clone().expect("bank_seed is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `lending_pool_add_bank_with_seed` CPI accounts.
  pub struct LendingPoolAddBankWithSeedCpiAccounts<'a, 'b> {
          
                    
              pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub admin: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub fee_payer: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub bank_mint: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub bank: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub liquidity_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub liquidity_vault: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub insurance_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub fee_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub fee_vault: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub rent: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `lending_pool_add_bank_with_seed` CPI instruction.
pub struct LendingPoolAddBankWithSeedCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub marginfi_group: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub admin: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub fee_payer: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub bank_mint: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub bank: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub liquidity_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub liquidity_vault: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub insurance_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub insurance_vault: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub fee_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub fee_vault: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub rent: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: LendingPoolAddBankWithSeedInstructionArgs,
  }

impl<'a, 'b> LendingPoolAddBankWithSeedCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: LendingPoolAddBankWithSeedCpiAccounts<'a, 'b>,
              args: LendingPoolAddBankWithSeedInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              marginfi_group: accounts.marginfi_group,
              admin: accounts.admin,
              fee_payer: accounts.fee_payer,
              bank_mint: accounts.bank_mint,
              bank: accounts.bank,
              liquidity_vault_authority: accounts.liquidity_vault_authority,
              liquidity_vault: accounts.liquidity_vault,
              insurance_vault_authority: accounts.insurance_vault_authority,
              insurance_vault: accounts.insurance_vault,
              fee_vault_authority: accounts.fee_vault_authority,
              fee_vault: accounts.fee_vault,
              rent: accounts.rent,
              token_program: accounts.token_program,
              system_program: accounts.system_program,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.marginfi_group.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fee_payer.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.bank_mint.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bank.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.liquidity_vault_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.liquidity_vault.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.insurance_vault_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.insurance_vault.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.fee_vault_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fee_vault.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = LendingPoolAddBankWithSeedInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::MARGINFI_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(14 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.marginfi_group.clone());
                        account_infos.push(self.admin.clone());
                        account_infos.push(self.fee_payer.clone());
                        account_infos.push(self.bank_mint.clone());
                        account_infos.push(self.bank.clone());
                        account_infos.push(self.liquidity_vault_authority.clone());
                        account_infos.push(self.liquidity_vault.clone());
                        account_infos.push(self.insurance_vault_authority.clone());
                        account_infos.push(self.insurance_vault.clone());
                        account_infos.push(self.fee_vault_authority.clone());
                        account_infos.push(self.fee_vault.clone());
                        account_infos.push(self.rent.clone());
                        account_infos.push(self.token_program.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `LendingPoolAddBankWithSeed` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` marginfi_group
                      ///   1. `[writable, signer]` admin
                      ///   2. `[writable, signer]` fee_payer
          ///   3. `[]` bank_mint
                ///   4. `[writable]` bank
          ///   5. `[]` liquidity_vault_authority
                ///   6. `[writable]` liquidity_vault
          ///   7. `[]` insurance_vault_authority
                ///   8. `[writable]` insurance_vault
          ///   9. `[]` fee_vault_authority
                ///   10. `[writable]` fee_vault
          ///   11. `[]` rent
          ///   12. `[]` token_program
          ///   13. `[]` system_program
pub struct LendingPoolAddBankWithSeedCpiBuilder<'a, 'b> {
  instruction: Box<LendingPoolAddBankWithSeedCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> LendingPoolAddBankWithSeedCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(LendingPoolAddBankWithSeedCpiBuilderInstruction {
      __program: program,
              marginfi_group: None,
              admin: None,
              fee_payer: None,
              bank_mint: None,
              bank: None,
              liquidity_vault_authority: None,
              liquidity_vault: None,
              insurance_vault_authority: None,
              insurance_vault: None,
              fee_vault_authority: None,
              fee_vault: None,
              rent: None,
              token_program: None,
              system_program: None,
                              bank_config: None,
                                bank_seed: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn marginfi_group(&mut self, marginfi_group: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.marginfi_group = Some(marginfi_group);
                    self
    }
      #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.admin = Some(admin);
                    self
    }
      #[inline(always)]
    pub fn fee_payer(&mut self, fee_payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.fee_payer = Some(fee_payer);
                    self
    }
      #[inline(always)]
    pub fn bank_mint(&mut self, bank_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.bank_mint = Some(bank_mint);
                    self
    }
      #[inline(always)]
    pub fn bank(&mut self, bank: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.bank = Some(bank);
                    self
    }
      #[inline(always)]
    pub fn liquidity_vault_authority(&mut self, liquidity_vault_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.liquidity_vault_authority = Some(liquidity_vault_authority);
                    self
    }
      #[inline(always)]
    pub fn liquidity_vault(&mut self, liquidity_vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.liquidity_vault = Some(liquidity_vault);
                    self
    }
      #[inline(always)]
    pub fn insurance_vault_authority(&mut self, insurance_vault_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.insurance_vault_authority = Some(insurance_vault_authority);
                    self
    }
      #[inline(always)]
    pub fn insurance_vault(&mut self, insurance_vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.insurance_vault = Some(insurance_vault);
                    self
    }
      #[inline(always)]
    pub fn fee_vault_authority(&mut self, fee_vault_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.fee_vault_authority = Some(fee_vault_authority);
                    self
    }
      #[inline(always)]
    pub fn fee_vault(&mut self, fee_vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.fee_vault = Some(fee_vault);
                    self
    }
      #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.rent = Some(rent);
                    self
    }
      #[inline(always)]
    pub fn token_program(&mut self, token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program = Some(token_program);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
              #[inline(always)]
      pub fn bank_config(&mut self, bank_config: BankConfigCompact) -> &mut Self {
        self.instruction.bank_config = Some(bank_config);
        self
      }
                #[inline(always)]
      pub fn bank_seed(&mut self, bank_seed: u64) -> &mut Self {
        self.instruction.bank_seed = Some(bank_seed);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = LendingPoolAddBankWithSeedInstructionArgs {
                                            bank_config: self.instruction.bank_config.clone().expect("bank_config is not set"),
                                                                  bank_seed: self.instruction.bank_seed.clone().expect("bank_seed is not set"),
                                    };
        let instruction = LendingPoolAddBankWithSeedCpi {
        __program: self.instruction.__program,
                  
          marginfi_group: self.instruction.marginfi_group.expect("marginfi_group is not set"),
                  
          admin: self.instruction.admin.expect("admin is not set"),
                  
          fee_payer: self.instruction.fee_payer.expect("fee_payer is not set"),
                  
          bank_mint: self.instruction.bank_mint.expect("bank_mint is not set"),
                  
          bank: self.instruction.bank.expect("bank is not set"),
                  
          liquidity_vault_authority: self.instruction.liquidity_vault_authority.expect("liquidity_vault_authority is not set"),
                  
          liquidity_vault: self.instruction.liquidity_vault.expect("liquidity_vault is not set"),
                  
          insurance_vault_authority: self.instruction.insurance_vault_authority.expect("insurance_vault_authority is not set"),
                  
          insurance_vault: self.instruction.insurance_vault.expect("insurance_vault is not set"),
                  
          fee_vault_authority: self.instruction.fee_vault_authority.expect("fee_vault_authority is not set"),
                  
          fee_vault: self.instruction.fee_vault.expect("fee_vault is not set"),
                  
          rent: self.instruction.rent.expect("rent is not set"),
                  
          token_program: self.instruction.token_program.expect("token_program is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

struct LendingPoolAddBankWithSeedCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            marginfi_group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                fee_payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                bank_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                bank: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                liquidity_vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                liquidity_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                insurance_vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                insurance_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                fee_vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                fee_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                  bank_config: Option<BankConfigCompact>,
                bank_seed: Option<u64>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}


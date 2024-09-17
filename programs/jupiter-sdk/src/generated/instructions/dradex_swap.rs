//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct DradexSwap {
    pub swap_program: solana_program::pubkey::Pubkey,

    pub pair: solana_program::pubkey::Pubkey,

    pub market: solana_program::pubkey::Pubkey,

    pub event_queue: solana_program::pubkey::Pubkey,

    pub dex_user: solana_program::pubkey::Pubkey,

    pub market_user: solana_program::pubkey::Pubkey,

    pub bids: solana_program::pubkey::Pubkey,

    pub asks: solana_program::pubkey::Pubkey,

    pub t0_vault: solana_program::pubkey::Pubkey,

    pub t1_vault: solana_program::pubkey::Pubkey,

    pub t0_user: solana_program::pubkey::Pubkey,

    pub t1_user: solana_program::pubkey::Pubkey,

    pub master: solana_program::pubkey::Pubkey,

    pub signer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub logger: solana_program::pubkey::Pubkey,
}

impl DradexSwap {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.swap_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pair, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.market,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.event_queue,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.dex_user,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.market_user,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bids, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asks, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.t0_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.t1_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.t0_user,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.t1_user,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.master,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.logger,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = DradexSwapInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct DradexSwapInstructionData {
    discriminator: [u8; 8],
}

impl DradexSwapInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [34, 146, 160, 38, 51, 85, 58, 151],
        }
    }
}

/// Instruction builder for `DradexSwap`.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[writable]` pair
///   2. `[writable]` market
///   3. `[writable]` event_queue
///   4. `[]` dex_user
///   5. `[writable]` market_user
///   6. `[writable]` bids
///   7. `[writable]` asks
///   8. `[writable]` t0_vault
///   9. `[writable]` t1_vault
///   10. `[writable]` t0_user
///   11. `[writable]` t1_user
///   12. `[]` master
///   13. `[writable]` signer
///   14. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   15. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   16. `[]` logger
#[derive(Default)]
pub struct DradexSwapBuilder {
    swap_program: Option<solana_program::pubkey::Pubkey>,
    pair: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    event_queue: Option<solana_program::pubkey::Pubkey>,
    dex_user: Option<solana_program::pubkey::Pubkey>,
    market_user: Option<solana_program::pubkey::Pubkey>,
    bids: Option<solana_program::pubkey::Pubkey>,
    asks: Option<solana_program::pubkey::Pubkey>,
    t0_vault: Option<solana_program::pubkey::Pubkey>,
    t1_vault: Option<solana_program::pubkey::Pubkey>,
    t0_user: Option<solana_program::pubkey::Pubkey>,
    t1_user: Option<solana_program::pubkey::Pubkey>,
    master: Option<solana_program::pubkey::Pubkey>,
    signer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    logger: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl DradexSwapBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn swap_program(&mut self, swap_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.swap_program = Some(swap_program);
        self
    }
    #[inline(always)]
    pub fn pair(&mut self, pair: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pair = Some(pair);
        self
    }
    #[inline(always)]
    pub fn market(&mut self, market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market = Some(market);
        self
    }
    #[inline(always)]
    pub fn event_queue(&mut self, event_queue: solana_program::pubkey::Pubkey) -> &mut Self {
        self.event_queue = Some(event_queue);
        self
    }
    #[inline(always)]
    pub fn dex_user(&mut self, dex_user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.dex_user = Some(dex_user);
        self
    }
    #[inline(always)]
    pub fn market_user(&mut self, market_user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market_user = Some(market_user);
        self
    }
    #[inline(always)]
    pub fn bids(&mut self, bids: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bids = Some(bids);
        self
    }
    #[inline(always)]
    pub fn asks(&mut self, asks: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asks = Some(asks);
        self
    }
    #[inline(always)]
    pub fn t0_vault(&mut self, t0_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.t0_vault = Some(t0_vault);
        self
    }
    #[inline(always)]
    pub fn t1_vault(&mut self, t1_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.t1_vault = Some(t1_vault);
        self
    }
    #[inline(always)]
    pub fn t0_user(&mut self, t0_user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.t0_user = Some(t0_user);
        self
    }
    #[inline(always)]
    pub fn t1_user(&mut self, t1_user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.t1_user = Some(t1_user);
        self
    }
    #[inline(always)]
    pub fn master(&mut self, master: solana_program::pubkey::Pubkey) -> &mut Self {
        self.master = Some(master);
        self
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn logger(&mut self, logger: solana_program::pubkey::Pubkey) -> &mut Self {
        self.logger = Some(logger);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = DradexSwap {
            swap_program: self.swap_program.expect("swap_program is not set"),
            pair: self.pair.expect("pair is not set"),
            market: self.market.expect("market is not set"),
            event_queue: self.event_queue.expect("event_queue is not set"),
            dex_user: self.dex_user.expect("dex_user is not set"),
            market_user: self.market_user.expect("market_user is not set"),
            bids: self.bids.expect("bids is not set"),
            asks: self.asks.expect("asks is not set"),
            t0_vault: self.t0_vault.expect("t0_vault is not set"),
            t1_vault: self.t1_vault.expect("t1_vault is not set"),
            t0_user: self.t0_user.expect("t0_user is not set"),
            t1_user: self.t1_user.expect("t1_user is not set"),
            master: self.master.expect("master is not set"),
            signer: self.signer.expect("signer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            logger: self.logger.expect("logger is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `dradex_swap` CPI accounts.
pub struct DradexSwapCpiAccounts<'a, 'b> {
    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_queue: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,

    pub t0_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub t1_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub t0_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub t1_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub master: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub logger: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `dradex_swap` CPI instruction.
pub struct DradexSwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub swap_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub market: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_queue: &'b solana_program::account_info::AccountInfo<'a>,

    pub dex_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub market_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub bids: &'b solana_program::account_info::AccountInfo<'a>,

    pub asks: &'b solana_program::account_info::AccountInfo<'a>,

    pub t0_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub t1_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub t0_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub t1_user: &'b solana_program::account_info::AccountInfo<'a>,

    pub master: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub logger: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> DradexSwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: DradexSwapCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            swap_program: accounts.swap_program,
            pair: accounts.pair,
            market: accounts.market,
            event_queue: accounts.event_queue,
            dex_user: accounts.dex_user,
            market_user: accounts.market_user,
            bids: accounts.bids,
            asks: accounts.asks,
            t0_vault: accounts.t0_vault,
            t1_vault: accounts.t1_vault,
            t0_user: accounts.t0_user,
            t1_user: accounts.t1_user,
            master: accounts.master,
            signer: accounts.signer,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            logger: accounts.logger,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.swap_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pair.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.market.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.event_queue.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.dex_user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.market_user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bids.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asks.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.t0_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.t1_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.t0_user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.t1_user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.master.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.logger.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = DradexSwapInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JUPITER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(17 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.swap_program.clone());
        account_infos.push(self.pair.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.event_queue.clone());
        account_infos.push(self.dex_user.clone());
        account_infos.push(self.market_user.clone());
        account_infos.push(self.bids.clone());
        account_infos.push(self.asks.clone());
        account_infos.push(self.t0_vault.clone());
        account_infos.push(self.t1_vault.clone());
        account_infos.push(self.t0_user.clone());
        account_infos.push(self.t1_user.clone());
        account_infos.push(self.master.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.logger.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `DradexSwap` via CPI.
///
/// ### Accounts:
///
///   0. `[]` swap_program
///   1. `[writable]` pair
///   2. `[writable]` market
///   3. `[writable]` event_queue
///   4. `[]` dex_user
///   5. `[writable]` market_user
///   6. `[writable]` bids
///   7. `[writable]` asks
///   8. `[writable]` t0_vault
///   9. `[writable]` t1_vault
///   10. `[writable]` t0_user
///   11. `[writable]` t1_user
///   12. `[]` master
///   13. `[writable]` signer
///   14. `[]` system_program
///   15. `[]` token_program
///   16. `[]` logger
pub struct DradexSwapCpiBuilder<'a, 'b> {
    instruction: Box<DradexSwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> DradexSwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(DradexSwapCpiBuilderInstruction {
            __program: program,
            swap_program: None,
            pair: None,
            market: None,
            event_queue: None,
            dex_user: None,
            market_user: None,
            bids: None,
            asks: None,
            t0_vault: None,
            t1_vault: None,
            t0_user: None,
            t1_user: None,
            master: None,
            signer: None,
            system_program: None,
            token_program: None,
            logger: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn swap_program(
        &mut self,
        swap_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.swap_program = Some(swap_program);
        self
    }
    #[inline(always)]
    pub fn pair(&mut self, pair: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pair = Some(pair);
        self
    }
    #[inline(always)]
    pub fn market(
        &mut self,
        market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market = Some(market);
        self
    }
    #[inline(always)]
    pub fn event_queue(
        &mut self,
        event_queue: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_queue = Some(event_queue);
        self
    }
    #[inline(always)]
    pub fn dex_user(
        &mut self,
        dex_user: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.dex_user = Some(dex_user);
        self
    }
    #[inline(always)]
    pub fn market_user(
        &mut self,
        market_user: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market_user = Some(market_user);
        self
    }
    #[inline(always)]
    pub fn bids(&mut self, bids: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.bids = Some(bids);
        self
    }
    #[inline(always)]
    pub fn asks(&mut self, asks: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.asks = Some(asks);
        self
    }
    #[inline(always)]
    pub fn t0_vault(
        &mut self,
        t0_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.t0_vault = Some(t0_vault);
        self
    }
    #[inline(always)]
    pub fn t1_vault(
        &mut self,
        t1_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.t1_vault = Some(t1_vault);
        self
    }
    #[inline(always)]
    pub fn t0_user(
        &mut self,
        t0_user: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.t0_user = Some(t0_user);
        self
    }
    #[inline(always)]
    pub fn t1_user(
        &mut self,
        t1_user: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.t1_user = Some(t1_user);
        self
    }
    #[inline(always)]
    pub fn master(
        &mut self,
        master: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.master = Some(master);
        self
    }
    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn logger(
        &mut self,
        logger: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.logger = Some(logger);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = DradexSwapCpi {
            __program: self.instruction.__program,

            swap_program: self
                .instruction
                .swap_program
                .expect("swap_program is not set"),

            pair: self.instruction.pair.expect("pair is not set"),

            market: self.instruction.market.expect("market is not set"),

            event_queue: self
                .instruction
                .event_queue
                .expect("event_queue is not set"),

            dex_user: self.instruction.dex_user.expect("dex_user is not set"),

            market_user: self
                .instruction
                .market_user
                .expect("market_user is not set"),

            bids: self.instruction.bids.expect("bids is not set"),

            asks: self.instruction.asks.expect("asks is not set"),

            t0_vault: self.instruction.t0_vault.expect("t0_vault is not set"),

            t1_vault: self.instruction.t1_vault.expect("t1_vault is not set"),

            t0_user: self.instruction.t0_user.expect("t0_user is not set"),

            t1_user: self.instruction.t1_user.expect("t1_user is not set"),

            master: self.instruction.master.expect("master is not set"),

            signer: self.instruction.signer.expect("signer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            logger: self.instruction.logger.expect("logger is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct DradexSwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    swap_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pair: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_queue: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    dex_user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market_user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bids: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asks: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    t0_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    t1_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    t0_user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    t1_user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    master: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    logger: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
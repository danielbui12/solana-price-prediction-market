pub mod curve;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "fluster_trading",
    project_url: "https://github.com/danielbui12/fluster_trading",
    contacts: "link:huytung139@gmail.com",
    policy: "https://github.com/danielbui12/fluster_trading",
    source_code: "https://github.com/danielbui12/fluster_trading",
    preferred_languages: "en",
    auditors: "#"
}

#[cfg(feature = "devnet")]
declare_id!("HdNeVJt9x8p5G5Q99A3PySR4bNnzaLzHdSAw5B5eWZzC");
#[cfg(not(feature = "devnet"))]
declare_id!("HdNeVJt9x8p5G5Q99A3PySR4bNnzaLzHdSAw5B5eWZzC");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("EgwWVewxT4qrvkSpfx3T6hMUztGZPR8XiAGRiYGKdUc7");
    #[cfg(not(feature = "devnet"))]
    declare_id!("EgwWVewxT4qrvkSpfx3T6hMUztGZPR8XiAGRiYGKdUc7");
}

pub mod pool_fee_receiver {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("Kd8e8t428wuB68bpksHTqu4VbM97cqYa3AKP3osYsKH");
    #[cfg(not(feature = "devnet"))]
    declare_id!("Kd8e8t428wuB68bpksHTqu4VbM97cqYa3AKP3osYsKH");
}

pub const AUTH_SEED: &str = "vault_auth_seed";
pub const USER_SEED: &str = "user_auth_seed";
pub const CLOCK_WORK_FEE: u64 = LAMPORTS_PER_SOL / 100;

#[program]
pub mod fluster_trading {
    use super::*;

    /// Initialize token pool
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `max_leverage` - the maximum leverage allowed
    ///
    pub fn initialize(
        ctx: Context<Initialize>,
        max_leverage: u8,
        protocol_fee_rate: u16,
    ) -> Result<()> {
        instructions::initialize(ctx, max_leverage, protocol_fee_rate)
    }

    /// Update pool state
    /// Must be called by the current admin
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `param`- The value can be 0, otherwise will report a error
    /// * `value`- The value of the equivalent field
    ///
    pub fn update_pool_state(ctx: Context<UpdatePoolState>, param: u8, value: u64) -> Result<()> {
        instructions::update_pool_state(ctx, param, value)
    }

    /// User deposits token to vault
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `amount` - Amount to deposit
    ///
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    /// User withdraws token from vault
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `amount` - Amount to withdraw
    ///
    pub fn withdraw(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    /// User close account
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    ///
    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        instructions::close_account(ctx)
    }

    /// Place an betting order for the given token pool
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `amount` - the amount to bet
    /// * `trade_direction` - the price trading. 1 for up, 0 for down
    ///
    pub fn betting(
        ctx: Context<Betting>,
        thread_id: Vec<u8>,
        trade_direction: u8,
        leverage: u8,
        amount: u64,
        duration: u64,
    ) -> Result<()> {
        instructions::betting(ctx, thread_id, trade_direction, leverage, amount, duration)
    }

    /// Cancel the betting order
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    ///
    pub fn cancel(ctx: Context<Betting>) -> Result<()> {
        // instructions::cancel(ctx)
        Ok(())
    }

    /// Reveal the order after the deadline
    /// Must be called by the current admin
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    ///
    pub fn reveal(ctx: Context<Betting>) -> Result<()> {
        // instructions::reveal(ctx)
        Ok(())
    }

    /// Claim the order after revealed
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    ///
    pub fn claim(ctx: Context<Betting>) -> Result<()> {
        // instructions::reveal(ctx)
        Ok(())
    }
}

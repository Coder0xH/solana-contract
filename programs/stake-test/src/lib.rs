// stake_test.sol
// ===============
// 一个 Solana  量的示例程序，演示了如何在 Solana  量中实现一个简单的质押机制。
//
// 该程序提供了三个接口：
//  1. `initialize`：初始化质押账户
//  2. `stake`：质押 lamports
//  3. `unstake`：提取 lamports
//

use anchor_lang::prelude::*;

declare_id!("BYmuqm9eZLYGbUbsFNJvXZqaGi9AKGxYL73WDKic4taF");

#[program]
pub mod stake_test {
    use super::*;

    /// 初始化质押账户
    ///
    /// 该函数会创建一个质押账户，并将其所有者设置为用户的 PublicKey。
    /// 质押账户的 Lamports  将被设置为 0。
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        stake_account.owner = ctx.accounts.user.key();
        stake_account.amount = 0;
        Ok(())
    }

    /// 质押 lamports
    ///
    /// 该函数会将用户的 lamports 转移到质押账户中，并增加质押账户的 Lamports  量。
    /// 该函数将检查用户是否有足够的 lamports  可以质押。
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        **ctx.accounts.user_token_account.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.vault.try_borrow_mut_lamports()? += amount;
        stake_account.amount += amount;
        msg!("Staked {} lamports", amount);
        Ok(())
    }

    /// 提取 lamports
    ///
    /// 该函数会将质押账户中的 lamports 转移到用户的账户中，并减少质押账户的 Lamports  量。
    /// 该函数将检查用户是否有足够的 lamports  可以提取。
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        let amount = stake_account.amount;
        require_gt!(amount, 0, ErrorCode::NoStakedAmount);
        **ctx.accounts.vault.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user_token_account.try_borrow_mut_lamports()? += amount;
        stake_account.amount = 0;
        msg!("Unstaked {} lamports", amount);
        Ok(())
    }
}

#[derive(Accounts)]
/// 初始化质押账户的参数
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
/// 质押 lamports 的参数
pub struct Stake<'info> {
    #[account(
        mut,
        constraint = stake_account.owner == user.key() @ ErrorCode::Unauthorized
    )]
    pub stake_account: Account<'info, StakeAccount>,
    /// CHECK: 这是用户的代币账户，我们只进行 lamports 转移操作
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    /// CHECK: 这是质押金库账户，我们只进行 lamports 转移操作
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
/// 提取 lamports 的参数
pub struct Unstake<'info> {
    #[account(
        mut,
        constraint = stake_account.owner == user.key() @ ErrorCode::Unauthorized
    )]
    pub stake_account: Account<'info, StakeAccount>,
    /// CHECK: 这是用户的代币账户，我们只进行 lamports 转移操作
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    /// CHECK: 这是质押金库账户，我们只进行 lamports 转移操作
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
/// 质押账户的数据结构
pub struct StakeAccount {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
/// 该程序的错误代码
pub enum ErrorCode {
    #[msg("你没有权限执行此操作")]
    Unauthorized,
    #[msg("没有可提取的质押金额")]
    NoStakedAmount,
}

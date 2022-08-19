use anchor_lang::prelude::*;
use mango::{matching::Book, state::PerpMarket};
use std::cmp::min;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod idontsee {
    use super::*;

    pub fn mango_guard(ctx: Context<MangoGuard>, is_long: bool, size: u64, price_limit: u64) -> Result<()> {
        let perp_market = PerpMarket::load_mut_checked(
            &ctx.accounts.market,
            ctx.accounts.mango_program.key,
            ctx.accounts.mango_group.key,
        )
        .unwrap();

        let order_book = Book::load_checked(
            ctx.accounts.mango_program.key,
            &ctx.accounts.bids,
            &ctx.accounts.bids,
            &perp_market,
        )
        .unwrap();

        let book_side = if is_long {
            order_book.bids
        } else {
            order_book.asks
        };

        let now =  {
            let clock = Clock::get()?;
            clock.unix_timestamp as u64
        };

        let mut outstanding_size = size as i64;
        let mut cost = 0;
        for (_, order) in book_side.iter_valid(now) {
            let filled_size = min(outstanding_size, order.quantity);
            cost += filled_size * order.price();
            outstanding_size -= filled_size;

            if outstanding_size <= 0 {
                break;
            }
        }

        let avg_price = cost as u64 / size;      
        let price_oob = if is_long {
            avg_price > price_limit
        } else {
            price_limit > avg_price
        };
        
        if price_oob {
            return Err(error!(ErrorCode::BadPrice))
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MangoGuard<'info> {
    /// CHECK: ( ͡° ͜ʖ ͡°)
    market: AccountInfo<'info>,
    /// CHECK: ( ͡° ͜ʖ ͡°)
    asks: AccountInfo<'info>,
    /// CHECK: ( ͡° ͜ʖ ͡°)
    bids: AccountInfo<'info>,
    /// CHECK: ( ͡° ͜ʖ ͡°)
    mango_group: AccountInfo<'info>,
    /// CHECK: ( ͡° ͜ʖ ͡°)
    mango_program: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("BadPrice")]
    BadPrice,
}

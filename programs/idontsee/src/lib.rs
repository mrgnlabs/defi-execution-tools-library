use anchor_lang::prelude::*;
use mango::{matching::Book, state::PerpMarket};
use std::cmp::min;

#[cfg(feature="mainnet-beta")]
declare_id!("iseejmLobW1224pCCMHPhSBdvDjtN6pYXeRfz1gv4zB");
#[cfg(not(feature="mainnet-beta"))]
declare_id!("HqcuJfLApXYSrEqjrtPJ1YUBzAiBjSDB8UpHCx8Ka83U");

#[program]
pub mod idontsee {
    use super::*;

    pub fn mango_guard(ctx: Context<MangoGuard>, is_long: bool, size: u64, price_limit: u64) -> Result<()> {
        msg!("Long: {} Size: {} Limit: {}", is_long, size, price_limit);
        let perp_market = PerpMarket::load_mut_checked(
            &ctx.accounts.market,
            ctx.accounts.mango_program.key,
            ctx.accounts.mango_group.key,
        )
        .map_err(|_| error!(ErrorCode::MangoAccountError))?;

        let order_book = Book::load_checked(
            ctx.accounts.mango_program.key,
            &ctx.accounts.bids,
            &ctx.accounts.asks,
            &perp_market,
        )
        .map_err(|_| error!(ErrorCode::MangoAccountError))?;

        let book_side = if is_long {
            order_book.bids
        } else {
            order_book.asks
        };

        let now =  {
            let clock = Clock::get()?;
            clock.unix_timestamp as u64
        };

        let size_lots = size as i64 / perp_market.base_lot_size;
        msg!("size_lots: {}, base_per_lot {}", size_lots, perp_market.base_lot_size);
        let mut outstanding_lots = size_lots;
        let mut cost = 0;
        for (_, order) in book_side.iter_valid(now) {
            let filled_lots = min(outstanding_lots, order.quantity);
            let price = order.price();
            msg!("Order price {} size {} outstanding {}", price, order.quantity, outstanding_lots);
            cost += filled_lots * price;
            outstanding_lots -= filled_lots;

            if outstanding_lots <= 0 {
                break;
            }
        }

        let avg_price = (cost / size_lots) as u64;      
        let price_oob = if is_long {
            avg_price > price_limit
        } else {
            price_limit > avg_price
        };

        msg!("Avg Price: {} Price OOB: {}", avg_price, price_oob);
        
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
    #[msg("MangoAccountError")]
    MangoAccountError,
}

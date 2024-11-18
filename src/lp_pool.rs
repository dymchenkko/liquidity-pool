use std::error::Error;

#[derive(Debug)]
pub struct TokenAmount(pub u64);
#[derive(Debug)]
pub struct Price(pub u64);

impl Price {
    fn is_zero(&self) -> bool {
        return self.0 == 0;
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Percentage(pub u64);
#[derive(Debug)]
pub struct LpTokenAmount(pub u64);
#[derive(Debug)]
pub struct StakedTokenAmount(pub u64);
#[derive(Debug)]
pub struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    fee_min: Percentage,
    fee_max: Percentage,
}

impl LpPool {
    pub fn init(
        price: Price,
        fee_min: Percentage,
        fee_max: Percentage,
        liquidity_target: TokenAmount,
    ) -> Result<Self, Box<dyn Error>> {
        if price.is_zero() {
            return Err("Price should be higher than zero".into());
        }

        if fee_min > fee_max {
            return Err("Min fee can't be higher than max fee".into());
        }
        Ok(LpPool {
            price,
            token_amount: TokenAmount(0),
            st_token_amount: StakedTokenAmount(0),
            lp_token_amount: LpTokenAmount(0),
            liquidity_target,
            fee_min,
            fee_max,
        })
    }
    pub fn add_liquidity(
        &mut self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Box<dyn Error>> {
        let lp_tokens_issued = if self.lp_token_amount.0 == 0 {
            token_amount.0
        } else {
            let total =
                self.token_amount.0 + self.st_token_amount.0 * self.price.0 / 1000000; // dividing by 10^6, because inputs were multiplied by 10^6
            self.lp_token_amount.0 * token_amount.0 / total
        };
        self.token_amount.0 += token_amount.0;
        self.lp_token_amount.0 += lp_tokens_issued;
        Ok(LpTokenAmount(lp_tokens_issued))
    }

    pub fn swap(&mut self, st_token_amount: StakedTokenAmount) -> Result<u64, Box<dyn Error>> {
        if st_token_amount.0 <= 0 {
            return Err("Swap amount must be greater than zero.".into());
        }
        //TODO find better solution than multiplying inputs by 10^6
        let new_token_amount = self.price.0 * st_token_amount.0 / 1_000_000; // dividing by 10^6, because inputs were multiplied by 10^6

        let fee_percentage = self.calculate_fee(new_token_amount);

        let amount_after_fee = new_token_amount * fee_percentage / 1_000_000;// dividing by 10^6, because inputs were multiplied by 10^6
        let token_amount_to_swap = new_token_amount - amount_after_fee;
        let new_token_amount = self.token_amount.0 - token_amount_to_swap;

        self.token_amount.0 = new_token_amount;
        self.st_token_amount.0 += st_token_amount.0;

        Ok(token_amount_to_swap)
    }

    fn calculate_fee(&self, trade_amount: u64) -> u64 {
        if self.liquidity_target.0 < self.token_amount.0 - trade_amount {
            return self.fee_min.0;
        }
        let fee_rate = self.fee_max.0
            - (self.fee_max.0 - self.fee_min.0) * (self.token_amount.0 - trade_amount)
                / self.liquidity_target.0;
        fee_rate
    }

    pub fn remove_liquidity(
        &mut self,
        lp_tokens: LpTokenAmount,
    ) -> Result<(u64, u64), Box<dyn Error>> {
        if lp_tokens.0 > self.lp_token_amount.0 || lp_tokens.0 <= 0 {
            return Err("Invalid LP token amount for removal.".into());
        }
        let withdraw_token_amount = self.token_amount.0 * lp_tokens.0 / self.lp_token_amount.0;
        let withdraw_st_token_amount = self.st_token_amount.0 * lp_tokens.0 / self.lp_token_amount.0;
        self.token_amount.0 -= withdraw_token_amount;
        self.st_token_amount.0 -= withdraw_st_token_amount;
        Ok((withdraw_token_amount, withdraw_st_token_amount))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn init_price_zero() {
        LpPool::init(Price(0), Percentage(0), Percentage(0), TokenAmount(0)).unwrap();
    }

    #[test]
    #[should_panic]
    fn init_min_max_fee() {
        LpPool::init(
            Price(1_000_00),
            Percentage(2000),
            Percentage(1000),
            TokenAmount(0),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn remove_liquidity_with_lp_token_amount_zero() {
        let mut lp_pool = LpPool::init(
            Price(1_000_00),
            Percentage(1000),
            Percentage(2000),
            TokenAmount(0),
        )
        .unwrap();

        lp_pool.remove_liquidity(LpTokenAmount(1_000_000)).unwrap();
    }
    #[test]
    #[should_panic]
    fn swap_zero() {
        let mut lp_pool = LpPool::init(
            Price(1_000_00),
            Percentage(1000),
            Percentage(2000),
            TokenAmount(0),
        )
        .unwrap();

        lp_pool.add_liquidity(TokenAmount(10_000_000)).unwrap();
        lp_pool.swap(StakedTokenAmount(0)).unwrap();
    }
    #[test]
    fn swap() {
        let mut lp_pool = LpPool::init(
            Price(1_000_00),
            Percentage(1000),
            Percentage(2000),
            TokenAmount(0),
        )
        .unwrap();

        lp_pool.add_liquidity(TokenAmount(10_000_000)).unwrap();
        lp_pool.swap(StakedTokenAmount(2_000_000)).unwrap();
    }

    #[test]
    fn remove_liquidity() {
        let mut lp_pool = LpPool::init(
            Price(1_000_00),
            Percentage(1000),
            Percentage(2000),
            TokenAmount(0),
        )
        .unwrap();

        lp_pool.add_liquidity(TokenAmount(10_000_000)).unwrap();
        lp_pool.swap(StakedTokenAmount(30_000_000)).unwrap();
        lp_pool.remove_liquidity(LpTokenAmount(1_000_000)).unwrap();
    }
}

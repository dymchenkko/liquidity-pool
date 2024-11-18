mod lp_pool;
use lp_pool::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // All inputs multiplied in 10^6, so are outputs
    let mut lp_pool = LpPool::init(
        Price(1_500_000),
        Percentage(1000),
        Percentage(90000),
        TokenAmount(90_000_000),
    )?;
    println!(
        "lp_pool.add_liquidity(100.0 Token) -> {:?}",
        lp_pool.add_liquidity(TokenAmount(100_000_000)).unwrap()
    );
    println!(
        "lp_pool.swap(6 StakedToken) -> {:?}",
        lp_pool.swap(StakedTokenAmount(6_000_000)).unwrap()
    );
    println!(
        "lp_pool.add_liquidity(10.0 Token) -> {:?}",
        lp_pool.add_liquidity(TokenAmount(10_000_000)).unwrap()
    );
    println!(
        "lp_pool.swap(30.0 StakedToken) -> {:?}",
        lp_pool.swap(StakedTokenAmount(30_000_000)).unwrap()
    );
    println!(
        "lp_pool.remove_liquidity(109.9991) -> {:?}",
        lp_pool
            .remove_liquidity(LpTokenAmount(109_999_100))
            .unwrap()
    );
    Ok(())
}

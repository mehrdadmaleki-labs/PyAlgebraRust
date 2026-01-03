//pure Rust logic, no PyO3.

use num_bigint::BigUint;
use num_traits::One;

fn product_range(lo: u64, hi: u64) -> BigUint {
    let mut acc = BigUint::one();
    if lo > hi {
        return acc;
    }
    for k in lo..=hi {
        acc *= BigUint::from(k);
    }
    acc
}

pub async fn factorial_async_4(n: u64) -> BigUint {
    if n <= 1 {
        return BigUint::one();
    }
    if n <= 5 {
        return product_range(2, n);
    }

    let q = n / 4;
    let (a_lo, a_hi) = (2, q);
    let (b_lo, b_hi) = (q + 1, 2 * q);
    let (c_lo, c_hi) = (2 * q + 1, 3 * q);
    let (d_lo, d_hi) = (3 * q + 1, n);

    let ta = tokio::task::spawn_blocking(move || product_range(a_lo, a_hi));
    let tb = tokio::task::spawn_blocking(move || product_range(b_lo, b_hi));
    let tc = tokio::task::spawn_blocking(move || product_range(c_lo, c_hi));
    let td = tokio::task::spawn_blocking(move || product_range(d_lo, d_hi));

    let (pa, pb, pc, pd) = tokio::try_join!(ta, tb, tc, td).expect("spawn_blocking task panicked");

    pa * pb * pc * pd
}

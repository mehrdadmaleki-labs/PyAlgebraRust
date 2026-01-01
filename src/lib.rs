use pyo3::prelude::*;

/// A Python module implemented in Rust.

#[pymodule]
mod linalg_lib {
    use num_bigint::BigUint;
    use num_traits::One;
    use pyo3::prelude::*;
    use tokio::task;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    fn product_range(lo: u64, hi: u64) -> BigUint {
        // Product of lo..=hi (inclusive). If lo > hi => 1
        let mut acc = BigUint::one();
        if lo > hi {
            return acc;
        }
        for k in lo..=hi {
            acc *= BigUint::from(k);
        }
        acc
    }

    async fn factorial_async_4(n: u64) -> BigUint {
        if n <= 1 {
            return BigUint::one();
        }
        if n <= 5 {
            return product_range(2, n);
        }

        let q = n / 4;

        // 4 chunks:
        // [2..q], [q+1..2q], [2q+1..3q], [3q+1..n]
        let (a_lo, a_hi) = (2, q);
        let (b_lo, b_hi) = (q + 1, 2 * q);
        let (c_lo, c_hi) = (2 * q + 1, 3 * q);
        let (d_lo, d_hi) = (3 * q + 1, n);

        let ta = task::spawn_blocking(move || product_range(a_lo, a_hi));
        let tb = task::spawn_blocking(move || product_range(b_lo, b_hi));
        let tc = task::spawn_blocking(move || product_range(c_lo, c_hi));
        let td = task::spawn_blocking(move || product_range(d_lo, d_hi));

        let (pa, pb, pc, pd) =
            tokio::try_join!(ta, tb, tc, td).expect("spawn_blocking task panicked");

        pa * pb * pc * pd
    }

    /// Python-callable wrapper.
    /// Returns a Python int (arbitrary precision) via BigUint.
    #[pyfunction]
    fn factorial_parallel(py: Python<'_>, n: u64) -> PyResult<Py<PyAny>> {
        // Compute in Rust (release GIL during CPU work)
        let big: num_bigint::BigUint = py.allow_threads(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok::<_, PyErr>(rt.block_on(factorial_async_4(n)))
        })?;

        // Convert BigUint -> decimal string -> Python int via builtins.int(...)
        let s = big.to_str_radix(10);
        let builtins = py.import("builtins")?;
        let py_int = builtins.getattr("int")?.call1((s,))?;

        Ok(py_int.into())
    }
}

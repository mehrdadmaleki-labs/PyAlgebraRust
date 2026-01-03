use pyo3::prelude::*;

mod factorial;

// This is the Python-facing function (registered by lib.rs)
#[pyfunction]
pub fn factorial_parallel(py: Python<'_>, n: u64) -> PyResult<Py<PyAny>> {
    let big: num_bigint::BigUint = py.detach(|| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok::<_, PyErr>(rt.block_on(factorial::factorial_async_4(n)))
    })?;

    // BigUint -> string -> Python int
    let s = big.to_str_radix(10);
    let builtins = py.import("builtins")?;
    let py_int = builtins.getattr("int")?.call1((s,))?;

    Ok(py_int.into())
}

use pyo3::prelude::*;

mod combinatorics;
//mod linalg;
//mod numbers;

#[pymodule]
fn pyalgebrarust(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(combinatorics::factorial_parallel, m)?)?;
    Ok(())
}

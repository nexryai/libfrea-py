use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod domains;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


/// This function takes a domain as an argument and checks if it exists in the DOMAINS list.
#[pyfunction]
fn is_blocked(domain: &str) -> PyResult<bool> {
    Ok(domains::DOMAINS.contains(&domain))
}

/// This module is a Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "libfrea")]
fn pyo3_domain_checker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_blocked, m)?)?;

    Ok(())
}

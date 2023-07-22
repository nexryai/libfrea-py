use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod domains;
mod mute;

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


#[pyfunction]
fn is_blocked(domain: &str) -> PyResult<bool> {
    Ok(domains::BLOCKED.contains(&domain))
}

#[pyfunction]
fn is_muted(domain: &str) -> PyResult<bool> {
    Ok(mute::UNTRUSTED.contains(&domain))
}

/// This module is a Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "libfrea")]
fn pyo3_domain_checker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_blocked, m)?)?;
    m.add_function(wrap_pyfunction!(is_muted, m)?)?;

    Ok(())
}

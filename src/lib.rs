mod service;
mod chat_service;

use pyo3::{prelude::*, wrap_pyfunction};
use service::run_peer;
use chat_service::chat;


#[pyfunction]
fn start_ping_service(py: Python<'_>, address: Option<String>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        run_peer(address).await;
        Ok("DONE".to_string())
    })
}

#[pyfunction]
fn start_chat_service(py: Python<'_>, address: Option<String>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        chat(address).await.unwrap();
        Ok("DONE".to_string())
    })
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_ping_service,m)?)?;
    m.add_function(wrap_pyfunction!(start_chat_service, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
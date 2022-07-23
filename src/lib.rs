mod service;

use pyo3::{prelude::*, wrap_pyfunction};
use service::run_peer;


#[pyfunction]
fn start_ping_service(py: Python<'_>, address: Option<String>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        run_peer(address).await;
        // async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        Ok("DONE".to_string())
    })
    // pyo3_asyncio::async_std::into_future(run_peer()).await.map(|_| "Started ping service".to_string());
    // py.allow_threads(service::run_peer);
    // executor::block_on(service::run_peer());
    // Ok("DONE".to_string())
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
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
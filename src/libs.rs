// #[pyfunction]
fn abc() {
    println!("Hello, world!");
}

#[pymodule]
#[pyo3(name = "gha-test")]
fn allen(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_wrapped(wrap_pyfunction!(abc)).unwrap();
    Ok(())
}

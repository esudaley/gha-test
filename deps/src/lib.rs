use std::convert::From;


use geographiclib_rs::{DirectGeodesic, Geodesic, InverseGeodesic};


use polars::prelude::*;

use rayon::prelude::*;

use pyo3::exceptions::PyValueError;

use pyo3::prelude::*;

use pyo3::types::{PyFloat, PyInt};

use pyo3::PyObject;
// 
use geos::Geometry as GGeometry;

use polars::prelude::*;


// #[pyfunction]
fn abc() {
    println!("Hello, world!");
}

#[pymodule]
#[pyo3(name = "allen")]
fn allen(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    Ok(())
}

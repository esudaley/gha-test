use std::convert::From;

use std::f64::consts::PI;

use std::f64::consts::{FRAC_PI_6, PI};

use geographiclib_rs::{DirectGeodesic, Geodesic, InverseGeodesic};

use geos::{CoordSeq, GResult, Geom as GGeom, Geometry as GGeometry};

use hashbrown::{HashMap, HashSet};

use itertools::{enumerate, Itertools};

use itertools_num::linspace;

use log::info;

use ordered_float::OrderedFloat;

use linfa::dataset::{Dataset, DatasetBase};

use linfa::metrics::SilhouetteScore;

use linfa::traits::{Fit, Predict};

use linfa_clustering::{KMeans, KMeansInit};

use linfa_nn::distance::L2Dist;

use rand_xoshiro::Xoshiro256Plus;

use ndarray::prelude::*;

use polars::prelude::*;

use rand_xoshiro::rand_core::SeedableRng;

use rayon::prelude::*;

use pyo3::exceptions::PyValueError;

use pyo3::prelude::*;

use pyo3::types::{PyFloat, PyInt};

use pyo3::PyObject;

// use error handling and don't panic! on everything

use std::borrow::Cow;

used in loop to determine step size in angle

use &&

use it

use geos::Geometry as GGeometry;

use polars::prelude::*;

use crate::*;

// #[pyfunction]
fn abc() {
    println!("Hello, world!");
}

#[pymodule]
#[pyo3(name = "allen")]
fn allen(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_wrapped(wrap_pyfunction!(abc)).unwrap();
    Ok(())
}

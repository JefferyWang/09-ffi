use std::fmt::{Display, Formatter};

use algo::{multiply, Matrix};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass(name = "Matrix")]
pub struct PyMatrix {
    inner: Matrix<f64>,
}

#[pymethods]
impl PyMatrix {
    #[new]
    pub fn try_new(data: Vec<Vec<f64>>) -> PyResult<Self> {
        if data.is_empty() || data[0].is_empty() {
            return Err(PyValueError::new_err(
                "row should not be empty or column should not be empty",
            ));
        }
        let row = data.len();
        let col = data[0].len();
        let data: Vec<_> = data.into_iter().flatten().collect();
        Ok(Self {
            inner: Matrix::new(data, row, col),
        })
    }

    pub fn mul(&self, other: &PyMatrix) -> PyResult<Self> {
        let result = multiply(&self.inner, &other.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: result })
    }

    pub fn multiply(&self, other: Vec<Vec<f64>>) -> PyResult<Self> {
        let other = PyMatrix::try_new(other)?;
        self.mul(&other)
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}

impl Display for PyMatrix {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

use std::fmt::{Display, Formatter};

use pyo3::{exceptions::PyValueError, prelude::*};
use roaring::RoaringBitmap;

#[pyclass(name = "Bitmap")]
pub struct PyBitmap {
    inner: RoaringBitmap,
    iter: Box<dyn Iterator<Item = u32> + Send>,
}

#[pymethods]
impl PyBitmap {
    #[new]
    pub fn new() -> Self {
        let map = RoaringBitmap::new();
        let iter = Box::new(map.clone().into_iter());
        Self { inner: map, iter }
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.inner.is_subset(&other.inner)
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.inner.is_superset(&other.inner)
    }

    #[staticmethod]
    pub fn full() -> Self {
        let map = RoaringBitmap::full();
        let iter = Box::new(map.clone().into_iter());
        Self { inner: map, iter }
    }

    pub fn insert(&mut self, value: u32) -> bool {
        self.inner.insert(value)
    }

    pub fn insert_range(&mut self, start: u32, end: u32) -> u64 {
        self.inner.insert_range(start..end)
    }

    pub fn push(&mut self, value: u32) -> bool {
        self.inner.insert(value)
    }

    pub fn remove(&mut self, value: u32) -> bool {
        self.inner.remove(value)
    }

    pub fn remove_range(&mut self, start: u32, end: u32) -> u64 {
        self.inner.remove_range(start..end)
    }

    pub fn contains(&self, value: u32) -> bool {
        self.inner.contains(value)
    }

    pub fn contains_range(&self, start: u32, end: u32) -> bool {
        self.inner.contains_range(start..end)
    }

    pub fn range_cardinality(&self, start: u32, end: u32) -> u64 {
        self.inner.range_cardinality(start..end)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.inner.is_full()
    }

    pub fn len(&self) -> u64 {
        self.inner.len()
    }

    pub fn min(&self) -> Option<u32> {
        self.inner.min()
    }

    pub fn max(&self) -> Option<u32> {
        self.inner.max()
    }

    pub fn rank(&self, value: u32) -> u64 {
        self.inner.rank(value)
    }

    pub fn select(&self, rank: u32) -> Option<u32> {
        self.inner.select(rank)
    }

    pub fn remove_smallest(&mut self, n: u64) {
        self.inner.remove_smallest(n)
    }

    pub fn remove_biggest(&mut self, n: u64) {
        self.inner.remove_biggest(n)
    }

    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u32> {
        slf.iter.next()
    }

    #[staticmethod]
    pub fn from_iter(data: Vec<u32>) -> Self {
        let map = RoaringBitmap::from_iter(data);
        let iter = Box::new(map.clone().into_iter());
        Self { inner: map, iter }
    }

    #[staticmethod]
    pub fn from_sorted_iter(mut data: Vec<u32>) -> PyResult<Self> {
        data.sort();
        let map = RoaringBitmap::from_sorted_iter(data.into_iter()).map_err(|e| {
            PyValueError::new_err(format!(
                "Failed to create RoaringBitmap from sorted iter: {}",
                e
            ))
        })?;
        let iter = Box::new(map.clone().into_iter());
        Ok(Self { inner: map, iter })
    }

    pub fn append(&mut self, mut data: Vec<u32>) -> PyResult<u64> {
        data.sort();
        match self.inner.append(data) {
            Ok(n) => Ok(n),
            Err(e) => Err(PyValueError::new_err(format!("Failed to append: {}", e))),
        }
    }

    pub fn intersection_len(&self, other: &Self) -> u64 {
        self.inner.intersection_len(&other.inner)
    }

    pub fn union_len(&self, other: &Self) -> u64 {
        self.inner.union_len(&other.inner)
    }

    pub fn difference_len(&self, other: &Self) -> u64 {
        self.inner.difference_len(&other.inner)
    }

    pub fn symmetric_difference_len(&self, other: &Self) -> u64 {
        self.inner.symmetric_difference_len(&other.inner)
    }

    pub fn get_items(&self) -> Vec<u32> {
        self.inner.clone().into_iter().collect()
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self)
    }
}

impl Display for PyBitmap {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Default for PyBitmap {
    fn default() -> Self {
        Self::new()
    }
}

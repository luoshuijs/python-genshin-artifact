extern crate core;
mod applications;

use applications::wasm::{get_damage_analysis, get_transformative_damage};
use applications::generate::character::gen_character_meta_as_json;
use pyo3::prelude::*;



#[pymodule]
fn genshin_artifact_function(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_damage_analysis, m)?)?;
    m.add_function(wrap_pyfunction!(get_transformative_damage, m)?)?;
    m.add_function(wrap_pyfunction!(gen_character_meta_as_json, m)?)?;
    Ok(())
}
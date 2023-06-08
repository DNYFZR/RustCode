// Python bindings for Rust code...

// Dependencies
use serde_json;

// Python module implementation
use pyo3::{prelude::*};

#[pymodule]
fn rustython(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
   
    Ok(())
}

// Module functions
#[pyfunction]
#[pyo3(signature = (url, header_key, header_value))]
fn get(url: &str, header_key:&str, header_value:&str,) -> PyResult<String>{
    let json_data = serde_json::json!({
        "a_key" : 0,
        "b_key" : [1,2,3],
        "headers": [header_key, header_value],
        "URL": url
    });

    Ok(json_data.as_str().unwrap().to_string())
}

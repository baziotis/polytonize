#![cfg(test)]
use pyo3::Python;
use pyo3::prelude::{PyAnyMethods, PyModule};

pub fn setup_python_paths() {
    let cwd = std::env::current_dir().unwrap();
    let pos_module_path = cwd.join("pos/src");
    let lib_path = cwd.join("pos/.venv/lib/python3.13/site-packages");

    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys").unwrap();
        let path = sys.getattr("path").unwrap();
        path.call_method1("append", (pos_module_path.display().to_string(),))
            .unwrap();
        path.call_method1("append", (lib_path.display().to_string(),))
            .unwrap();
    })
}

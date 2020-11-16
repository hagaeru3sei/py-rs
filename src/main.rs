use numpy::PyArray1;
//use pyo3::prelude::{PyResult, Python};
use pyo3::types::IntoPyDict;
use pyo3::types::PyDict;
use pyo3::types::PyList;
use pyo3::prelude::*;
use std::env;

fn main() -> PyResult<()> {
    // v 0.12
    //Python::with_gil(|py| {
    //    let np = py.import("numpy")?;
    //    let locals = [("np", np)].into_py_dict(py);
    //    let pyarray: &PyArray1<i32> = py
    //        .eval("np.absolute(np.array([-1, -2, -3], dtype='int32'))", Some(locals), None)?
    //        .extract()?;
    //    let readonly = pyarray.readonly();
    //    let slice = readonly.as_slice()?;
    //    assert_eq!(slice, &[1, 2, 3]);
    //    Ok(())
    //})

    let gil = Python::acquire_gil();
    let py = gil.python();
    let path: &PyList = py.import("sys").unwrap().get("path").unwrap().try_into().unwrap();
    // Append venv dir
    path.insert(0, "/Users/nmochizuki/work/local/repos/py-rs/venv39/lib/python3.9/site-packages").unwrap();
    let root_dir = env::current_dir()?;
    path.insert(0, format!("{}", root_dir.display())).unwrap();
    println!("{}", path);
    let hello = py.import("hello")?;
    let res: String = hello.call1("say", ())?.extract()?;
    println!("{}", res);
    //let _np = py.import("numpy")?;
    //println!("{}", _np);
    Ok(())
}

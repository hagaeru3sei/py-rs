use pyo3::{
    types::{
        PyBytes, PyDict, PyList, IntoPyDict
    }, 
    prelude::*
};
use std::env;

//fn main() -> PyResult<()> {
//    // v 0.12
//    Python::with_gil(|py| {
//        let np = py.import("numpy")?;
//        let locals = [("np", np)].into_py_dict(py);
//        let pyarray: &PyArray1<i32> = py
//            .eval("np.absolute(np.array([-1, -2, -3], dtype='int32'))", Some(locals), None)?
//            .extract()?;
//        let readonly = pyarray.readonly();
//        let slice = readonly.as_slice()?;
//        assert_eq!(slice, &[1, 2, 3]);
//        Ok(())
//    })
//    Ok(())
//}

fn main() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let path: &PyList = py.import("sys").unwrap().get("path").unwrap().try_into().unwrap();
    // Append venv dir
    let root_dir = env::current_dir()?;
    path.insert(0, format!("{}", root_dir.display())).unwrap();
    println!("{}", path);

    let hello = py.import("hello")?;
    let res: String = hello.call1("say", ())?.extract()?;
    println!("{}", res);

    //let _np = py.import("numpy")?;
    //println!("{}", _np);

    let locals = PyDict::new(py);
    py.run(r#"
try:
    import base64
    s = 'Hello Rust!'
    ret = base64.b64encode(s.encode('utf-8'))
except ImportError as e:
    ret = b"SGVsbG8gUnVzdCE="
    print(e)
"#, None, Some(locals)).unwrap();
    let ret = locals.get_item("ret").unwrap();
    let b64: &PyBytes = ret.downcast().unwrap();
    assert_eq!(b64.as_bytes(), b"SGVsbG8gUnVzdCE=");

    let hello = py.import("hello")?;
    hello.call1("load", ())?;

    Ok(())
}

//fn main_(py: Python) -> PyResult<()> {
//    let sys = py.import("sys")?;
//    let version: String = sys.get("version")?.extract()?;
//    let locals = [("os", py.import("os")?)].into_py_dict(py);
//    let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
//    let user: String = py.eval(code, None, Some(&locals))?.extract()?;
//    println!("Hello {}, I'm Python {}", user, version);
//    Ok(())
//}
//
//fn main() -> Result<(), ()> {
//    Python::with_gil(|py| {
//        main_(py).map_err(|e| {
//          // We can't display Python exceptions via std::fmt::Display,
//          // so print the error here manually.
//          e.print_and_set_sys_last_vars(py);
//        })
//    })
//}

use pyo3::FromPyObject;

#[derive(FromPyObject)]
pub enum BytesOrStr {
    #[pyo3(transparent)]
    Str(String),
    #[pyo3(transparent)]
    Bytes(Vec<u8>),
}

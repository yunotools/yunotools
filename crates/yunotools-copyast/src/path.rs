use std::path::PathBuf;

pub fn normalize(path: &str) -> PathBuf {
    let p = PathBuf::from(path);

    match p.canonicalize() {
        Ok(abs) => abs,
        Err(_) => p,
    }
}

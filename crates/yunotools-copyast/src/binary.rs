use std::path::Path;

pub fn is_binary(path: &Path) -> bool {
    let data = match std::fs::read(path) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let sample = data.iter().take(8000);

    for byte in sample {
        if *byte == 0 {
            return true;
        }
    }

    false
}

use super::{default, docker, go, node, python, rust};

pub fn generate() -> String {
    let mut result = String::new();

    result.push_str("\n# =========================\n# DEFAULT\n# =========================\n");

    result.push_str(default::TEMPLATE);

    result.push_str("\n# =========================\n# RUST\n# =========================\n");

    result.push_str(rust::TEMPLATE);

    result.push_str("\n# =========================\n# NODE\n# =========================\n");

    result.push_str(node::TEMPLATE);

    result.push_str("\n# =========================\n# PYTHON\n# =========================\n");

    result.push_str(python::TEMPLATE);

    result.push_str("\n# =========================\n# GO\n# =========================\n");

    result.push_str(go::TEMPLATE);

    result.push_str("\n# =========================\n# DOCKER\n# =========================\n");

    result.push_str(docker::TEMPLATE);

    result
}

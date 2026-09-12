use super::{
    c_cpp, csharp, dart, default, docker, elixir, go, haskell, java, kotlin, lua, node, php,
    powershell, python, ruby, rust, scala, shell, sql, swift, terraform, web,
};

pub struct TemplateDefinition {
    pub name: &'static str,
    pub title: &'static str,
    pub aliases: &'static [&'static str],
    pub content: &'static str,
}

const TEMPLATES: &[TemplateDefinition] = &[
    TemplateDefinition {
        name: "default",
        title: "DEFAULT",
        aliases: &["common", "general"],
        content: default::TEMPLATE,
    },
    TemplateDefinition {
        name: "rust",
        title: "RUST",
        aliases: &["rs"],
        content: rust::TEMPLATE,
    },
    TemplateDefinition {
        name: "node",
        title: "NODE / JAVASCRIPT / TYPESCRIPT",
        aliases: &["javascript", "js", "typescript", "ts", "nodejs", "node.js"],
        content: node::TEMPLATE,
    },
    TemplateDefinition {
        name: "python",
        title: "PYTHON",
        aliases: &["py"],
        content: python::TEMPLATE,
    },
    TemplateDefinition {
        name: "go",
        title: "GO",
        aliases: &["golang"],
        content: go::TEMPLATE,
    },
    TemplateDefinition {
        name: "java",
        title: "JAVA",
        aliases: &[],
        content: java::TEMPLATE,
    },
    TemplateDefinition {
        name: "kotlin",
        title: "KOTLIN",
        aliases: &["kt"],
        content: kotlin::TEMPLATE,
    },
    TemplateDefinition {
        name: "c-cpp",
        title: "C / C++",
        aliases: &["c", "cpp", "c++", "cplusplus"],
        content: c_cpp::TEMPLATE,
    },
    TemplateDefinition {
        name: "csharp",
        title: "C# / .NET",
        aliases: &["c#", "cs", "dotnet", ".net"],
        content: csharp::TEMPLATE,
    },
    TemplateDefinition {
        name: "php",
        title: "PHP",
        aliases: &[],
        content: php::TEMPLATE,
    },
    TemplateDefinition {
        name: "ruby",
        title: "RUBY",
        aliases: &["rb"],
        content: ruby::TEMPLATE,
    },
    TemplateDefinition {
        name: "swift",
        title: "SWIFT",
        aliases: &[],
        content: swift::TEMPLATE,
    },
    TemplateDefinition {
        name: "dart",
        title: "DART / FLUTTER",
        aliases: &["flutter"],
        content: dart::TEMPLATE,
    },
    TemplateDefinition {
        name: "shell",
        title: "SHELL",
        aliases: &["sh", "bash", "zsh"],
        content: shell::TEMPLATE,
    },
    TemplateDefinition {
        name: "powershell",
        title: "POWERSHELL",
        aliases: &["pwsh", "ps1"],
        content: powershell::TEMPLATE,
    },
    TemplateDefinition {
        name: "lua",
        title: "LUA",
        aliases: &[],
        content: lua::TEMPLATE,
    },
    TemplateDefinition {
        name: "elixir",
        title: "ELIXIR",
        aliases: &["ex"],
        content: elixir::TEMPLATE,
    },
    TemplateDefinition {
        name: "haskell",
        title: "HASKELL",
        aliases: &["hs"],
        content: haskell::TEMPLATE,
    },
    TemplateDefinition {
        name: "scala",
        title: "SCALA",
        aliases: &[],
        content: scala::TEMPLATE,
    },
    TemplateDefinition {
        name: "terraform",
        title: "TERRAFORM / OPENTOFU",
        aliases: &["tf", "opentofu", "tofu"],
        content: terraform::TEMPLATE,
    },
    TemplateDefinition {
        name: "web",
        title: "HTML / CSS",
        aliases: &["html", "css", "frontend"],
        content: web::TEMPLATE,
    },
    TemplateDefinition {
        name: "sql",
        title: "SQL / DATABASE",
        aliases: &["database"],
        content: sql::TEMPLATE,
    },
    TemplateDefinition {
        name: "docker",
        title: "DOCKER",
        aliases: &["container", "dockerfile"],
        content: docker::TEMPLATE,
    },
];

pub fn all() -> &'static [TemplateDefinition] {
    TEMPLATES
}

pub fn find(template_name: &str) -> Option<&'static TemplateDefinition> {
    let normalized_name = template_name.trim().to_ascii_lowercase();

    TEMPLATES.iter().find(|template| {
        template.name == normalized_name || template.aliases.contains(&normalized_name.as_str())
    })
}

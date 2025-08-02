use std::path::{Path, PathBuf};

use dialoguer::{FuzzySelect, Input, theme::ColorfulTheme};
use eyre::ContextCompat;
use git2::Repository;

struct Template {
    name: &'static str,
    repository: &'static str,
}

static TEMPLATES: &[Template] = &[
    Template {
        name: "Javascript (example-js)",
        repository: "https://github.com/TilePad/tilepad-example-js.git",
    },
    Template {
        name: "Typescript (example-ts)",
        repository: "https://github.com/TilePad/tilepad-example-ts.git",
    },
    Template {
        name: "Rust (example-rs)",
        repository: "https://github.com/TilePad/tilepad-example-rs.git",
    },
];

pub fn create(project_path: Option<PathBuf>) -> eyre::Result<()> {
    let theme = ColorfulTheme::default();

    let items = TEMPLATES
        .iter()
        .map(|item| item.name.to_string())
        .collect::<Vec<_>>();

    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt("Choose a project template")
        .items(&items)
        .interact()?;

    let item = TEMPLATES
        .get(selection)
        .context("selected item index out of bounds")?;

    let target_path = match project_path {
        Some(value) => value,
        None => {
            let project_name = Input::with_theme(&theme)
                .with_prompt("Choose a project name (Directory Name)")
                .default("tilepad-plugin".to_string())
                .show_default(true)
                .interact_text()?;

            Path::new(".").join(project_name)
        }
    };

    Repository::clone(item.repository, &target_path)?;

    println!("Installation complete");

    Ok(())
}

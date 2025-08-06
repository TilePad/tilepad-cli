use std::path::PathBuf;

use dialoguer::{FuzzySelect, Input, theme::ColorfulTheme};
use eyre::{Context, ContextCompat};
use git2::Repository;

struct Template {
    id: &'static str,
    name: &'static str,
    repository: &'static str,
}

static TEMPLATES: &[Template] = &[
    Template {
        id: "example-js",
        name: "Javascript",
        repository: "https://github.com/TilePad/tilepad-example-js.git",
    },
    Template {
        id: "example-ts",
        name: "Typescript",
        repository: "https://github.com/TilePad/tilepad-example-ts.git",
    },
    Template {
        id: "example-rs",
        name: "Rust",
        repository: "https://github.com/TilePad/tilepad-example-rs.git",
    },
];

pub fn create(project_path: Option<PathBuf>, template_id: Option<String>) -> eyre::Result<()> {
    // Get current directory
    let current_dir = std::env::current_dir().context("failed to get current directory")?;

    let theme = ColorfulTheme::default();

    let target_path = match project_path {
        Some(value) => value,
        None => {
            let project_name = Input::with_theme(&theme)
                .with_prompt("Project name:")
                .default("tilepad-plugin".to_string())
                .show_default(true)
                .interact_text()?;

            current_dir.join(project_name)
        }
    };

    let template = match template_id {
        Some(template_id) => TEMPLATES
            .iter()
            .find(|template| template.id == template_id)
            .with_context(|| format!("template \"{template_id}\" not found"))?,
        None => {
            let items = TEMPLATES
                .iter()
                .map(|item| format!("{} ({})", item.name, item.id))
                .collect::<Vec<_>>();

            let selection = FuzzySelect::with_theme(&theme)
                .with_prompt("Select a project template")
                .items(&items)
                .default(2)
                .interact()?;

            TEMPLATES
                .get(selection)
                .context("selected item index out of bounds")?
        }
    };

    let repo = Repository::clone(template.repository, &target_path)?;
    let git_dir = repo.path().to_path_buf();
    drop(repo);

    // Remove the git directory
    if git_dir.exists() {
        std::fs::remove_dir_all(git_dir)?;
    }

    if ["example-js", "example-ts"].contains(&template.id) {
        println!("\nProject created. Now run:\n");

        if target_path
            .parent()
            .is_some_and(|parent| parent.eq(&current_dir))
        {
            if let Some(filename) = target_path.file_name() {
                println!("  cd {}", filename.display());
            } else {
                println!("  cd {}", target_path.display());
            }
        } else {
            println!("  cd {}", target_path.display());
        }

        println!("  npm install");
    } else {
        println!("\nProject created.");
    }

    Ok(())
}

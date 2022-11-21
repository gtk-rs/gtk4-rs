use std::env;
use std::path::PathBuf;

use anyhow::Context;
use walkdir::WalkDir;
use xshell::{cmd, Shell};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("install") => install()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
install          install everything needed to run the listings
"
    )
}

fn install() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    install_schema(sh)?;
    Ok(())
}

fn install_schema(sh: Shell) -> anyhow::Result<()> {
    let schema_dir = schema_dir()?;
    println!("Create directory: {schema_dir:#?}");
    sh.create_dir(&schema_dir)?;
    for schema in schema() {
        println!("Copy schema: {schema:#?}");
        sh.copy_file(&schema, &schema_dir)?;
    }

    cmd!(sh, "glib-compile-schemas {schema_dir}").run()?;

    Ok(())
}

fn schema_dir() -> anyhow::Result<PathBuf> {
    let schema_dir = if cfg!(windows) {
        PathBuf::from("C:/ProgramData/glib-2.0/schemas/")
    } else {
        dirs::data_dir()
            .context("Could not get data dir")?
            .join("glib-2.0/schemas")
    };
    Ok(schema_dir)
}

fn schema() -> Vec<PathBuf> {
    WalkDir::new(".")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            if let Some(file_name) = entry.path().file_name() {
                if let Some(file_name) = file_name.to_str() {
                    if file_name.ends_with("gschema.xml") {
                        return Some(entry.into_path());
                    }
                }
            }
            None
        })
        .collect()
}

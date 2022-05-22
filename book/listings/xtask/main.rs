use anyhow::Context;
use std::{env, path::PathBuf};
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
    match task.as_ref().map(|it| it.as_str()) {
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
    install_scheme(sh)?;
    Ok(())
}

fn install_scheme(sh: Shell) -> anyhow::Result<()> {
    let scheme_dir = scheme_dir()?;
    println!("Create directory: {scheme_dir:#?}");
    sh.create_dir(&scheme_dir)?;
    for schema in scheme() {
        println!("Copy schema: {schema:#?}");
        sh.copy_file(&schema, &scheme_dir)?;
    }

    cmd!(sh, "glib-compile-schemas {scheme_dir}").run()?;

    Ok(())
}

fn scheme_dir() -> anyhow::Result<PathBuf> {
    let scheme_dir = if cfg!(windows) {
        PathBuf::from("C:/ProgramData/glib-2.0/schemas/")
    } else {
        dirs::data_dir()
            .context("Could not get data dir")?
            .join("glib-2.0/schemas")
    };
    Ok(scheme_dir)
}

fn scheme() -> Vec<PathBuf> {
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

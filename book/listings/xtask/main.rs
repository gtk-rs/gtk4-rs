use std::env;
use std::path::PathBuf;

use gtk::glib;
use walkdir::WalkDir;
use xshell::{Shell, cmd};

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
        Some("configure") => configure()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
install          install everything needed to run the listings
configure        configure meson projects
"
    )
}

fn install() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    let meson_dirs = meson_projects();
    configure_meson_projects(&sh, &meson_dirs)?;
    install_meson_projects(&sh, &meson_dirs)?;
    install_schemas(&sh, &meson_dirs)?;

    Ok(())
}

fn configure() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let meson_dirs = meson_projects();
    configure_meson_projects(&sh, &meson_dirs)?;
    Ok(())
}

fn configure_meson_projects(sh: &Shell, meson_dirs: &[PathBuf]) -> anyhow::Result<()> {
    let prefix = glib::user_data_dir()
        .parent()
        .expect("user_data_dir should have parent")
        .to_path_buf();

    for meson_dir in meson_dirs {
        println!("Configuring meson project: {meson_dir:?}");
        let builddir = meson_dir.join("builddir");
        cmd!(sh, "meson setup --prefix={prefix} {builddir} {meson_dir}").run()?;
    }
    Ok(())
}

fn install_meson_projects(sh: &Shell, meson_dirs: &[PathBuf]) -> anyhow::Result<()> {
    for meson_dir in meson_dirs {
        println!("Installing meson project: {meson_dir:?}");
        let builddir = meson_dir.join("builddir");
        cmd!(sh, "meson install -C {builddir}").run()?;
    }
    Ok(())
}

fn meson_projects() -> Vec<PathBuf> {
    WalkDir::new(".")
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            if entry.file_name() == "meson.build" && entry.depth() == 3 {
                return entry.path().parent().map(|p| p.to_path_buf());
            }
            None
        })
        .collect()
}

fn install_schemas(sh: &Shell, meson_dirs: &[PathBuf]) -> anyhow::Result<()> {
    let schemas: Vec<_> = schemas()
        .into_iter()
        .filter(|schema| {
            !meson_dirs
                .iter()
                .any(|meson_dir| schema.starts_with(meson_dir))
        })
        .collect();

    if schemas.is_empty() {
        return Ok(());
    }

    let schema_dir = schema_dir()?;
    println!("Create directory: {schema_dir:#?}");
    sh.create_dir(&schema_dir)?;

    for schema in schemas {
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
        gtk::glib::user_data_dir().join("glib-2.0/schemas")
    };
    Ok(schema_dir)
}

fn schemas() -> Vec<PathBuf> {
    WalkDir::new(".")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            if let Some(file_name) = entry.path().file_name()
                && let Some(file_name) = file_name.to_str()
                && file_name.ends_with("gschema.xml")
            {
                return Some(entry.into_path());
            }
            None
        })
        .collect()
}

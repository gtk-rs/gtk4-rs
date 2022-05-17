use anyhow::Context;
use std::{env, path::PathBuf};
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

    let scheme = vec![
        "actions/7/org.gtk-rs.Actions7.gschema.xml",
        "saving_window_state/1/org.gtk-rs.SavingWindowState1.gschema.xml",
        "settings/1/org.gtk-rs.Settings1.gschema.xml",
        "settings/2/org.gtk-rs.Settings2.gschema.xml",
        "todo_app/2/org.gtk-rs.Todo2.gschema.xml",
        "todo_app/3/org.gtk-rs.Todo3.gschema.xml",
        "todo_app/4/org.gtk-rs.Todo4.gschema.xml",
    ];

    install_scheme(sh, scheme)?;

    Ok(())
}

fn install_scheme(sh: Shell, scheme: Vec<&str>) -> anyhow::Result<()> {
    let scheme_dir = if cfg!(windows) {
        PathBuf::from("C:/ProgramData/glib-2.0/schemas/")
    } else {
        dirs::data_dir()
            .context("Could not get data dir")?
            .join("glib-2.0/schemas")
    };

    sh.create_dir(&scheme_dir)?;
    for schema in scheme {
        sh.copy_file(&schema, &scheme_dir)?;
    }

    cmd!(sh, "glib-compile-schemas {scheme_dir}").run()?;

    Ok(())
}

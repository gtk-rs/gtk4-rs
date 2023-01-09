use anyhow::Result;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub(crate) fn compile_blueprint(blueprint: &[u8]) -> Result<String> {
    let mut compiler = Command::new("blueprint-compiler")
        .args(&["compile", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = compiler.stdin.take().unwrap();

    stdin.write(b"using Gtk 4.0;\n")?;
    stdin.write_all(blueprint)?;

    drop(stdin);

    let mut buf = String::new();
    compiler.stdout.unwrap().read_to_string(&mut buf)?;

    Ok(buf)
}

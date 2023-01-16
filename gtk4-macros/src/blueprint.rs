// Take a look at the license at the top of the repository in the LICENSE file.

use anyhow::{bail, Result};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub(crate) fn compile_blueprint(blueprint: &[u8]) -> Result<String> {
    let mut compiler = Command::new("blueprint-compiler")
        .args(["compile", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("blueprint-compiler not found"));

    let mut stdin = compiler.stdin.take().unwrap();
    stdin.write_all(b"using Gtk 4.0;\n")?;
    stdin.write_all(blueprint)?;
    drop(stdin);

    let mut buf = String::new();
    compiler.stdout.unwrap().read_to_string(&mut buf)?;

    if !buf.starts_with('<') {
        bail!(buf);
    }

    Ok(buf)
}

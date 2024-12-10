// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    io::{Error, ErrorKind, Result, Write},
    process::{Command, Stdio},
};

pub(crate) fn compile_blueprint(blueprint: &[u8]) -> Result<String> {
    let mut compiler = Command::new("blueprint-compiler")
        .args(["compile", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("blueprint-compiler not found"));
    let mut stdin = compiler.stdin.take().unwrap();
    stdin.write_all(blueprint)?;
    drop(stdin);

    let output = compiler
        .wait_with_output()
        .unwrap_or_else(|e| panic!("blueprint-compiler process failed {e}"));

    let buf = String::from_utf8(output.stdout).unwrap();

    if !buf.starts_with('<') {
        return Err(Error::new(ErrorKind::Other, buf));
    }

    Ok(buf)
}

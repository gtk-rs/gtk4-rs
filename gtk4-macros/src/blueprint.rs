// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    io::Write,
    process::{Command, Stdio},
};

pub(crate) fn compile_blueprint(blueprint: &[u8]) -> Result<String, String> {
    let mut compiler = Command::new("blueprint-compiler")
        .args(["compile", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("blueprint-compiler couldn't be spawned: {e}"))?;
    let mut stdin = compiler.stdin.take().unwrap();
    if let Err(e) = stdin
        .write_all(b"using Gtk 4.0;\n")
        .and_then(|_| stdin.write_all(blueprint))
    {
        let _ = compiler.wait();
        return Err(format!(
            "Couldn't send blueprint to blueprint-compiler: {e}"
        ));
    }
    drop(stdin);

    let output = compiler
        .wait_with_output()
        .map_err(|e| format!("blueprint-compiler process failed: {e}"))?;

    let buf = String::from_utf8(output.stdout).unwrap();
    if !buf.starts_with('<') {
        return Err(format!("blueprint-compiler failed: {buf}"));
    }

    Ok(buf)
}

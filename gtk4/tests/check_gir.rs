// Take a look at the license at the top of the repository in the LICENSE file.

#[test]
fn check_gir_file() {
    let res = gir_format_check::check_gir_file("Gir.toml");
    println!("{}", res.to_string());
    assert_eq!(res.nb_errors, 0);
}

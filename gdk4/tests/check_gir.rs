// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

extern crate gir_format_check;

#[test]
fn check_gir_file() {
    let res = gir_format_check::check_gir_file("Gir.toml");
    println!("{}", res.to_string());
    assert_eq!(res.nb_errors, 0);
}

cargo = find_program('cargo')
cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_options += ['--target-dir', meson.project_build_root() / 'target']

if not is_devel
    cargo_options += ['--release']
    rust_target = 'release'
else
    rust_target = 'debug'
endif

custom_target(
    'cargo-build',
    build_by_default: true,
    build_always_stale: true,
    output: meson.project_name(),
    console: true,
    install: true,
    install_dir: bindir,
    depends: resources,
    env: {
        'CARGO_HOME': meson.project_build_root() / 'cargo-home',
        'APP_ID': application_id,
        'GETTEXT_PACKAGE': meson.project_name(),
        'LOCALEDIR': localedir,
        'RESOURCES_FILE': pkgdatadir / 'resources.gresource',
    },
    command: [
        cargo,
        'build',
        cargo_options,
        '&&',
        'cp',
        meson.project_build_root() / 'target' / rust_target / meson.project_name(),
        '@OUTPUT@',
    ],
)

project(
  'org-gnome-test',
  'rust',
  version: '0.0.1',
  meson_version: '>= 0.50',
)

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.0.0')

cargo = find_program('cargo', required: true)
cargo_script = find_program('build-aux/cargo.sh')

bindir = get_option('prefix') / get_option('bindir')

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.source_root()
)

subdir('examples')

name = [
  'basics',
  'builder_pattern',
  'clipboard',
  'clock',
  'composite_template',
  'content_provider',
  'css',
  'custom_application',
  'custom_buildable',
  'custom_editable',
  'custom_layout_manager',
  'custom_orientable',
  'custom_paintable',
  'custom_widget',
  'dialog',
  'entry_completion',
  'entry_undo',
  'flow_box',
  'grid_packing',
  'gtk_builder',
  'list_view_apps_launcher',
  'list_view_example_apps_launcher',
  'scale_bin',
  'search_bar',
  'squares',
  'squeezer_bin',
  'text_viewer',
  'video_player',
  'virtual_methods',
]

message(bindir)

custom_target(
  'cargo-build',
  output: name,
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    cargo_script,
    meson.source_root(),
    meson.build_root() / 'target' / 'debug',
  ]
)

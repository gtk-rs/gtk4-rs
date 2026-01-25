subdir('icons')
subdir('resources')

desktop_conf = configuration_data()
desktop_conf.set('APP_ID', application_id)
configure_file(
    input: '@0@.desktop.in'.format(base_id),
    output: '@0@.desktop'.format(application_id),
    configuration: desktop_conf,
    install: true,
    install_dir: datadir / 'applications',
)

gschema_conf = configuration_data()
gschema_conf.set('APP_ID', application_id)
configure_file(
    input: '@0@.gschema.xml.in'.format(base_id),
    output: '@0@.gschema.xml'.format(application_id),
    configuration: gschema_conf,
    install: true,
    install_dir: datadir / 'glib-2.0' / 'schemas',
)


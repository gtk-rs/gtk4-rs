if is_devel
    icon_name = '@0@.Devel.svg'.format(base_id)
else
    icon_name = '@0@.svg'.format(base_id)
endif

install_data(
    icon_name,
    install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps',
    rename: '@0@.svg'.format(application_id),
)

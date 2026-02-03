# Set up the project
project('todo-10', 'rust', version: '0.1.0', meson_version: '>= 1.4')

# Import GNOME and i18n modules
gnome = import('gnome')
i18n = import('i18n')

# Set the base_id
base_id = 'org.gtk_rs.Todo10'

# Define variables depending on the current profile
is_devel = get_option('profile') == 'development'
if is_devel
    profile = 'Devel'
    application_id = '@0@.@1@'.format(base_id, profile)
else
    profile = ''
    application_id = base_id
endif

# Set a couple of useful variables
bindir = get_option('prefix') / get_option('bindir')
localedir = get_option('prefix') / get_option('localedir')
datadir = get_option('prefix') / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

# Enter these subdirectories and execute the meson.build files in them
subdir('data')
subdir('po')
subdir('src')

# Execute these tasks after installing
gnome.post_install(
    gtk_update_icon_cache: true,
    glib_compile_schemas: true,
    update_desktop_database: true,
)

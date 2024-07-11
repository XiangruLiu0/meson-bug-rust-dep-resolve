project(
  'meson-rust',
  'c',
  'rust',
  version: '0.1',
  meson_version: '>= 1.5.0',
  default_options: {},
)

subdir('lib')

executable(
  'demo',
  sources: ['main.c'],
  link_with: [my_lib],
  include_directories: my_inc,
)

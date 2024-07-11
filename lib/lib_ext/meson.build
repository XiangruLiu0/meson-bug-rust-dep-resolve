lib_ext_lib = static_library(
  'lib_ext',
  'src/lib.rs',
  rust_abi: 'c',
  dependencies: [
    dependency('figment-0.10-rs'),
    dependency(
      'rocket-0.5-rs',
    ),
    dependency('tokio-1-rs'),
  ],
)

lib_ext_dep = declare_dependency(
  link_with: lib_ext_lib,
)

# meson.override_dependency('lib_ext_dep', declare_dependency(
#   link_with : lib_ext_dep
# ))

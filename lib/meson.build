rust = import('rust')
# subproject('lib_ext')
# now create a lib_ext that is written in Rust
# my_lib_ext = static_library('my_lib_ext', 'src/lib.rs', rust_abi: 'c')
subdir('lib_ext')

my_lib = static_library('my_lib', ['lib.c'], dependencies: [lib_ext_dep])

my_inc = include_directories('.')

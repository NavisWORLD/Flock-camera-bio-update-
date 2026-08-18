set(FLOCK_SIGNAL_CARGO_TARGET_DIR "${CMAKE_BINARY_DIR}/cargo-target")

add_custom_target(rust_ffi ALL
    COMMAND ${CMAKE_COMMAND} -E env
            CARGO_TARGET_DIR=${FLOCK_SIGNAL_CARGO_TARGET_DIR}
            cargo build --manifest-path ${CMAKE_SOURCE_DIR}/Cargo.toml --release -p flock-signal-ffi
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building Rust FFI library"
)

add_library(flock_signal_ffi SHARED IMPORTED GLOBAL)

if(WIN32)
    set_target_properties(flock_signal_ffi PROPERTIES
        IMPORTED_LOCATION "${FLOCK_SIGNAL_CARGO_TARGET_DIR}/release/flock_signal_ffi.dll"
        IMPORTED_IMPLIB "${FLOCK_SIGNAL_CARGO_TARGET_DIR}/release/flock_signal_ffi.dll.lib"
    )
elseif(APPLE)
    set_target_properties(flock_signal_ffi PROPERTIES
        IMPORTED_LOCATION "${FLOCK_SIGNAL_CARGO_TARGET_DIR}/release/libflock_signal_ffi.dylib"
    )
else()
    set_target_properties(flock_signal_ffi PROPERTIES
        IMPORTED_LOCATION "${FLOCK_SIGNAL_CARGO_TARGET_DIR}/release/libflock_signal_ffi.so"
    )
endif()

add_executable(flock_signal_cpp_smoke tests/smoke.cpp)
add_dependencies(flock_signal_cpp_smoke rust_ffi)
target_compile_features(flock_signal_cpp_smoke PRIVATE cxx_std_20)
target_include_directories(flock_signal_cpp_smoke PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(flock_signal_cpp_smoke PRIVATE flock_signal_ffi)

if(UNIX AND NOT APPLE)
    set_target_properties(flock_signal_cpp_smoke PROPERTIES
        BUILD_RPATH "${FLOCK_SIGNAL_CARGO_TARGET_DIR}/release"
    )
elseif(APPLE)
    set_target_properties(flock_signal_cpp_smoke PROPERTIES
        BUILD_RPATH "${FLOCK_SIGNAL_CARGO_TARGET_DIR}/release"
    )
endif()

add_test(NAME cpp_rust_ffi_smoke COMMAND flock_signal_cpp_smoke)

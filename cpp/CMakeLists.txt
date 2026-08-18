cmake_minimum_required(VERSION 3.20)

find_program(CARGO_EXECUTABLE cargo REQUIRED)

set(RUST_TARGET_DIR "${CMAKE_SOURCE_DIR}/target/release")
if(WIN32)
    set(RUST_SHARED_LIB "${RUST_TARGET_DIR}/signal_ffi.dll")
    set(RUST_IMPORT_LIB "${RUST_TARGET_DIR}/signal_ffi.dll.lib")
elseif(APPLE)
    set(RUST_SHARED_LIB "${RUST_TARGET_DIR}/libsignal_ffi.dylib")
else()
    set(RUST_SHARED_LIB "${RUST_TARGET_DIR}/libsignal_ffi.so")
endif()

add_custom_target(signal_ffi_rust ALL
    COMMAND "${CARGO_EXECUTABLE}" build -p signal-ffi --release
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
    BYPRODUCTS "${RUST_SHARED_LIB}"
    COMMENT "Building Rust signal-ffi library"
)

add_library(signal_ffi_native SHARED IMPORTED GLOBAL)
set_target_properties(signal_ffi_native PROPERTIES IMPORTED_LOCATION "${RUST_SHARED_LIB}")
if(WIN32)
    set_target_properties(signal_ffi_native PROPERTIES IMPORTED_IMPLIB "${RUST_IMPORT_LIB}")
endif()
add_dependencies(signal_ffi_native signal_ffi_rust)

add_library(flock_signal_cpp STATIC src/flock_signal.cpp)
target_compile_features(flock_signal_cpp PUBLIC cxx_std_20)
target_include_directories(flock_signal_cpp PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(flock_signal_cpp PUBLIC signal_ffi_native)
add_dependencies(flock_signal_cpp signal_ffi_rust)

add_executable(flock_signal_cpp_smoke tests/smoke.cpp)
target_compile_features(flock_signal_cpp_smoke PRIVATE cxx_std_20)
target_link_libraries(flock_signal_cpp_smoke PRIVATE flock_signal_cpp)
if(APPLE OR UNIX)
    set_target_properties(flock_signal_cpp_smoke PROPERTIES BUILD_RPATH "${RUST_TARGET_DIR}")
endif()

add_test(NAME flock_signal_cpp_smoke COMMAND flock_signal_cpp_smoke)

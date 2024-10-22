# Rust FFI Bindings for Fusion Library

I am experimenting with IMU sensor fusion algorithms on my STM32F3DISCOVERY, and I wanted to try the Madgwick/Fusion algorithm (https://github.com/xioTechnologies/Fusion) as it fits my use case. However it is written in C, and I have been enjoying using Rust for my embedded projects, so I decided to generate Rust FFI bindings for it.

These bindings were generated by passing the header files which expose the Fusion API to the bindgen tool.

Since I want to run this on my STM32F3DISCOVERY, the build.rs can cross compile to the thumbv7em-none-eabihf target. However it is also possible to build and run the examples on the host architecture too (macOS with apple silicon in my case).

# Modification of the Fusion Repository

I kept modification of the Fusion repository to a minimum, however in the CMakeLists.txt at the project root, I removed the following lines to exclude the examples and Python implementation from being built, as that is not necessary.

```
add_subdirectory("Examples/Advanced")
add_subdirectory("Examples/Simple")
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_subdirectory("Python/Python-C-API") # do not include when run by CI
endif()
```

# Building and Running

I used nightly Rust to build this, as the 2024 Rust prelude is included. You can use nightly rust by running ```rustup default nightly```.

Since I am cross-compiling to the thumbv7em-none-eabihf target, I had to download the ARM GNU toolchain for the arm-none-eabi target here: https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads. The ARM_NONE_EABI_INCLUDE_DIR constant in build.rs should point to the arm-none-eabi/include directory so it can get the necessary header files.

The FUSION_DIR constant should point to the clone of the Fusion repo.

After this, the normal cargo workflow can be used to build this crate.
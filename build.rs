use cmake::Config;
use std::{env, path::PathBuf, process::Command};
extern crate cmake;

const FUSION_DIR: &str = "/Users/daniel/Documents/repos/Fusion";
const ARM_NONE_EABI_INCLUDE_DIR: &str = "/System/Volumes/Data/Applications/ArmGNUToolchain/13.3.rel1/arm-none-eabi//arm-none-eabi/include";

fn main() {
    /* Represents whether this cargo run is cross compiling to cortex m4 or not */
    let cross_compile = env::var("TARGET").expect("Could not get compilation target from $TARGET.")
        == "thumbv7em-none-eabihf";

    /* Get path to the input header file using this crate root */
    let crate_root_string = env::var("CARGO_MANIFEST_DIR")
        .expect("Could not get crate root from $CARGO_MANIFEST_DIR.\n");

    let input_string = PathBuf::from(&crate_root_string)
        .join("wrapper.h")
        .to_string_lossy()
        .into_owned();

    let output_dir_string =
        env::var("OUT_DIR").expect("Could not get output directory from $OUT_DIR.");
    let output_dir_path = PathBuf::from(&output_dir_string);

    /* Build libFusion for the target of this cargo config */
    let mut libfusion = Config::new(FUSION_DIR);

    /* Specify cmake toolchain file if cross compiling */
    if cross_compile {
        libfusion.define(
            "CMAKE_TOOLCHAIN_FILE",
            PathBuf::from(crate_root_string).join("toolchain-arm-none-eabi.cmake"),
        );
    }

    let libfusion = libfusion
        .always_configure(true)
        .build_target("all")
        .very_verbose(true)
        .env("CMAKE_BUILD_PARALLEL_LEVEL", "1")
        .build();

    /* Tell cargo to look for shared libraries in the specified directories */
    println!(
        "cargo:rustc-link-search={}/build/Fusion",
        libfusion.display()
    );
    println!("cargo:rustc-link-search={}", &output_dir_string);

    /* Create rust FFI bindings for the libFusion API */
    let mut bindings = bindgen::Builder::default();

    /* If cross compiling, point to directory containing C library header files */
    if cross_compile {
        bindings = bindings.clang_arg(format!("-I{}", ARM_NONE_EABI_INCLUDE_DIR));
    }

    let bindings = bindings
        .header(&input_string) /* The input header we would like to generate bindings for. */
        .ctypes_prefix("cty") /* Use c formatted types */
        .use_core() /* Do not use std library so this can be used in no_std environments */
        .derive_default(true) /* Generate structs which implement the Derive trait */
        .wrap_static_fns_path(
            output_dir_path
                .join("extern")
                .to_string_lossy()
                .into_owned(),
        ) /* Output wrappers for static functions to <out_dir>/extern.c */
        .wrap_static_fns(true) /* Generate wrappers for static functions */
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) /* Tell cargo to invalidate the built crate
        whenever any of the included header files changed. */
        .generate() /* Finish the builder and generate the bindings. */
        .expect("Unable to generate bindings."); /* Unwrap the Result and panic on failure. */

    /* Compile the generated wrappers into an object file. */
    let obj_path = output_dir_path.join("extern.o");

    let compiler = if cross_compile {
        "arm-none-eabi-gcc"
    } else {
        "gcc"
    };

    let wrapper_output = std::process::Command::new(compiler)
        .arg("-O")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(output_dir_path.join("extern.c"))
        .arg("-include")
        .arg(&input_string)
        .output()
        .unwrap();

    if !wrapper_output.status.success() {
        panic!(
            "Could not compile object file:\n{}",
            String::from_utf8_lossy(&wrapper_output.stderr)
        );
    }

    /* Turn the object file into a static library */
    let lib_output = Command::new("ar")
        .arg("rcs")
        .arg(output_dir_path.join("libextern.a"))
        .arg(&obj_path)
        .output()
        .unwrap();
    if !lib_output.status.success() {
        panic!(
            "Could not emit library file:\n{}",
            String::from_utf8_lossy(&lib_output.stderr)
        );
    }

    /* Tell rust to link to the required static libraries*/
    println!("cargo:rustc-link-lib=static=Fusion");
    println!("cargo:rustc-link-lib=static=extern");

    /* Write the bindings to the $OUT_DIR/bindings.rs file. */
    let out_path = PathBuf::from(output_dir_string);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

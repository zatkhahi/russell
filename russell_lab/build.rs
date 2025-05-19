#[cfg(feature = "intel_mkl")]
const MKL_VERSION: &str = "2023.2.0";

#[cfg(all(feature = "intel_mkl", target_os = "linux"))]
const DEFAULT_MKL_PATH: &str = "/opt/intel/oneapi/mkl";
#[cfg(all(feature = "intel_mkl", target_os = "windows"))]
const DEFAULT_MKL_PATH: &str = "";

#[cfg(all(feature = "intel_mkl", target_os = "linux"))]
const DEFAULT_COMPILER_PATH: &str = "/opt/intel/oneapi/compiler";
#[cfg(all(feature = "intel_mkl", target_os = "windows"))]
const DEFAULT_COMPILER_PATH: &str = "C:\\Program Files (x86)\\IntelSWTools\\compilers_and_libraries_2020.4.311";

#[cfg(feature = "intel_mkl")]
fn compile_blas() {
    // Get paths from environment variables or use defaults
    let mkl_path = std::env::var("MKL_DIR")
        .unwrap_or_else(|_| DEFAULT_MKL_PATH.to_string());
    let compiler_path = std::env::var("INTEL_COMPILER_PATH")
        .unwrap_or_else(|_| mkl_path.clone() + "/../compiler");

    // Build the C code
    cc::Build::new()
        .file("c_code/interface_blas.c")
        .include(format!("{}/include", mkl_path))
        .define("USE_INTEL_MKL", None)
        .compile("c_code_interface_blas");

    // Set up linking
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=native={}/{}/lib/intel64",mkl_path, MKL_VERSION);
        println!("cargo:rustc-link-search=native={}/{}/linux/compiler/lib/intel64_lin",compiler_path, MKL_VERSION);
        println!("cargo:rustc-link-lib=mkl_intel_lp64");
        println!("cargo:rustc-link-lib=mkl_intel_thread");
        println!("cargo:rustc-link-lib=mkl_core");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=iomp5");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=native={}\\lib\\intel64", mkl_path);
        println!("cargo:rustc-link-search=native={}\\lib\\intel64_win",compiler_path);
        println!("cargo:rustc-link-lib=mkl_intel_lp64");
        println!("cargo:rustc-link-lib=mkl_intel_thread");
        println!("cargo:rustc-link-lib=mkl_core");
        println!("cargo:rustc-link-lib=libiomp5md");
    }
}

// OpenBLAS
#[cfg(not(feature = "intel_mkl"))]
fn compile_blas() {
    cc::Build::new()
        .file("c_code/interface_blas.c")
        .includes(&[
            "/usr/include/openblas",              // Arch
            "/opt/homebrew/opt/lapack/include",   // macOS
            "/opt/homebrew/opt/openblas/include", // macOS
            "/usr/local/opt/lapack/include",      // macOS
            "/usr/local/opt/openblas/include",    // macOS
        ])
        .compile("c_code_interface_blas");
    for d in &[
        "/opt/homebrew/opt/lapack/lib",   // macOS
        "/opt/homebrew/opt/openblas/lib", // macOS
        "/usr/local/opt/lapack/lib",      // macOS
        "/usr/local/opt/openblas/lib",    // macOS
    ] {
        println!("cargo:rustc-link-search=native={}", *d);
    }
    println!("cargo:rustc-link-lib=dylib=openblas");
    println!("cargo:rustc-link-lib=dylib=lapack");
}

fn main() {
    compile_blas();
}

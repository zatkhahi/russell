#[cfg(all(target_os = "windows"))]
const DEFAULT_KLU_INCLUDE_PATH: &str = "C:\\githubs\\SuiteSparse\\install\\include\\suitesparse";
const DEFAULT_KLU__LIB_PATH: &str = "C:\\githubs\\SuiteSparse\\install\\lib";
const STATIC_LIB: bool = true;

fn main() {
    // SuiteSparse ----------------------------------------------------------
    let libs = vec!["suitesparseconfig", "amd", "cholmod", "ccolamd", "camd", "btf", "colamd", "klu", "umfpack"];

    let klu_include_path = std::env::var("SUITESPARSE_INCLUDE_DIR")
        .unwrap_or_else(|_| DEFAULT_KLU_INCLUDE_PATH.to_string());
    println!("cargo:warning=klu include path is: {}", klu_include_path);
    let klu_lib_path = std::env::var("SUITESPARSE_LIBRARY_DIR")
        .unwrap_or_else(|_| DEFAULT_KLU__LIB_PATH.to_string());
    #[cfg(not(feature = "local_suitesparse"))]
    let lib_dirs = vec![
        "/usr/lib/x86_64-linux-gnu", // Debian
        "/usr/lib",                  // Arch
        "/usr/lib64",                // Rocky
        "/opt/homebrew/lib",         // macOS
    ];

    #[cfg(not(feature = "local_suitesparse"))]
    let inc_dirs = vec![
        "/usr/include/suitesparse", // Linux
    ];

    #[cfg(target_os = "macos")]
    let inc_dirs = vec![
        "/opt/homebrew/include/suitesparse", // macOS
        "/usr/local/include/suitesparse",    // macOS
    ];

    #[cfg(all(target_os = "windows"))]
    let inc_dirs = vec![klu_include_path];
    #[cfg(all(target_os = "windows"))]
    let lib_dirs = vec![klu_lib_path];

    #[cfg(feature = "local_suitesparse")]
    let lib_dirs = vec!["/usr/local/lib/suitesparse"];

    #[cfg(feature = "local_suitesparse")]
    let inc_dirs = vec!["/usr/local/include/suitesparse"];

    cc::Build::new()
        .file("c_code/interface_complex_klu.c")
        .file("c_code/interface_complex_umfpack.c")
        .file("c_code/interface_klu.c")
        .file("c_code/interface_umfpack.c")
        .includes(&inc_dirs)
        .compile("c_code_suitesparse");
    for d in &lib_dirs {
        println!("cargo:rustc-link-search=native={}", *d);
    }
    for l in &libs {
        if STATIC_LIB {
            println!("cargo:rustc-link-lib=static={}", *l);
        } else {
            println!("cargo:rustc-link-lib=dylib={}", *l);
        }
    }

    // MUMPS ----------------------------------------------------------------

    #[cfg(feature = "with_mumps")]
    {
        cc::Build::new()
            .file("c_code/interface_complex_mumps.c")
            .file("c_code/interface_mumps.c")
            .include("/usr/local/include/mumps")
            .compile("c_code_mumps");
        println!("cargo:rustc-link-search=native=/usr/local/lib/mumps");
        println!("cargo:rustc-link-lib=dylib=dmumps_cpmech");
        println!("cargo:rustc-link-lib=dylib=zmumps_cpmech");
    }
}

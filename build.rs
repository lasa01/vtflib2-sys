use std::env;

fn main() {
    // no need to build/find anything for docs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-env-changed=VTFLIB_STATIC");
    println!("cargo:rerun-if-env-changed=VTFLIB_PATH");
    println!("cargo:rerun-if-env-changed=VTFLIB13_PATH");

    if let Ok(path) = env::var("VTFLIB13_PATH") {
        println!("cargo:rustc-link-search=native={}", path);
        println!("cargo:rustc-link-lib=static=VTFLib13");
        return;
    }

    if let Ok(path) = env::var("VTFLIB_PATH") {
        println!("cargo:rustc-link-search=native={}", path);
        println!("cargo:rustc-link-lib=static=VTFLib");
        return;
    }

    let statik = cfg!(feature = "static") || env::var("VTFLIB_STATIC").is_ok();

    let mut pkg = pkg_config::Config::new();
    pkg.statik(statik);
    pkg.atleast_version("1.3.2");
    if pkg
        .probe("VTFLib13")
        .or_else(|_| pkg.probe("VTFLib"))
        .is_err()
    {
        if statik {
            build_static();
        } else {
            eprintln!("error: could not locate dynamic library VTFLib");
            eprintln!("help: enable the `static` feature or set the env var `VTFLIB_STATIC` to build a static library automatically");
            panic!("could not locate dynamic library VTFLib");
        }
    }
}

fn build_static() {
    let mut build_config = cmake::Config::new("vendor");
    build_config.define("BUILD_SHARED_LIBS", "OFF");
    build_config.pic(true);
    // todo: a feature for this
    build_config.define("USE_LIBTXC_DXTN", "OFF");
    build_config.define("USE_NVDXT", "OFF");

    let dest = build_config.build();

    let lib_dest = dest.join("lib");
    let lib64_dest = dest.join("lib64");

    println!("cargo:rustc-link-search=native={}", lib_dest.display());
    println!("cargo:rustc-link-search=native={}", lib64_dest.display());
    println!("cargo:rustc-link-lib=static=VTFLib13");

    if cfg!(unix) {
        let library_name = if cfg!(target_os = "macos") {
            "c++"
        } else {
            "stdc++"
        };

        println!("cargo:rustc-link-lib=dylib={}", library_name);
    }
}

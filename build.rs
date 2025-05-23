fn compile() -> String {
    let dst = cmake::Config::new("draco")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("DRACO_POINT_CLOUD_COMPRESSION", "ON")
        .define("DRACO_MESH_COMPRESSION", "ON")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        // .define("CMAKE_CXX_FLAGS", "-D_GLIBCXX_USE_CXX11_ABI=1")
        .cxxflag("-fPIC")
        // silent all build noise from the upstream draco library
        .cxxflag("-w")
        .cxxflag("-Wno-everything")
        .build();

    dst.display().to_string()
}

fn generate_bindings(out_dir: String) -> miette::Result<()> {
    let includes = vec![
        "src".to_string(),
        "draco/src".to_string(),
        format!("{}/include", out_dir),
    ];

    let mut b = autocxx_build::Builder::new("src/bindgen.rs", &includes)
        .extra_clang_args(&[
            "-std=c++14",
            "-w", // silences all warnings during clang parsing
            "-Wno-everything",
        ])
        .build()?;

    b.opt_level(3)
        .cpp(true)
        .std("c++14")
        // .flag("-ldraco")
        // .flag("-Wl,-l:libdraco.a")
        // .flag(format!("-L{}", out_dir))
        // .flag(format!("-L{}/build", out_dir))
        .compile("draco-rs");

    println!("cargo:rerun-if-changed=src/bindgen.rs");
    println!("cargo:rerun-if-changed=src/extra.h");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=draco-rs");

    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=draco");

    Ok(())
}

fn main() -> miette::Result<()> {
    let out_dir = compile();

    generate_bindings(out_dir)?;
    Ok(())
}

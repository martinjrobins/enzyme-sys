use std::{env, fs, path::PathBuf};

fn compile_enzyme(llvm_dir: String) -> String {
    let dst = cmake::Config::new("Enzyme/enzyme")
        .define("LLVM_DIR", llvm_dir)
        .build();
    let dst_disp = dst.display();
    let lib_loc = format!("{}/lib", dst_disp);
    lib_loc
}

fn _generate_bindings(inc_dir: String) -> Result<(), String> {
    let header_path = PathBuf::from(inc_dir.as_str()).join("Enzyme/CApi.h");
    dbg!(&header_path);

    // tell cargo to re-run the builder if the header has changed
    println!("cargo:rerun-if-changed={}", header_path.display());
    let content: String = fs::read_to_string(header_path.clone()).unwrap();

    let bindings = bindgen::Builder::default()
        .header_contents("CApi.hpp", &content) // read it as .hpp so bindgen can ignore the class successfully
        .clang_args([
            format!("-I{}", inc_dir.as_str())
        ])
        //.blacklist_item("CustomFunctionForward")
        //.blacklist_item("DiffeGradientUtils")
        .allowlist_type("CConcreteType")
        .rustified_enum("CConcreteType")
        .allowlist_type("CDerivativeMode")
        .rustified_enum("CDerivativeMode")
        .allowlist_type("CDIFFE_TYPE")
        .rustified_enum("CDIFFE_TYPE")
        .allowlist_type("LLVMContextRef")
        .allowlist_type("CTypeTreeRef")
        .allowlist_type("EnzymeTypeAnalysisRef")
        .allowlist_function("EnzymeNewTypeTree")
        .allowlist_function("EnzymeNewTypeTreeCT")
        .allowlist_function("EnzymeFreeTypeTree")
        .allowlist_function("EnzymeMergeTypeTree")
        .allowlist_function("EnzymeTypeTreeOnlyEq")
        .allowlist_function("EnzymeMergeTypeTree")
        .allowlist_function("EnzymeTypeTreeShiftIndiciesEq")
        .allowlist_function("EnzymeTypeTreeToString")
        .allowlist_function("EnzymeTypeTreeToStringFree")
        // Next two are for debugging / printning type information
        .allowlist_function("EnzymeSetCLBool")
        .allowlist_function("EnzymeSetCLInteger")
        .allowlist_function("CreateTypeAnalysis")
        .allowlist_function("ClearTypeAnalysis")
        .allowlist_function("FreeTypeAnalysis")
        .allowlist_function("CreateEnzymeLogic")
        .allowlist_function("ClearEnzymeLogic")
        .allowlist_function("FreeEnzymeLogic")
        .allowlist_type("LLVMOpaqueModule")
        .allowlist_function("EnzymeCreatePrimalAndGradient")
        .allowlist_function("EnzymeCreateAugmentedPrimal")
        //.allowlist_function("LLVMModuleCreateWithName")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate();

    let bindings = match bindings {
        Ok(v) => v,
        Err(_) => {
            return Err(format!(
                "Unable to generate bindings from {}.",
                header_path.display()
            ))
        }
    };

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()); 
    let out_file = out_path.join("bindings.rs");
    if out_file.exists() {
        fs::remove_file(out_file.clone()).unwrap();
    }

    let result = bindings.write_to_file(out_file.clone());

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "Couldn't write bindings to {}.",
            out_file.display()
        )),
    }
}

fn main() {
    println!("all env vars: {:?}", env::vars().collect::<Vec<(String, String)>>());
    // get env vars matching DEP_LLVM_*_LIBDIR regex    
    let llvm_dirs: Vec<_> = env::vars().filter(|(k, _)| k.starts_with("DEP_LLVM_") && k.ends_with("_LIBDIR")).collect();
    // take first one
    let llvm_dir = llvm_dirs.first().expect("DEP_LLVM_*_LIBDIR not set").1.clone();

    dbg!("llvm_dir", &llvm_dir);
    
    // compile enzyme
    let libdir= compile_enzyme(llvm_dir);
    println!("cargo:libdir={}", libdir); // DEP_ENZYME_SYS_LIBDIR
    
    // Generate bindings (remove for now)
    //let inc_dir = format!("{}/Enzyme/enzyme", env::var("CARGO_MANIFEST_DIR").unwrap());
    //generate_bindings(inc_dir).unwrap();
}
#[cfg(target_family = "unix")]
fn main() {
    
}

#[cfg(target_family = "windows")]
fn main() {
    // dependency, supported OS and privilege are set by default
    // see documentation for [embed_manifest::new_manifest]
    embed_manifest::embed_manifest(embed_manifest::new_manifest("SealUpdater"))
        .expect("failed to create manifest");
    println!("cargo:rerun-if-changed=build.rs");

    static_vcruntime::metabuild();
}

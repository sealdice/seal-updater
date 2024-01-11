fn main() {
    if cfg!(windows) {
        // By default, the manifest:
        //
        //   - is compatible with from Windows 7 to Windows 11
        //   - uses "AsInvoker" execution level
        //   - adds a dependency on Common Controls 6.0.0.0
        //
        embed_manifest::embed_manifest(embed_manifest::new_manifest("SealUpdater"))
            .expect("failed to create manifest");
        println!("cargo:rerun-if-changed=build.rs");

        static_vcruntime::metabuild();
    }
}

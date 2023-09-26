fn main() {
    #[cfg(target_family = "windows")]
    static_vcruntime::metabuild();
}

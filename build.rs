fn main() {
    let flux_path = std::env::var("ZIRVFLUX_DIR")
        .unwrap_or_else(|_| "../zirvflux".to_string());
    println!("cargo:rustc-link-search={}", flux_path);
    println!("cargo:rustc-link-lib=static=zirvflux");
}

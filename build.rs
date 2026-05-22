fn main() {
    println!("cargo:rustc-link-search=/home/gauthamnair2005/projects/zirvium/zirvflux");
    println!("cargo:rustc-link-lib=static=zirvflux");
}

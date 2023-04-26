fn main() {
    println!("cargo:rustc-link-search=target/debug");
    println!("cargo:rustc-link-lib=my_get_int");
}

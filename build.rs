fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        let _ = embed_resource::compile("res/icon.rc", embed_resource::NONE);
    }
    if target_os == "android" {
        println!("cargo:rustc-link-lib=c++");
    }
}

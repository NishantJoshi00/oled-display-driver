use std::env;

fn main() {
    match env::var("PROFILE") {
        Ok(x) if x.contains("release") => {
            println!("cargo:rustc-cfg=feature=\"board\"");
        }
        _ => {
            println!("cargo:rustc-cfg-feature=\"sim\"");
        }
    }
}

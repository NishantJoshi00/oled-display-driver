use std::env;


fn main() {
    let target = env::var("TARGET").unwrap();
    if target == String::from("arm-unknown-linux-gnueabihf") {

    } else {

    }

    match env::var("TARGET") {
        Ok(x) if x.contains("arm-unknown-linux-gnueabihf") => {
            println!("cargo:rustc-cfg=feature=\"board\"");
        },
        _ => {
            println!("cargo:rustc-cfg-feature=\"sim\"");
        }
    }
}

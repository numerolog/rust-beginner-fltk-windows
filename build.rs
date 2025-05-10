fn main() {
    // hide console in windows
    if std::env::var("TARGET").unwrap_or("".parse().unwrap()).contains("windows")
    {
        println!("cargo:rustc-link-arg=-mwindows");
    }
}
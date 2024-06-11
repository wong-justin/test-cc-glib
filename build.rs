fn main() {
    let source_c_files = std::fs::read_dir("vendor/glib")
        .unwrap()
        .filter_map(|entry| {
            let path = &entry.expect("couldn't unwrap entry inside vendor/").path();
            let pathstr = path.to_str().expect("couldn't convert vendored .c path to string");
            match path.is_file() 
                & path.ends_with(".c") 
            {
                true => Some(String::from(pathstr)),
                false => None
            }
        });

    // this step currently fails: `ar` cannot find libglib.a
    cc::Build::new()
        .files(source_c_files)
        .compile("glib");
}

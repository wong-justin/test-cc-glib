fn main() {
    let paths = &mut Vec::<std::path::PathBuf>::new();
    let source_c_files = read_dir_recursive("vendor/glib", paths);

    std::fs::write("log.txt", format!("{:?}", &source_c_files));

    source_c_files
        .iter()
        .filter_map(|path| {
            let pathstr = path.to_str().expect("couldn't convert vendored .c path to string");
            match (
                path.ends_with(".c") &
                !pathstr.contains("tests/") &
                !pathstr.contains("tools/") &
                !pathstr.contains("fuzzing/")
            ) {
                true => Some(String::from(pathstr)),
                false => None
            }
        });

    cc::Build::new()
        .files(source_c_files)
        .compile("glib");
}

fn read_dir_recursive<P>(dir: P, accumulated_paths: &mut Vec<std::path::PathBuf>) -> &mut Vec<std::path::PathBuf>
    where P: AsRef<std::path::Path>
{
    let entries = std::fs::read_dir(dir).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        match &path.is_dir() {
            true => {
                let dup = &mut accumulated_paths.clone();
                let child_paths = read_dir_recursive(path, dup);
                accumulated_paths.append(child_paths);
            },
            false => accumulated_paths.push(path),
        };
    }
    return accumulated_paths;
}

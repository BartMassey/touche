fn main() {
    let input = std::env::args_os().skip(1).map(std::path::PathBuf::from);

    for path in input {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("{}: {e}", parent.display());
                std::process::exit(1);
            });
        }
        if path.file_name().is_some() {
            std::fs::OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(&path)
                .unwrap_or_else(|e| {
                    eprintln!("{}: {e}", path.display());
                    std::process::exit(1);
                });
        }
    }
}

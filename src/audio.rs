pub fn ring(path: &str) {
    let r = play::play(&path);
    if let Err(e) = r {
        eprintln!("cannot play audio file \"{}\": {}", path, e);
    }
}

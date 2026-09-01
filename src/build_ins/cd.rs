pub fn cd(path: &str) -> Result<(), std::io::Error> {
    std::env::set_current_dir(path)
}

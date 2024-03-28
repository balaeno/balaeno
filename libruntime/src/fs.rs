// get absolute path of a path
pub fn abs_path(path: &str) -> Option<String> {
    let exp_path = shellexpand::full(path).ok()?;
    let can_path = std::fs::canonicalize(exp_path.as_ref()).ok()?;
    can_path.into_os_string().into_string().ok()
}

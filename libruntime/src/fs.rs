use std::{fs::create_dir_all, path::Path};

use anyhow::{anyhow, Result};
use nix::{
    mount::umount2,
    mount::MntFlags,
    unistd::{chdir, pivot_root},
};

// get absolute path of a path
pub fn abs_path(path: &str) -> Option<String> {
    let exp_path = shellexpand::full(path).ok()?;
    let can_path = std::fs::canonicalize(exp_path.as_ref()).ok()?;
    can_path.into_os_string().into_string().ok()
}

pub fn pivot_rootfs(root: &Path) -> Result<()> {
    match chdir(root) {
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("failed to chdir to rootfs: {:?}", e));
        }
    }

    static OLD_ROOT: &str = "old_root";

    match create_dir_all(root.join(OLD_ROOT)) {
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("failed to create old_root path: {:?}", e));
        }
    }

    match pivot_root(root.as_os_str(), root.join(OLD_ROOT).as_os_str()) {
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("failed to pivot_root: {:?}", e));
        }
    }

    match umount2(format!("./{}", OLD_ROOT).as_str(), MntFlags::MNT_DETACH) {
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("failed to unmount old_root: {:?}", e));
        }
    }

    Ok(())
}

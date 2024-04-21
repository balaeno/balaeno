use std::{fs::create_dir_all, os::unix::fs::symlink, path::Path};

use anyhow::{anyhow, Result};
use nix::{
    mount::{mount, umount2, MntFlags, MsFlags},
    unistd::{chdir, pivot_root},
};

// get absolute path of a path
pub fn abs_path(path: &str) -> Option<String> {
    let exp_path = shellexpand::full(path).ok()?;
    let can_path = std::fs::canonicalize(exp_path.as_ref()).ok()?;
    can_path.into_os_string().into_string().ok()
}

pub fn mount_rootfs(root: &Path) -> Result<()> {
    match mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    ) {
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("failed to mount rootfs: {:?}", e));
        }
    }

    match mount::<Path, Path, str, str>(
        Some(&root),
        &root,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    ) {
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("failed to bind mount rootfs: {:?}", e));
        }
    }

    Ok(())
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

// https://github.com/opencontainers/runtime-spec/blob/main/runtime-linux.md
pub fn dev_symlinks(root: &Path) -> Result<()> {
    let links = [
        ("/proc/self/fd", "/dev/fd"),
        ("/proc/self/fd/0", "/dev/stdin"),
        ("/proc/self/fd/1", "/dev/stdout"),
        ("/proc/self/fd/2", "/dev/stderr"),
    ];

    links
        .iter()
        .for_each(|(src, dst)| match rootfs_symlink(root, src, dst) {
            Ok(_) => {}
            Err(e) => {
                panic!("failed to create symlink: {:?}", e);
            }
        });

    Ok(())
}

pub fn rootfs_symlink(root: &Path, src: &str, dst: &str) -> Result<()> {
    match symlink(src, root.join(dst)) {
        Ok(_) => {}
        Err(e) => {
            return Err(anyhow!("failed to create symlink: {:?}", e));
        }
    }
    Ok(())
}

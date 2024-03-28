use crate::error::{ErrorType, RuntimeError};
use crate::namespace::to_flags;
use anyhow::{anyhow, Error};
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::{fcntl::open, sched::clone, sched::CloneFlags, unistd::Pid};
use oci_spec::runtime::{LinuxNamespace, Spec};

pub fn clone_child(
    child_fun: impl FnMut() -> isize,
    namespaces: &Vec<LinuxNamespace>,
) -> Result<Pid, Error> {
    const STACK_SIZE: usize = 4 * 1024 * 1024; // 4 MB
    let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];

    let spec_namespaces = namespaces.iter().map(to_flags).reduce(|a, b| a | b);

    let clone_flags = match spec_namespaces {
        Some(flags) => flags,
        None => CloneFlags::empty(),
    };

    let child = unsafe { clone(Box::new(child_fun), stack, clone_flags, None) };

    child.map_err(|err| {
        anyhow!(
            "{}",
            RuntimeError {
                message: err.to_string(),
                error_type: ErrorType::Container
            }
        )
    })
}

pub fn fork_container(spec: &Spec, namespaces: &Vec<LinuxNamespace>) -> Result<Pid, Error> {
    let pid = clone_child(|| todo!(), namespaces)?;
    let Some(linux) = &spec.linux() else {
        return Err(anyhow!(
            "{}",
            RuntimeError {
                message: "linux is not defined".to_string(),
                error_type: ErrorType::Container
            }
        ));
    };

    let Some(namespaces) = &linux.namespaces() else {
        return Err(anyhow!(
            "{}",
            RuntimeError {
                message: "namespaces is not defined".to_string(),
                error_type: ErrorType::Container
            }
        ));
    };

    for ns in namespaces {
        let Some(path) = &ns.path() else {
            continue;
        };
        let _fd = match open(path.as_os_str(), OFlag::empty(), Mode::empty()) {
            Ok(fd) => fd,
            Err(err) => {
                return Err(anyhow!(
                    "{}",
                    RuntimeError {
                        message: err.to_string(),
                        error_type: ErrorType::Container
                    }
                ));
            }
        };
    }

    Ok(pid)
}

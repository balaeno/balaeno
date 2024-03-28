use crate::error::{ErrorType, RuntimeError};
use crate::namespace::{self, to_flags};
use anyhow::{anyhow, Error};
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::{sched::clone, sched::CloneFlags,
    unistd::Pid,
    fcntl::open};
use oci_spec::runtime::{LinuxNamespace, Spec};

pub fn clone_child(
    child_fun: impl FnMut() -> isize,
    namespaces: &Vec<LinuxNamespace>,
) -> Result<Pid, Error> {
    const STACK_SIZE: usize = 4 * 1024 * 1024; // 4 MB
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let spec_namespaces = namespaces
        .into_iter()
        .map(|ns| to_flags(ns))
        .reduce(|a, b| a | b);

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
    let pid = clone_child(||{todo!()} , namespaces)?;
    if let Some(linux) = &spec.linux(){
        if let Some(namespaces) = &linux.namespaces(){
        for ns in namespaces{
            if let Some(path) = &ns.path(){
                let fd = match open(path.as_os_str(), OFlag::empty(), Mode::empty()) {
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
        }
    }
    }

    Ok(pid)
}
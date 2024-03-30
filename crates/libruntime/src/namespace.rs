use nix::sched::CloneFlags;
use oci_spec::runtime::{LinuxNamespace, LinuxNamespaceType};

// todo : support Windows
pub(super) fn to_flags(namespace: &LinuxNamespace) -> CloneFlags {
    match namespace.typ() {
        LinuxNamespaceType::Pid => CloneFlags::CLONE_NEWPID,
        LinuxNamespaceType::Network => CloneFlags::CLONE_NEWNET,
        LinuxNamespaceType::Mount => CloneFlags::CLONE_NEWNS,
        LinuxNamespaceType::Ipc => CloneFlags::CLONE_NEWIPC,
        LinuxNamespaceType::Uts => CloneFlags::CLONE_NEWUTS,
        LinuxNamespaceType::User => CloneFlags::CLONE_NEWUSER,
        LinuxNamespaceType::Cgroup => CloneFlags::CLONE_NEWCGROUP,
        _ => panic!("unknown namespace {:?}", namespace.typ()),
    }
}

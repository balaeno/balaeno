use super::OCI_VERSION;
use derive_builder::Builder;
use getset::{Getters, Setters};

#[derive(Clone)]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

#[derive(Getters, Setters, Builder, Clone)]
pub struct State {
    #[builder(default = "OCI_VERSION.to_string()")]
    pub(super) oci_version: String,
    pub(super) container_id: String,
    #[builder(default = "Status::Creating")]
    pub(super) status: Status,
    pub(super) pid: i32,
    pub(super) bundle: String,
}

pub fn state(container_id: String) -> Result<State> {
    let state = StateBuilder::default()
        .container_id(container_id)
        .status(Status::Creating)
        .pid(0)
        .bundle("".to_string())
        .build()
        .unwrap();
    Ok(state)
}

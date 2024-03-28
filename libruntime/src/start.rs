use crate::context::Context;
use crate::fs;
use anyhow::{anyhow, Result};
use log::{debug, info};

use oci_spec::runtime::Spec;
use vfs::{PhysicalFS, VfsPath};

pub struct StartBuilder {
    pub(super) id: String,
    pub(super) root: String,
}

impl StartBuilder {
    pub fn new(id: String, root: String) -> Self {
        Self { id, root }
    }
}

pub fn start(_ctx: Context, params: StartBuilder) -> Result<()> {
    debug!("start: id={}, root={}", params.id, params.root);
    let root_path =
        fs::abs_path(params.root.as_str()).expect("failed to get root path") + params.id.as_str();
    let root_fs: VfsPath = PhysicalFS::new(root_path).into();
    let mut config_str = String::new();
    root_fs
        .join("config.json")?
        .open_file()?
        .read_to_string(&mut config_str)?;

    info!("config_str: {:?}", config_str);
    let spec = serde_json::from_str::<Spec>(&config_str)?;
    info!("spec: {:?}", spec);
    Ok(())
}

use crate::context::Context;
use crate::fs;
use anyhow::Result;
use log::{debug, info};

use oci_spec::runtime::Spec;
use vfs::{PhysicalFS, VfsPath};

pub struct CreateBuilder {
    pub(super) bundle: String,
    pub(super) id: String,
}

impl CreateBuilder {
    pub fn new(bundle: String, id: String) -> Self {
        Self { bundle, id }
    }
}

pub fn create(_ctx: Context, params: CreateBuilder) -> Result<()> {
    debug!("create: bundle={}, id={}", params.bundle, params.id);
    let bundle_path = fs::abs_path(params.bundle.as_str()).expect("failed to get bundle path");
    let bundle_fs: VfsPath = PhysicalFS::new(bundle_path).into();
    let mut config_str = String::new();
    bundle_fs
        .join("config.json")?
        .open_file()?
        .read_to_string(&mut config_str)?;

    let spec = serde_json::from_str::<Spec>(&config_str)?;
    debug!("spec: {:?}", spec);



    Ok(())
}

use anyhow::Result;
use bollard::Docker;
use futures_util::{future::ready, StreamExt};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub async fn extract(image_url: String, path: String) -> Result<()> {
    let docker = Docker::connect_with_local_defaults().expect("failed to connect to docker");
    let image = docker.export_image(&image_url);

    fs::create_dir_all(&path).expect("failed to create directory");
    let temp_file_path = Path::new(&path).join("image.tar");
    let mut temp_file = File::create(temp_file_path).expect("failed to create temp file");
    // Shouldn't load the whole file into memory, stream it to disk instead
    image
        .for_each(move |data| {
            temp_file.write_all(&data.unwrap()).unwrap();
            temp_file.sync_all().unwrap();
            ready(())
        })
        .await;
    Ok(())
}

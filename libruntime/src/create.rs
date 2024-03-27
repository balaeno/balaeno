pub struct CreateBuilder {
    pub(super) bundle: String,
    pub(super) id: String,
}

impl CreateBuilder {
    pub fn new(bundle: String, id: String) -> Self {
        Self { bundle, id }
    }
}

pub fn create(params: CreateBuilder) {
    println!(
        "create container: bundle: {:?}, id: {:?}",
        params.bundle, params.id
    );
}

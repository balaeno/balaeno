use simple_logger::init_with_level;

pub fn init_logger() {
    init_with_level(log::Level::Info).unwrap();
}

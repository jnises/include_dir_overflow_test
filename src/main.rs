use include_dir::{include_dir, include_dir_impl};

static DIR: include_dir::Dir = include_dir!("stuff");

fn main() {
    println!("Hello, world!");
}

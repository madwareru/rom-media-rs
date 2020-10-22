use gl_generator::{
    Registry,
    Api,
    Profile,
    Fallbacks,
    GlobalGenerator
};

const OUT_DIRECTORY_ENV_VAR: &str = "OUT_DIR";

fn main() {

    let destination = std::env::var(OUT_DIRECTORY_ENV_VAR).unwrap();
    let path = std::path::Path::new(&destination).join("gl_bindings.rs");
    //let path_debug = format!("{:?}", &path);
    //panic!(path_debug.clone());

    let mut gl_bindings_file = std::fs::File::create(&path).unwrap();

    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut gl_bindings_file)
        .unwrap();
}
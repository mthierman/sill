use tools;

fn main() {
    println!("cargo::rustc-link-arg-bins=/WX");

    let data = tools::current_dir().parent().unwrap().join("data");

    let rc = data.join("app.rc");
    tools::compile_resource(rc);

    let manifest = data.join("app.manifest");
    tools::embed_manifest(manifest);
}

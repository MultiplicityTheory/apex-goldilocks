fn main() {
    let dir = std::path::PathBuf::from("tree-sitter-pirtm/src");
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-pirtm");
    println!("cargo:rerun-if-changed=tree-sitter-pirtm/src/parser.c");
}

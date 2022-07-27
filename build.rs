fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .type_attribute(".", "#[derive(Eq, Hash)]")
        .compile_protos(&["pwd.proto"], &["proto"])

        .unwrap();
}

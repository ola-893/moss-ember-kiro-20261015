fn main() {
    fission::site::build_from_cli(moss_ember::site_app())
        .expect("failed to build Fission site");
}

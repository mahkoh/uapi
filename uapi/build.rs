fn main() {
    cc::Build::new()
        .file("src/black_box.c")
        .compile("libblack_box.a");
}

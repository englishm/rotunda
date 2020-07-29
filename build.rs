use cc;

fn main() {
    cc::Build::new().file("src/c/src/util/tunnel.c")
        .include("src/c/third_party/assert-macros")
        .include("src/c/third_party/pt")
        .compile("tunnel");
}

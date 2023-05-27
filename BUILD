load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@crate_index//:defs.bzl", "all_crate_deps", "aliases")

rust_binary(
    name = "app",
    srcs = glob(["src/**/*.rs"]),
    edition = "2018",
    deps = all_crate_deps(),
)

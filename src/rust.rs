use indoc::formatdoc;

pub fn generate_cargo_toml(name: &String) -> String {
    // TODO: get author, GitHub username and email programmatically.
    formatdoc! {r#"
        [package]
        name = "{name}"
        version = "0.1.0"
        edition = "2021"
        authors = ["Matt Cook <matt@mattcook.dev>"]
        description = ""
        readme = "README.md"
        repository = "https://github.com/sciencefidelity/{name}"
        license = "MIT or Apache-2.0"

        [lints.rust]
        unsafe_code = "forbid"

        [lints.clippy]
        enum_glob_use = "deny"
        pedantic = {{ level = "deny", priority = 1 }}
        nursery = {{ level = "deny", priority = 2 }}
        unwrap_used = "deny"

        [profile.release]
        opt-level = "z"
        lto = true
        codegen-units = 1
        panic = "abort"
        strip = "symbols"

        [dependencies]
    "#}
}

pub fn generate_bin() -> String {
    formatdoc! {r#"
        fn main() {{
            println!("Hello, world!");
        }}
    "#}
}

pub fn generate_lib() -> String {
    formatdoc! {r#"
        pub fn add(left: u64, right: u64) -> u64 {{
            left + right
        }}

        #[cfg(test)]
        mod tests {{
            use super::*;

            #[test]
            fn it_works() {{
                let result = add(2, 2);
                assert_eq!(result, 4);
            }}
        }}
    "#}
}

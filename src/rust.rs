use indoc::formatdoc;

use crate::Args;

pub fn generate_cargo_toml(args: &Args) -> String {
    let name = args.name.as_str();
    let description = args.description.as_str();
    let edition = args.edition;
    let mut lint_groups = String::new();
    if !args.lint_groups.is_empty() {
        lint_groups.push_str("\n[lints.clippy]\n");
    }
    for (i, group) in args.lint_groups.iter().enumerate() {
        let text = formatdoc! {r#"
            {group} = {{ level = "deny", priority = {} }}
        "#, i + 1};
        lint_groups.push_str(text.as_str());
    }
    // TODO: get author, GitHub username and email programmatically.
    formatdoc! {r#"
        [package]
        name = "{name}"
        version = "0.1.0"
        edition = "{edition}"
        authors = ["Matt Cook <matt@mattcook.dev>"]
        description = "{description}"
        readme = "README.md"
        repository = "https://github.com/sciencefidelity/{name}"
        license = "MIT or Apache-2.0"

        [lints.rust]
        unsafe_code = "forbid"
        {lint_groups}
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

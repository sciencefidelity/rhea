use indoc::{formatdoc, indoc};

use crate::{config::Config, Args};

pub fn generate_cargo_toml(args: &Args, config: &Config) -> String {
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

    formatdoc! {r#"
        [package]
        name = "{name}"
        version = "0.1.0"
        edition = "{edition}"
        authors = ["{} <{}>"]
        description = "{description}"
        readme = "README.md"
        repository = "https://github.com/{}/{name}"
        license = "{}"

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
    "#,
    config.user.name,
    config.user.email,
    config.user.github_username,
    config.package.license
    }
}

pub fn generate_bin() -> String {
    indoc! {r#"
        fn main() {
            println!("Hello, world!");
        }
    "#}
    .to_owned()
}

pub fn generate_lib() -> String {
    indoc! {r#"
        pub fn add(left: u64, right: u64) -> u64 {
            left + right
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn it_works() {
                let result = add(2, 2);
                assert_eq!(result, 4);
            }
        }
    "#}
    .to_owned()
}

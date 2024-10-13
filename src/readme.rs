use indoc::formatdoc;

use crate::Args;

pub fn generate_readme(args: &Args) -> String {
    let name = args.name.as_str();
    let mut content = formatdoc! {r#"
        # {name}
    "#};

    if !args.description.is_empty() {
        content.push_str(
            formatdoc! {r"

            ## {}
        ", args.description}
            .as_str(),
        );
    };
    content
}

use oxc_coverage::{suite::Case, AppArgs, SemanticTypeScriptCase};
use pico_args::Arguments;
use rayon::ThreadPoolBuilder;

fn main() {
    return debug();

    let args = AppArgs {
        debug: true,
        filter: Some(
            "typescript/tests/cases/compiler/jsxNamespaceImplicitImportJSXNamespace.tsx".into(),
        ),
        detail: false,
        diff: false,
    };

    if args.debug {
        ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    }

    args.run_semantic();
}

fn debug() {
    let path = std::path::Path::new(
        "typescript/tests/cases/compiler/jsxNamespaceImplicitImportJSXNamespace.tsx",
    )
    .to_owned();
    let code = include_str!(
        "../typescript/tests/cases/compiler/jsxNamespaceImplicitImportJSXNamespace.tsx"
    );

    // remove the Byte Order Mark in some of the TypeScript files
    let code = code.trim_start_matches('\u{feff}').to_string();
    let mut case = SemanticTypeScriptCase::new(path, code);
    case.run();
}

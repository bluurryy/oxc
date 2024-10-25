use oxc_coverage::{
    suite::{Case, TestResult},
    AppArgs, SemanticTypeScriptCase,
};
use rayon::ThreadPoolBuilder;

fn main() {
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

    {
        let this = &mut case;
        let units = this.base.units.clone();

        {
            let unit = dbg!(&units[2]);

            this.base.code = unit.content.to_string();
            let result = this.execute(unit.source_type);
            if result != TestResult::Passed {
                this.base.result = result;
                return;
            }
        }

        this.base.result = TestResult::Passed;
    }
}

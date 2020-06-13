pub mod enum_derive;
pub mod fragment_arguments_derive;
pub mod fragment_derive;
pub mod inline_fragments_derive;
pub mod query_dsl;
pub mod query_module;
pub mod scalar_derive;

mod error;
mod field_argument;
mod field_type;
mod ident;
mod module;
mod schema;
mod type_index;
mod type_path;

use error::Error;
use field_argument::FieldArgument;
use field_type::FieldType;
use ident::Ident;
use schema::load_schema;
use type_index::TypeIndex;
use type_path::TypePath;

pub fn output_query_dsl<P: AsRef<std::path::Path>>(schema: P, output_path: P) -> Result<(), Error> {
    use query_dsl::QueryDslParams;
    use std::io::Write;

    let tokens = query_dsl::query_dsl_from_schema(QueryDslParams {
        schema_filename: schema.as_ref().to_str().unwrap().to_string(),
    })?;

    {
        let mut out = std::fs::File::create(output_path.as_ref()).unwrap();
        write!(&mut out, "{}", tokens).unwrap();
    }

    format_code(output_path.as_ref());

    Ok(())
}

#[allow(unused_variables)]
fn format_code(filename: &std::path::Path) {
    #[cfg(feature = "rustfmt")]
    {
        std::process::Command::new("cargo")
            .args(&["fmt", "--", filename.to_str().unwrap()])
            .spawn()
            .expect("failed to execute process");
    }
    #[cfg(not(feature = "rustfmt"))]
    {
        return code;
    }
}

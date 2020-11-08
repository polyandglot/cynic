mod inputs;
mod leaf_types;
mod normalisation;
mod sorting;
mod types;
mod value;

use value::Value;

use crate::{query::Document, schema::OutputType, Error, TypeIndex};

use std::rc::Rc;

pub fn parse_query_document<'text>(
    doc: &Document<'text>,
    type_index: &Rc<TypeIndex<'text>>,
) -> Result<types::Output<'text, 'text>, Error> {
    let normalised = normalisation::normalise(doc, type_index)?;
    let input_objects = inputs::extract_input_objects(&normalised)?;

    let (enums, scalars) = leaf_types::extract_leaf_types(&normalised, &input_objects, type_index)?;

    // TODO: Ok, so in here i think we should name things.
    // Probably after the top sort.

    let query_fragments = sorting::topological_sort(normalised.selection_sets.into_iter())
        .into_iter()
        .map(make_query_fragment)
        .collect::<Result<Vec<_>, _>>()?;

    let input_objects = sorting::topological_sort(input_objects.into_iter())
        .into_iter()
        .map(make_input_object)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(types::Output {
        query_fragments,
        input_objects,
        enums,
        scalars,
        // TODO: argument structs
        argument_structs: vec![],
    })
}

fn make_query_fragment<'text>(
    selection: Rc<normalisation::SelectionSet<'text, 'text>>,
) -> Result<types::QueryFragment<'text, 'text>, Error> {
    use crate::{schema::TypeDefinition, type_ext::TypeExt};
    use normalisation::Selection;
    use types::{Field, FieldArgument};

    Ok(types::QueryFragment {
        fields: selection
            .selections
            .iter()
            .map(|selection| match selection {
                Selection::Field(field) => {
                    let schema_field = &field.schema_field;

                    Field {
                        name: schema_field.name,
                        field_type: schema_field.value_type.clone(),
                        arguments: field
                            .arguments
                            .iter()
                            .map(|(def, value)| -> Result<FieldArgument, Error> {
                                Ok(FieldArgument::new(
                                    def.name,
                                    value.clone(),
                                    def.value_type.clone(),
                                ))
                            })
                            .collect::<Result<Vec<_>, _>>()
                            .unwrap(),
                    }
                }
            })
            .collect(),
        argument_struct_name: None,
        name: selection.target_type.name().to_string(),
        target_type: selection.target_type.name().to_string(),
    })
}

fn make_input_object<'text>(
    input: Rc<inputs::InputObject>,
) -> Result<types::InputObject<'text>, Error> {
    use crate::{schema::TypeDefinition, type_ext::TypeExt};
    use normalisation::Selection;
    use types::{Field, FieldArgument};

    let mut fields = Vec::new();
    for (field_name, _) in &input.fields {
        fields.push(todo!());
    }

    Ok(types::InputObject {
        name: input.target_type.clone(),
        fields,
    })
}

pub use queries::*;

#[cynic::query_module(schema_path = r#"src/schema.graphql"#, query_module = "query_dsl")]
mod queries {
    use super::query_dsl;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct IntrospectionQuery {
        pub schema: Schema,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Schema")]
    pub struct Schema {
        pub query_type: TypeRef,
        pub mutation_type: Option<TypeRef>,
        pub subscription_type: Option<TypeRef>,
        pub types: Vec<SchemaType>,
        pub directives: Vec<Directive>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Directive")]
    pub struct Directive {
        pub name: String,
        pub description: Option<String>,
        pub args: Vec<InputValue>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct SchemaType {
        pub kind: TypeKind,
        pub name: Option<String>,
        pub description: Option<String>,
        #[arguments(include_deprecated = true)]
        pub fields: Option<Vec<Field>>,
        pub input_fields: Option<Vec<InputValue>>,
        pub interfaces: Option<Vec<TypeTree>>,
        #[arguments(include_deprecated = true)]
        pub enum_values: Option<Vec<EnumValue>>,
        pub possible_types: Option<Vec<TypeTree>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__EnumValue")]
    pub struct EnumValue {
        pub name: String,
        pub description: Option<String>,
        pub is_deprecated: bool,
        pub deprecation_reason: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Field")]
    pub struct Field {
        pub name: String,
        pub description: Option<String>,
        pub args: Vec<InputValue>,
        pub r#type: TypeTree,
        pub is_deprecated: bool,
        pub deprecation_reason: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__InputValue")]
    pub struct InputValue {
        pub name: String,
        pub description: Option<String>,
        pub r#type: TypeTree,
        pub default_value: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct TypeTree {
        pub kind: TypeKind,
        pub name: Option<String>,
        #[cynic(recurse = 6)]
        pub of_type: Box<Option<TypeTree>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct TypeRef {
        pub name: Option<String>,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(graphql_type = "__TypeKind", rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum TypeKind {
        Scalar,
        Object,
        Interface,
        Union,
        Enum,
        InputObject,
        List,
        NonNull,
    }
}

mod query_dsl {
    cynic::query_dsl!(r#"src/schema.graphql"#);
}

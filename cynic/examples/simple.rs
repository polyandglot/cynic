fn main() {
    //println!("{}", cynic::to_query::<TestStruct>(TestArgs {}));
}

mod query_dsl {
    type Json = serde_json::Value;

    cynic::query_dsl!("examples/simple.graphql");
}

use cynic::selection_set;

#[derive(Clone, cynic::FragmentArguments)]
struct TestArgs {}

#[derive(cynic::QueryFragment)]
#[cynic(
    schema_path = "examples/simple.graphql",
    query_module = "query_dsl",
    graphql_type = "TestStruct",
    argument_struct = "TestArgs"
)]
struct TestStruct {
    #[cynic_arguments(x = Some(1), y = Some("1".to_string()))]
    field_one: String,
    nested: Nested,
    opt_nested: Option<Nested>,
}

impl TestStruct {
    fn new(field_one: String, nested: Nested) -> Self {
        TestStruct {
            field_one,
            nested,
            opt_nested: None,
        }
    }

    fn new2((field_one, nested): (String, Nested)) -> Self {
        TestStruct {
            field_one,
            nested,
            opt_nested: None,
        }
    }
}

#[derive(cynic::QueryFragment)]
#[cynic(
    schema_path = "examples/simple.graphql",
    query_module = "query_dsl",
    graphql_type = "Nested"
)]
struct Nested {
    a_string: String,
    opt_string: Option<String>,
}

impl Nested {
    fn new(a_string: String) -> Self {
        Nested {
            a_string: a_string,
            opt_string: None,
        }
    }
}

#[derive(cynic::QueryFragment)]
#[cynic(
    schema_path = "examples/simple.graphql",
    query_module = "query_dsl",
    graphql_type = "TestStruct"
)]
struct Test {
    #[cynic_arguments(x = Some(1), y = Some("1".to_string()))]
    field_one: String,
}

impl Test {
    fn new(field_one: String) -> Self {
        Test { field_one }
    }
}

#[derive(cynic::InlineFragments)]
#[cynic(
    schema_path = "examples/simple.graphql",
    query_module = "query_dsl",
    graphql_type = "MyUnionType"
)]
enum MyUnionType {
    Test(Test),
    Nested(Nested),
}

fn query() {
    // Current:

    let query = query_dsl::Query::test_struct(selection_set::map2(
        TestStruct::new,
        query_dsl::TestStruct::field_one(query_dsl::test_struct::FieldOneOptionalArgs::default()),
        query_dsl::TestStruct::nested(selection_set::map(
            Nested::new,
            query_dsl::Nested::a_string(),
        )),
    ));

    // With builder-style structs for args & a select function.
    let query = query_dsl::Query::test_struct()
        .an_arg(1)
        .select(selection_set::map2(
            TestStruct::new,
            query_dsl::TestStruct::field_one(),
            query_dsl::TestStruct::nested().select(selection_set::map(
                Nested::new,
                query_dsl::Nested::a_string(),
            )),
        ));

    // Builder style & field selection macro.
    let query = query_dsl::Query::test_struct()
        .an_arg(1)
        .select(selection_set::fields!(
            query_dsl::TestStruct::field_one(),
            query_dsl::TestStruct::nested().select(
                selection_set::fields!(query_dsl::Nested::a_string()).construct(Nested::new)
            )
        ))
        .construct(TestStruct::new);

    // Builder style w/ field selection macro but prefix constructors.
    let query = query_dsl::Query::test_struct().an_arg(1).build(
        TestStruct::new,
        selection_set::fields!(
            query_dsl::TestStruct::field_one(x, y),
            query_dsl::TestStruct::nested().build(
                Nested::new,
                selection_set::fields!(query_dsl::Nested::a_string())
            )
        ),
    );

    // Builder style w/ prefix constructors and just some tuples.
    let query = query_dsl::Query::test_struct().an_arg(1).build(
        TestStruct::new,
        (
            query_dsl::TestStruct::field_one(x, y),
            query_dsl::TestStruct::nested().build(Nested::new, (query_dsl::Nested::a_string(),)),
        ),
    );

    trait TN<TypeLock> {
        type SelectionSets;

        fn convert(other: Self::SelectionSets) -> Self {
            // Handwaving a bit about how this is implemented, reckon
            // it'd need some sort of TupleDecoder concept that has impls for
            // all the Ns and knows how to build up a tuple by calling a series of decoders
            // a bunch.
            todo!()
        }
    }

    impl<TypeLock, T> TN<TypeLock> for (T,) {
        type SelectionSets = (cynic::SelectionSet<'static, T, TypeLock>,);
    }

    impl<TypeLock, T1, T2> TN<TypeLock> for (T1, T2) {
        type SelectionSets = (
            cynic::SelectionSet<'static, T1, TypeLock>,
            cynic::SelectionSet<'static, T2, TypeLock>,
        );
    }

    /*
    // Testing the above
    fn build<Tuple, Out>(f: impl Fn(Tuple) -> Out, t: Tuple) -> Out {
        f(t)
    }*/
    fn build<Tuple: TN<TypeLock>, Out, TypeLock>(
        f: impl Fn(Tuple) -> Out,
        t: <Tuple as TN<TypeLock>>::SelectionSets,
    ) {
        f(Tuple::convert(t));
    }

    macro_rules! fields {
        ($($x:expr),+ $(,)?) => (
            ($($x),*)
        );
    }

    build(TestStruct::new2, fields!("a", "a"));
    build(
        TestStruct::new2,
        fields!(
            query_dsl::TestStruct::field_one(
                query_dsl::test_struct::FieldOneOptionalArgs::default()
            ),
            query_dsl::TestStruct::nested(selection_set::map(
                Nested::new,
                query_dsl::TestStruct::a_string(),
            ))
        ),
    )
}

impl cynic::QueryRoot for query_dsl::TestStruct {}

// TODO: Some sort of ToQuery trait
// That's only implemented when QueryFragment::SelectionSet::TypeLock == RootQuery
// TODO: I should figure out how arguments could work?

/*

impl cynic::QueryFragment<'static> for TestStruct {
    type SelectionSet = selection_set::SelectionSet<'static, Self, query_dsl::TestStruct>;
    type Arguments = ArgStruct;

    fn query() -> Self::SelectionSet {
        // TODO: Got to say I'm not that enamoured with this syntax.
        // Is there a better way to write this?
        selection_set::map2(
            TestStruct::new,
            query_dsl::TestStruct::field_one(),
            query_dsl::TestStruct::nested(Nested::selection_set()),
        )
    }
}

impl cynic::QueryFragment<'static> for Nested {
    type SelectionSet = selection_set::SelectionSet<'static, Self, query_dsl::Nested>;

    fn query() -> Self::SelectionSet {
        selection_set::map(Nested::new, query_dsl::Nested::a_string())
    }
}
*/

mod test {

    type JSON = serde_json::Value;

    // A custom scalars.
    pub struct DateTime {}

    impl cynic::Scalar for DateTime {
        fn decode(_: &serde_json::Value) -> Result<Self, json_decode::DecodeError> {
            Ok(DateTime {})
        }
        fn encode(&self) -> Result<serde_json::Value, ()> {
            todo!()
        }
    }

    // Another custom scalar
    struct Upload;

    //cynic::query_dsl!("cms-schema.gql");
}

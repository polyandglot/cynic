use std::marker::PhantomData;

use crate::SelectionSet;
use json_decode::BoxDecoder;

/*
Goal API:
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
    */

trait SelectableTuple<'a, TypeLock> {
    type SelectionSets;

    fn fields_and_decoder(
        selection_sets: Self::SelectionSets,
    ) -> (Vec<crate::field::Field>, BoxDecoder<'a, Self>);
}

impl<'a, TypeLock, T> SelectableTuple<'a, TypeLock> for (T,)
where
    T: 'a,
{
    type SelectionSets = (SelectionSet<'a, T, TypeLock>,);

    fn fields_and_decoder(
        selection_sets: Self::SelectionSets,
    ) -> (Vec<crate::field::Field>, BoxDecoder<'a, Self>) {
        let mut selection_sets = selection_sets;
        let mut fields = Vec::with_capacity(1);
        fields.append(&mut selection_sets.0.fields);
        (fields, json_decode::tuple_1(selection_sets.0.decoder))
    }
}

impl<'a, TypeLock, T1, T2> SelectableTuple<'a, TypeLock> for (T1, T2)
where
    T1: 'a,
    T2: 'a,
{
    type SelectionSets = (
        SelectionSet<'a, T1, TypeLock>,
        SelectionSet<'a, T2, TypeLock>,
    );

    fn fields_and_decoder(
        selection_sets: Self::SelectionSets,
    ) -> (Vec<crate::field::Field>, BoxDecoder<'a, Self>) {
        let mut selection_sets = selection_sets;
        let mut fields = Vec::with_capacity(2);
        fields.append(&mut selection_sets.0.fields);
        fields.append(&mut selection_sets.1.fields);
        (
            fields,
            json_decode::tuple_2(selection_sets.0.decoder, selection_sets.1.decoder),
        )
    }
}

/*
// Testing the above
fn build<Tuple, Out>(f: impl Fn(Tuple) -> Out, t: Tuple) -> Out {
    f(t)
}*/
fn select_tuple_into<'a, Tuple: SelectableTuple<'a, TypeLock>, Out, TypeLock>(
    f: impl Fn(Tuple) -> Out + 'a + Send + Sync,
    selections: <Tuple as SelectableTuple<'a, TypeLock>>::SelectionSets,
) -> SelectionSet<'a, Out, TypeLock>
where
    Tuple: 'a,
    Out: 'a,
{
    let (fields, tuple_decoder) = Tuple::fields_and_decoder(selections);

    SelectionSet {
        fields,
        decoder: json_decode::map(f, tuple_decoder),
        phantom: PhantomData,
    }
}

macro_rules! fields {
        ($($x:expr),+ $(,)?) => (
            ($($x),*)
        );
    }

fn test() {
    use crate::selection_set::{integer, string};

    // TODO: Ok, suspicion confirmed: error messages on this are terrible...
    // Would building a selection set tuple _then_ mapping be better for error messages?
    // Hard to say...
    select_tuple_into(|(a, b): (String, i64)| (), fields!(integer(), integer()));
}

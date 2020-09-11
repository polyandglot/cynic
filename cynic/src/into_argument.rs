use crate::{argument::SerializableArgument, Id};

/// IntoArgument is used to type-check arguments to queries in cynic.
///
/// A GraphQL argument that accepts `String!` will accept any type that is
/// `IntoArgument<String>`.  Similarly, an optional `String` in GraphQL will
/// accept any `IntoArgument<Option<String>>`.
///
/// There are implementations of this for most of the built in scalars to allow
/// users to express arguments in a simple manner.  The `cynic::Enum` derive
/// also generates impls for converting options & refs easily.
pub trait IntoArgument<T> {
    type Output: SerializableArgument;

    fn into_argument(self) -> Self::Output;
}

// TODO: Ok, so what if instead of IntoArgument<T> where T is the
// destination type, we did IntoArgument<TypeLock> where TypeLock is the
// enum typelock or similar?
// Would need somewhere for the return value to live...
// Could it live on an associated type?  Suppose it could but would
// be an easy signature to fuck up...

// Could also maybe do IntoArgument<Argument, TypeLock> ?

// Would it help to have IntoScalar for scalars, IntoEnum for enums and
// IntoInputObject for input objects?
// Could then do away with the `impl IntoArgument<impl Enum<T>>` and just have
// impl IntoEnum<T>.  Or could I?  Not sure actually...

// Ok, a radical idea - what if IntoArgument was only used in the fragment derive
// layer, and _just_ for converting something from Option to a given type?
//
// pub trait IntoArgument<FieldModifiers> {
//     type Output;
//     fn into(self) -> Self::Output
// }
//
// impl<T> IntoArgument<Required> for T {
//     type Output = T;
//     fn into(self) -> T {
//         return self
//     }
// }
//
// impl<T> IntoArgument<NotRequired> for T {
//     type Output = Option<T>;
//     fn into(self) -> T {
//         return Some(self)
//     }
// }
//
// Fuck, this doesn't work because this is a conflicting impl after the above...
// TODO: Can I do this, but in a macro like I have now?
//
// impl<T> IntoArgument<NotRequired> for Option<T> {
//     type Output = Option<T>;
//     fn into(self) -> T {
//         return self
//     }
// }
//
// Used like
// .states(IntoArgument::<Required>.into(IssueState::Open))
//
//

impl<T> IntoArgument<T> for T
where
    T: SerializableArgument,
{
    type Output = T;

    fn into_argument(self) -> T {
        self
    }
}

/// Defines useful argument conversions for scalar-like types
///
/// Mostly just converts references to owned via cloning and
/// non option-wrapped types into Option where appropriate.
#[macro_export]
macro_rules! impl_into_argument_for_options {
    ($inner:ty) => {
        impl $crate::IntoArgument<Option<$inner>> for $inner {
            type Output = Option<$inner>;

            fn into_argument(self) -> Option<$inner> {
                Some(self)
            }
        }

        impl<'a> $crate::IntoArgument<Option<$inner>> for &'a $inner {
            type Output = Option<&'a $inner>;

            fn into_argument(self) -> Option<&'a $inner> {
                Some(self)
            }
        }

        impl $crate::IntoArgument<Vec<$inner>> for $inner {
            type Output = Vec<$inner>;

            fn into_argument(self) -> Vec<$inner> {
                vec![self]
            }
        }

        impl $crate::IntoArgument<Option<Vec<$inner>>> for $inner {
            type Output = Option<Vec<$inner>>;

            fn into_argument(self) -> Option<Vec<$inner>> {
                Some(vec![self])
            }
        }

        impl $crate::IntoArgument<Vec<Option<$inner>>> for $inner {
            type Output = Vec<Option<$inner>>;

            fn into_argument(self) -> Vec<Option<$inner>> {
                vec![Some(self)]
            }
        }

        /*
        impl $crate::IntoArgument<Option<Vec<Option<$inner>>>> for $inner {
            fn into_argument(self) -> Option<Vec<Option<$inner>>> {
                Some(vec![Some(self)])
            }
        }*/

        /*
        TODO: re-instate this?
        impl $crate::into_argument::IntoArgument2<$crate::into_argument::Required> for $inner {
            type Output = $inner;
            fn into(self) -> $inner {
                return self;
            }
        }

        impl $crate::into_argument::IntoArgument2<$crate::into_argument::NotRequired> for $inner {
            type Output = Option<$inner>;
            fn into(self) -> Option<$inner> {
                return Some(self);
            }
        }
        */
    };
}

macro_rules! impl_into_argument_for_option_refs {
    ($inner:ty) => {
        impl<'a> $crate::IntoArgument<Option<$inner>> for Option<&'a $inner> {
            type Output = Option<&'a $inner>;

            fn into_argument(self) -> Option<&'a $inner> {
                self
            }
        }

        impl<'a> $crate::IntoArgument<Option<$inner>> for &'a Option<$inner> {
            type Output = Option<&'a $inner>;

            fn into_argument(self) -> Option<&'a $inner> {
                self.as_ref()
            }
        }
    };
}

impl_into_argument_for_options!(i32);
impl_into_argument_for_options!(f64);
impl_into_argument_for_options!(String);
impl_into_argument_for_options!(bool);
impl_into_argument_for_options!(Id);

impl_into_argument_for_option_refs!(i32);
impl_into_argument_for_option_refs!(f64);
impl_into_argument_for_option_refs!(String);
impl_into_argument_for_option_refs!(bool);
impl_into_argument_for_option_refs!(Id);

impl<'a> IntoArgument<String> for &'a str {
    type Output = &'a str;

    fn into_argument(self) -> &'a str {
        self
    }
}

impl<'a> IntoArgument<Option<String>> for &'a str {
    type Output = Option<&'a str>;

    fn into_argument(self) -> Option<&'a str> {
        Some(self)
    }
}

impl<'a> IntoArgument<Option<String>> for Option<&'a str> {
    type Output = Option<&'a str>;

    fn into_argument(self) -> Option<&'a str> {
        self
    }
}

// New stuff

pub trait IntoArgument2<FieldModifiers> {
    type Output;
    fn into(self) -> Self::Output;
}

pub struct Required;
pub struct NotRequired;

impl<T> IntoArgument2<NotRequired> for Option<T>
where
    T: IntoArgument2<Required>,
{
    type Output = Option<<T as IntoArgument2<Required>>::Output>;

    fn into(self) -> Self::Output {
        self.map(IntoArgument2::<Required>::into)
    }
}

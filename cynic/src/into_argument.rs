use crate::{argument::SerializableArgument, Id};

/// IntoArgument is used to type-check arguments to queries in cynic.
///
/// A GraphQL argument that accepts `String!` will accept any type that is
/// `IntoArgument<String>`.  Similarly, an optional `String` in GraphQL will
/// accept any `IntoArgument<Option<String>>`.
///
/// There are implementations of this for most of the built in scalars to allow
/// users to express arguments in a simple manner.
pub trait IntoArgument<Argument>
where
    Argument: SerializableArgument + Send + 'static,
{
    fn into_argument(self) -> Argument;
}

impl<T> IntoArgument<T> for T
where
    T: SerializableArgument + Send + 'static,
{
    fn into_argument(self) -> T {
        self
    }
}

macro_rules! define_for_scalar {
    ($inner:ty) => {
        impl IntoArgument<Option<$inner>> for $inner {
            fn into_argument(self) -> Option<$inner> {
                Some(self.clone())
            }
        }

        impl IntoArgument<Option<$inner>> for Option<&$inner> {
            fn into_argument(self) -> Option<$inner> {
                self.cloned()
            }
        }

        impl IntoArgument<Option<$inner>> for &Option<$inner> {
            fn into_argument(self) -> Option<$inner> {
                self.clone()
            }
        }
    };
}

define_for_scalar!(i32);
define_for_scalar!(f64);
define_for_scalar!(String);
define_for_scalar!(bool);
define_for_scalar!(Id);

impl IntoArgument<String> for &str {
    fn into_argument(self) -> String {
        self.to_string()
    }
}

impl IntoArgument<Option<String>> for &str {
    fn into_argument(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl IntoArgument<Option<String>> for Option<&str> {
    fn into_argument(self) -> Option<String> {
        self.map(|s| s.to_string())
    }
}

// TODO: Do I also want to define things for Vecs?

// TODO: Define some more for Enums & InputObjects, though maybe want the derives to take care
//       of that.

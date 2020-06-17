use crate::SerializeError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id(String);

impl Id {
    pub fn new(s: impl Into<String>) -> Self {
        Id(s.into())
    }
    pub fn inner(&self) -> &str {
        return &self.0;
    }

    pub fn into_inner(self) -> String {
        return self.0;
    }
}

impl<T: Into<String>> From<T> for Id {
    fn from(s: T) -> Id {
        Id(s.into())
    }
}

impl crate::Scalar for Id {
    fn decode(value: &serde_json::Value) -> Result<Self, json_decode::DecodeError> {
        String::decode(value).map(Into::into)
    }

    fn encode(&self) -> Result<serde_json::Value, SerializeError> {
        self.0.encode()
    }
}

/*
enum MaybeBorrowed<'a, T: 'a> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> From<T> for MaybeBorrowed<'_, T> {
    fn from(t: T) -> MaybeBorrowed<'static, T> {
        MaybeBorrowed::Owned(t)
    }
}

impl<'a, T> From<&'a T> for MaybeBorrowed<'a, T> {
    fn from(t: &'a T) -> MaybeBorrowed<'a, T> {
        MaybeBorrowed::Borrowed(t)
    }
}

fn test<'a, R, T: Into<MaybeBorrowed<'a, R>>>(
    x: impl Into<Option<T>>,
) -> Option<MaybeBorrowed<'a, R>> {
    x.into().map(|x| x.into()).into()
}

fn other() {
    let x: Option<MaybeBorrowed<String>> = test("ah");
}
*/

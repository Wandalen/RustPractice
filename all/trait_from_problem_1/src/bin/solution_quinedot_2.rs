fn main() {
    /* from i32 to Struct */
    {
        let src = Struct1 { a: 13 };
        let got: i32 = src.into2();
        let exp = 13;
        assert_eq!(got, exp);
    }

    /* from &i32 to Struct */
    {
        let src = Struct1 { a: 13 };
        let got: i32 = (&&&&src).into2();
        let exp = 13;
        assert_eq!(got, exp);
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Struct1 {
    a: i32,
}

pub trait AsNumber: From<i32> + Into<i32> {
    fn into2<Other: AsNumber>(self) -> Other where Self: Sized {
        Other::from(self.into())
    }
}

impl AsNumber for i32 {}

impl From<i32> for Struct1 {
    fn from(a: i32) -> Self {
        Self { a }
    }
}

impl From<Struct1> for i32 {
    fn from(s: Struct1) -> Self {
        s.a
    }
}

impl AsNumber for Struct1 {}
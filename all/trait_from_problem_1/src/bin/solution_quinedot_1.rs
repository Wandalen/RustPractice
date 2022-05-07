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

pub trait AsNumber {
    fn make(src: i32) -> Self;
    fn as_number(self) -> i32;
    fn into2<Other: AsNumber>(self) -> Other where Self: Sized {
        Other::make(self.as_number())
    }
}

impl AsNumber for i32 {
    fn make(src: i32) -> Self {
        src
    }
    fn as_number(self) -> i32 {
        self
    }
}

impl AsNumber for Struct1 {
    fn make(src: i32) -> Self {
        Struct1 { a: src }
    }
    fn as_number(self) -> i32 {
        self.a
    }
}

pub enum Enum<'a> {
    A(&'a str),
    B { ptr: usize, len: usize },
    C(&'a [u8]),
    D(u8),
}

impl Enum<'_> {
    pub(crate) fn foo(&self, tmp: &mut Vec<u8>) {
        match self {
            Self::A(_) => tmp.push(0),
            Enum::B { .. } => tmp.push(1),
            Enum::C(_) => tmp.push(2),
            Enum::D(_) => tmp.push(3),
        }
    }
}

pub struct B<'a> {
    slice: &'a [Enum<'a>],
}
impl B<'_> {
    pub fn foo(&self, tmp: &mut Vec<u8>) {
        for e in self.slice {
            e.foo(tmp);
        }
    }
}

use bounce::Atom;


#[derive(Atom, PartialEq)]
pub struct Count {
    pub value: u32,
}

impl Default for Count {
    fn default() -> Self {
        Count {
            value: 0,
        }
    }
}

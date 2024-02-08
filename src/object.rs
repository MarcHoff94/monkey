use std::fmt::Debug;

#[derive(Debug)]
pub enum MonkeyObject {
    INTEGER(Integer),
    BOOLEAN(Bool),
    NULL(Null),
    BLOCK(Block),
}

impl MonkeyObject {
    pub fn into_obj(self) -> Box<dyn Object> {
        match self {
            Self::INTEGER(x) => Box::new(x),
            Self::BOOLEAN(x) => Box::new(x),
            Self::NULL(x) => Box::new(x),
            Self::BLOCK(x) => Box::new(x),
        }
    }
}

pub trait Object: ObjectInterface + Debug {}

pub trait ObjectInterface {
    fn inspect(&self) -> String;
}
#[derive(Debug)]
pub struct Integer {
    pub value: i64
}
impl Integer {
    pub fn new(val: i64) -> Integer {
        Integer { value: val }
    }
}
impl Object for Integer {}

impl ObjectInterface for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct Bool {
    pub value: bool
}
impl Bool {
    pub fn new(val:bool) -> Bool {
        Bool{ value: val}
    }
}
impl Object for Bool {}

impl ObjectInterface for Bool {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct Null {}
impl Object for Null {}
impl ObjectInterface for Null {
    fn inspect(&self) -> String {
        String::from("Null")
    }
}
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<MonkeyObject>,
}
impl Object for Block {}
impl ObjectInterface for Block {
    fn inspect(&self) -> String {
        String::from("Number of Statements in block:{self.statements.len()}")
    }
}
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
}
trait Object: ObjectInterface + Debug {}

pub trait ObjectInterface {
    fn r#type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
#[derive(Debug)]
pub struct Integer {
    Value: i64
}
impl Integer {
    pub fn new(val: i64) -> Integer {
        Integer { Value: val }
    }
}
impl Object for Integer {}

impl ObjectInterface for Integer {
    fn r#type(&self) -> ObjectType {
        ObjectType::INTEGER
    }
    fn inspect(&self) -> String {
        self.Value.to_string()
    }
}

#[derive(Debug)]
pub struct Bool {
    Value: bool
}
impl Bool {
    pub fn new(val:bool) -> Bool {
        Bool{ Value: val}
    }
}
impl Object for Bool {}

impl ObjectInterface for Bool {
    fn r#type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }
    fn inspect(&self) -> String {
        self.Value.to_string()
    }
}

#[derive(Debug)]
pub struct Null {}
impl Object for Null {}
impl ObjectInterface for Null {
    fn r#type(&self) -> ObjectType {
        ObjectType::NULL
    }
    fn inspect(&self) -> String {
        String::from("Null")
    }
}
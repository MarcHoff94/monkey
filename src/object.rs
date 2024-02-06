use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
    VARIABLE,
}
pub trait Object: ObjectInterface + Debug {}

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
    pub value: bool
}
impl Bool {
    pub fn new(val:bool) -> Bool {
        Bool{ value: val}
    }
}
impl Object for Bool {}

impl ObjectInterface for Bool {
    fn r#type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }
    fn inspect(&self) -> String {
        self.value.to_string()
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

#[derive(Debug)]
pub struct Variable {
    name: String,
    value: Box<dyn Object>
}
impl Variable {
    pub fn new(name: String, val: Box<dyn Object>) -> Variable {
        Variable { name: name, value:  val }
    }
}
impl Object for Variable {}

impl ObjectInterface for Variable {
    fn r#type(&self) -> ObjectType {
        ObjectType::VARIABLE
    }
    fn inspect(&self) -> String {
        self.name.to_string()
    }
}
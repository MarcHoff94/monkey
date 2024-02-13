use std::{cell::{Ref, RefCell}, collections::HashMap, fmt::Debug, rc::Rc};

use crate::token::{BlockStatement, Identifier};

#[derive(Debug, PartialEq, Clone)]
pub enum MonkeyObject {
    INTEGER(Integer),
    BOOLEAN(Bool),
    NULL(Null),
    BLOCK(Block),
    RETURN(ReturnValue),
    FUNCTION(Function),
}

impl MonkeyObject {
    pub fn into_obj(self) -> Box<dyn Object> {
        match self {
            Self::INTEGER(x) => Box::new(x),
            Self::BOOLEAN(x) => Box::new(x),
            Self::NULL(x) => Box::new(x),
            Self::BLOCK(x) => Box::new(x),
            Self::RETURN(x) => Box::new(x),
            Self::FUNCTION(x) => Box::new(x),
        }
    }
}

pub trait Object: ObjectInterface + Debug {}

pub trait ObjectInterface {
    fn inspect(&self) -> String;
}
#[derive(Debug, PartialEq,Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq,Clone)]
pub struct Null {}
impl Object for Null {}
impl ObjectInterface for Null {
    fn inspect(&self) -> String {
        String::from("Null")
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<MonkeyObject>,
}
impl Object for Block {}
impl ObjectInterface for Block {
    fn inspect(&self) -> String {
        String::from("Number of Statements in block:{self.statements.len()}")
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct ReturnValue {
    pub value: Box<MonkeyObject>
}
impl ReturnValue {
    pub fn new(value: Box<MonkeyObject>) -> ReturnValue {
        ReturnValue { value: value }
    }
}
impl Object for ReturnValue {}
impl ObjectInterface for ReturnValue {
    fn inspect(&self) -> String {
        String::from("dummy")
    }
}
#[derive(Debug, PartialEq,Clone)]
pub struct Function  {
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>
}
impl Function {
    pub fn new(params: Vec<Identifier>, body: BlockStatement, env: Rc<RefCell<Environment>>) -> Function {
        Function { params: params, body: body, env: env }
    }
}
impl Object for Function {}
impl ObjectInterface for Function {
    fn inspect(&self) -> String {
        String::from("dummy")
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct Environment {
    store: HashMap<String, MonkeyObject>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn new() -> Environment {
        Environment { store: HashMap::new(), outer: None }
    }
    pub fn new_enclosed_evironment(hashmap: HashMap<String, MonkeyObject>, env: Rc<RefCell<Environment>>) -> Environment {
        Environment{ store: hashmap, outer: Some(Rc::clone(&env))}
    }
    pub fn get(&self, key: &String) -> Option<MonkeyObject> {
        match &self.outer {
            Some(x) => {
                match x.borrow().get(key) {
                    Some(monkeyobj) => Some(monkeyobj.clone()),
                    None => match self.store.get(key) {
                        Some(x) => Some(x.clone()),
                        None => None,
                    }
                }
            },
            None => match self.store.get(key) {
                Some(x) => Some(x.clone()),
                None => None,
            }
        }
    }
    pub fn set(&mut self, key: String, value: MonkeyObject) {
        self.store.insert(key, value);
    }
}
pub mod tra;
use self::tra::Tra;


pub struct Type {
    a: i32,
    b: String,
}

impl Type {
    pub fn new(a: i32, b: &str) -> Type {
        Type {
            a: a,
            b: b.to_string(),
        }
    }

    pub fn concat(&self) -> String {
        "".to_string() + &self.b + &self.a.to_string()
    }
}

impl Tra for Type {
    fn foo(&self) -> i32 {
        self.a
    }
}

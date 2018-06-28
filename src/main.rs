
trait Console_name {
    fn console(&self) ->String {
        "naruto!".to_string()
    }
    fn format_name(&self) -> String;
}
#[derive(Debug)]
struct Persion {
    name: String,
    age: u32,
    addr: String,
}
impl Console_name for Persion {
    fn format_name(&self) -> String {
        let a = format!("{},{}",self.name,self.addr);
        a
    }
}


fn main() {
    let y = Persion {
        name: String::from("joe michael"),
        age: 22,
        addr: String::from("beijing,china")
    };
    println!("{}", y.format_name());
}
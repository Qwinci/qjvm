use crate::vm::{ClassInstance, Frame, JType, VM};

mod loader;
mod vm;

fn main() {
    let main_class = loader::load_class("tests/Add.class");
    let instance = ClassInstance::new(&main_class);
    let vm = VM::new();

    let arg1 = JType::Int(1);
    let arg2 = JType::Int(2);
    let frame = Frame::new(instance, "add", &[arg1, arg2]);
    let ret = vm.execute(frame);
    let ret_value = ret.int_array();
    let ret_value = ret_value.borrow();
    println!("{:?}", ret_value);
    println!("Hello, world!");
}

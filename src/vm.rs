#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
use crate::loader::{Class, Field};

pub struct ClassInstance {
	class: Class,
	fields: Vec<Field>
}
impl ClassInstance {
	pub fn new(class: &Class) -> ClassInstance {
		ClassInstance {class: class.clone(), fields: class.fields.to_vec()}
	}
}

#[derive(Clone)]
pub enum JType {
	None,
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Char(u16),
	Float(f32),
	Double(f64),
	ReturnAddress(u32),
	BooleanArray(Rc<RefCell<Vec<i8>>>),
	CharArray(Rc<RefCell<Vec<u16>>>),
	FloatArray(Rc<RefCell<Vec<f32>>>),
	DoubleArray(Rc<RefCell<Vec<f64>>>),
	ByteArray(Rc<RefCell<Vec<i8>>>),
	ShortArray(Rc<RefCell<Vec<i16>>>),
	IntArray(Rc<RefCell<Vec<i32>>>),
	LongArray(Rc<RefCell<Vec<i64>>>),
	ReferenceArray(Rc<RefCell<Vec<JType>>>)
}
impl JType {
	fn byte(&self) -> i8 {
		if let JType::Byte(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn short(&self) -> i16 {
		if let JType::Short(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn int(&self) -> i32 {
		if let JType::Int(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn long(&self) -> i64 {
		if let JType::Long(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn char(&self) -> u16 {
		if let JType::Char(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn float(&self) -> f32 {
		if let JType::Float(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn double(&self) -> f64 {
		if let JType::Double(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn return_address(&self) -> u32 {
		if let JType::ReturnAddress(value) = &self {
			return *value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn boolean_array(self) -> Rc<RefCell<Vec<i8>>> {
		if let JType::BooleanArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn char_array(self) -> Rc<RefCell<Vec<u16>>> {
		if let JType::CharArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn float_array(self) -> Rc<RefCell<Vec<f32>>> {
		if let JType::FloatArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn double_array(self) -> Rc<RefCell<Vec<f64>>> {
		if let JType::DoubleArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn byte_array(self) -> Rc<RefCell<Vec<i8>>> {
		if let JType::ByteArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn short_array(self) -> Rc<RefCell<Vec<i16>>> {
		if let JType::ShortArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn int_array(self) -> Rc<RefCell<Vec<i32>>> {
		if let JType::IntArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn long_array(self) -> Rc<RefCell<Vec<i64>>> {
		if let JType::LongArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
	pub fn reference_array(self) -> Rc<RefCell<Vec<JType>>> {
		if let JType::ReferenceArray(value) = self {
			return value;
		}
		panic!("wrong enum type accessed");
	}
}

pub struct Frame {
	class: ClassInstance,
	locals: Vec<JType>,
	stack: Vec<JType>,
	code: Vec<u8>,
	ip: usize
}
impl Frame {
	pub fn new(class: ClassInstance, method_name: &str, args: &[JType]) -> Frame {
		for method in class.class.methods.iter() {
			if method.name == method_name {
				for attribute in method.attributes.iter() {
					if attribute.name == "Code" {
						let max_stack = u16::from_be_bytes([attribute.info[0], attribute.info[1]]);
						let max_locals = u16::from_be_bytes([attribute.info[2], attribute.info[3]]);

						let code_length = u32::from_be_bytes(attribute.info[4..8].try_into().unwrap());
						let code = attribute.info[8..(8 + code_length) as usize].to_vec();

						let mut locals: Vec<JType> = vec![JType::None; max_locals as usize];
						for i in 0..max_locals {
							if let Some(arg) = args.get(i as usize) {
								locals[i as usize] = arg.clone();
							}
						}

						return Frame {class, locals,
						stack: Vec::with_capacity(max_stack as usize), code, ip: 0}
					}
				}
			}
		}

		panic!();
	}
}

pub struct VM {

}
impl VM {
	pub fn new() -> VM {
		VM {}
	}
	pub fn execute(&self, mut frame: Frame) -> JType {
		loop {
			let op: u8 = frame.code[frame.ip as usize];
			let stack_length = frame.stack.len();
			match op {
				1 => { // aconst_null
					frame.stack.push(JType::None);
				}
				3 => { // iconst_0
					frame.stack.push(JType::Int(0));
				}
				4 => { // iconst_1
					frame.stack.push(JType::Int(1));
				}
				5 => { // iconst_2
					frame.stack.push(JType::Int(2));
				}
				25 => { // aload
					let index = frame.code[frame.ip + 1];
					frame.ip += 1;
					frame.stack.push(frame.locals[index as usize].clone());
				}
				26 => { // iload_0
					frame.stack.push(frame.locals[0].clone());
				}
				27 => { // iload_1
					frame.stack.push(frame.locals[1].clone());
				}
				28 => { // iload_2
					frame.stack.push(frame.locals[2].clone());
				}
				29 => { // iload_3
					frame.stack.push(frame.locals[3].clone());
				}
				42 => { // aload_0
					frame.stack.push(frame.locals[0].clone());
				}
				43 => { // aload_1
					frame.stack.push(frame.locals[1].clone());
				}
				44 => { // aload_2
					frame.stack.push(frame.locals[2].clone());
				}
				45 => { // aload_3
					frame.stack.push(frame.locals[3].clone());
				}
				50 => { // aaload
					let index = frame.stack.pop().unwrap().int();
					let array = frame.stack.pop().unwrap().reference_array();
					let array = array.as_ref().borrow_mut();
					*frame.stack.last_mut().unwrap() = array[index as usize].clone();
				}
				77 => { // astore_2
					let reference = frame.stack.pop().unwrap();
					frame.locals[2] = reference;
				}
				79 => { // iastore
					let value = frame.stack.pop().unwrap().int();
					let index = frame.stack.pop().unwrap().int();
					let array = frame.stack.pop().unwrap().int_array();
					let mut array = array.as_ref().borrow_mut();
					array[index as usize] = value;
				}
				83 => { // aastore
					let value = frame.stack.pop().unwrap();
					let index = frame.stack.pop().unwrap().int();
					let array = frame.stack.pop().unwrap().reference_array();
					let mut array = array.as_ref().borrow_mut();
					array[index as usize] = value;
				}
				89 => { // dup
					frame.stack.push(frame.stack.last().unwrap().clone());
				}
				172 => { // ireturn
					return frame.stack[stack_length - 1].clone();
				}
				176 => { // areturn
					return frame.stack.pop().unwrap();
				}
				188 => { // newarray
					let array_type = frame.code[frame.ip + 1];
					frame.ip += 1;
					let count = frame.stack.last().unwrap().int();

					match array_type {
						4 => { // bool
							*frame.stack.last_mut().unwrap() =
								JType::BooleanArray(Rc::new(
									RefCell::new(
										vec![0; count as usize]
									)));
						}
						5 => { // char
							*frame.stack.last_mut().unwrap() =
								JType::CharArray(Rc::new(
									RefCell::new(
										vec![0; count as usize]
									)));
						}
						6 => { // float
							*frame.stack.last_mut().unwrap() =
								JType::FloatArray(Rc::new(
									RefCell::new(
										vec![0 as f32; count as usize]
									)));
						}
						7 => { // double
							*frame.stack.last_mut().unwrap() =
								JType::DoubleArray(Rc::new(
									RefCell::new(
										vec![0 as f64; count as usize]
									)));
						}
						8 => { // byte
							*frame.stack.last_mut().unwrap() =
								JType::ByteArray(Rc::new(
									RefCell::new(
										vec![0; count as usize]
									)));
						}
						9 => { // short
							*frame.stack.last_mut().unwrap() =
								JType::ShortArray(Rc::new(
									RefCell::new(
										vec![0; count as usize]
									)));
						}
						10 => { // int
							*frame.stack.last_mut().unwrap() =
								JType::IntArray(Rc::new(
									RefCell::new(
										vec![0; count as usize]
									)));
						}
						11 => { // long
							*frame.stack.last_mut().unwrap() =
								JType::LongArray(Rc::new(
									RefCell::new(
										vec![0; count as usize]
									)));
						}
						_ => {}
					}
				}
				_ => panic!("Unsupported instruction {}", op)
			}
			frame.ip += 1;
		}
	}
}
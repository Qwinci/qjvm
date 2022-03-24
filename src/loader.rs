#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Loader {
	reader: BufReader<File>
}
impl Loader {
	fn new(reader: BufReader<File>) -> Loader {
		Loader {reader}
	}

	fn u1(&mut self) -> u8 {
		let mut result: [u8; 1] = [0; 1];
		self.reader.read_exact(&mut result).expect("Failed to read file data.");
		return u8::from_be(result[0]);
	}
	fn u2(&mut self) -> u16 {
		let mut result: [u8; 2] = [0; 2];
		self.reader.read_exact(&mut result).expect("Failed to read file data.");
		return u16::from_be_bytes(result);
	}
	fn u4(&mut self) -> u32 {
		let mut result: [u8; 4] = [0; 4];
		self.reader.read_exact(&mut result).expect("Failed to read file data.");
		return u32::from_be_bytes(result);
	}
	fn u8(&mut self) -> u64 {
		let mut result: [u8; 8] = [0; 8];
		self.reader.read_exact(&mut result).expect("Failed to read file data.");
		return u64::from_be_bytes(result);
	}
	fn bytes(&mut self, count: usize) -> Vec<u8> {
		let mut result = vec![0u8; count];
		self.reader.read_exact(&mut result).expect("Failed to read file data.");
		return result;
	}
}
#[derive(Clone)]
pub enum Data1 {
	None,
	NameIndex(u16),
	ClassIndex(u16),
	StringIndex(u16),
	Int(i32),
	Float(f32),
	Long(i64),
	Double(f64),
	String(String),
	ReferenceKind(u8),
	BootstrapMethodAttrIndex(u16)
}
#[derive(Clone)]
pub enum Data2 {
	None,
	NameAndTypeIndex(u16),
	DescriptorIndex(u16),
	ReferenceIndex(u16)
}

#[derive(Clone)]
pub struct Constant {
	tag: u8,
	pub data1: Data1,
	pub data2: Data2
}
impl Constant {
	fn new(tag: u8) -> Constant {
		Constant {tag, data1: Data1::None, data2: Data2::None}
	}
}

#[derive(Clone)]
pub struct Attribute {
	pub name: String,
	pub info: Vec<u8>
}
impl Attribute {
	fn new(name: String, info: Vec<u8>) -> Attribute {
		Attribute {name, info}
	}
}

#[derive(Clone)]
pub struct Field {
	pub access_flags: u16,
	pub name: String,
	pub descriptor: String,
	pub attributes: Vec<Attribute>
}
impl Field {
	fn new(access_flags: u16, name: String, descriptor: String, attributes: Vec<Attribute>) -> Field {
		Field {access_flags, name, descriptor, attributes}
	}
}

fn parse_constant_pool(loader: &mut Loader) -> Vec<Constant> {
	let count = loader.u2();
	let mut constant_pool: Vec<Constant> = Vec::new();
	let mut i: u16 = 0;
	while i < count - 1 {
		let tag = loader.u1();
		let mut constant = Constant::new(tag);
		match tag {
			1 => { // Utf8
				let length = loader.u2();
				let string = String::from_utf8(loader.bytes(length as usize)).unwrap();
				constant.data1 = Data1::String(string);
			}
			3 => { // Integer
				let bytes = loader.bytes(4);
				let integer = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
				constant.data1 = Data1::Int(integer);
			}
			4 => { // Float
				let bytes = loader.bytes(4);
				let integer = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

				if integer == 0x7f800000 {
					constant.data1 = Data1::Float(f32::INFINITY);
				}
				else if integer == -8388608i32 {
					constant.data1 = Data1::Float(f32::NEG_INFINITY);
				}
				else if integer >= 0x7f800001 {
					constant.data1 = Data1::Float(f32::NAN);
				}
				else if integer >= -8388607i32 && integer <= -1i32 {
					constant.data1 = Data1::Float(f32::NAN);
				}
				else {
					let s: i32 = if integer >> 31 == 0 {1} else {-1};
					let e: i32 = integer >> 23 & 0xFF;
					let m: i32 = if e == 0 {(integer & 0x7FFFFF) << 1}
					else {(integer & 0x7FFFFF) | 0x800000};
					let float: f32 = s as f32 * m as f32 * 2f32.powi(e - 150);
					constant.data1 = Data1::Float(float);
				}
			}
			5 => { // Long
				let high_bytes = loader.u4();
				let low_bytes = loader.u4();
				let integer: i64 = (high_bytes as i64) << 32 + low_bytes as i64;
				constant.data1 = Data1::Long(integer);
				i += 1;
			}
			6 => { // Double
				let high_bytes = loader.u4();
				let low_bytes = loader.u4();
				let integer: i64 = (high_bytes as i64) << 32 + low_bytes as i64;

				if integer == 0x7ff0000000000000 {
					constant.data1 = Data1::Double(f64::INFINITY);
				}
				else if integer == -4503599627370496i64 {
					constant.data1 = Data1::Double(f64::NEG_INFINITY);
				}
				else if integer >= 0x7ff0000000000001 {
					constant.data1 = Data1::Double(f64::NAN);
				}
				else if integer >= -4503599627370495i64 && integer <= -1i64 {
					let s: i32 = if integer >> 63 == 0 {1} else {-1};
					let e: i32 = (integer >> 52 & 0x7FF) as i32;
					let m: i64 = if e == 0 {(integer & 0xfffffffffffff) << 1}
					else {(integer & 0xfffffffffffff) | 0x10000000000000};
					let double: f64 = s as f64 * m as f64 * 2f64.powi(e - 1075);
					constant.data1 = Data1::Double(double);
				}
				i += 1;
			}
			7 => { // Class
				constant.data1 = Data1::NameIndex(loader.u2());
			}
			8 => { // String
				constant.data1 = Data1::StringIndex(loader.u2());
			}
			9 | 10 | 11 => { // FieldRef, MethodRef, InterfaceMethodRef
				constant.data1 = Data1::ClassIndex(loader.u2());
				constant.data2 = Data2::NameAndTypeIndex(loader.u2());
			}
			12 => { // NameAndType
				constant.data1 = Data1::NameIndex(loader.u2());
				constant.data2= Data2::DescriptorIndex(loader.u2());
			}
			15 => { // MethodHandle
				constant.data1 = Data1::ReferenceKind(loader.u1());
				constant.data2 = Data2::ReferenceIndex(loader.u2());
			}
			16 => { // MethodType
				constant.data2 = Data2::DescriptorIndex(loader.u2());
			}
			17 | 18 => { // Dynamic, InvokeDynamic
				constant.data1 = Data1::BootstrapMethodAttrIndex(loader.u2());
				constant.data2 = Data2::NameAndTypeIndex(loader.u2());
			}
			19 => { // Module
				constant.data1 = Data1::NameIndex(loader.u2());
			}
			20 => { // Package
				constant.data1 = Data1::NameIndex(loader.u2());
			}
			_ => panic!("Unsupported tag {}", tag)
		}
		i += 1;
		constant_pool.push(constant);
	}

	return constant_pool;
}

fn get_string_at(constant_pool: &Vec<Constant>, index: u16) -> String {
	let constant = constant_pool.get(index as usize)
		.expect(format!("Invalid index {}", index).as_str());
	if let Data1::NameIndex(index) = &constant.data1 {
		if let Data1::String(string) = &constant_pool[(index - 1) as usize].data1 {
			return string.clone();
		}
	}
	return String::new();
}

fn parse_interfaces(constant_pool: &Vec<Constant>, loader: &mut Loader) -> Vec<String> {
	let count = loader.u2();
	let mut interfaces: Vec<String> = Vec::with_capacity(count as usize);
	for _ in 0..count {
		let name = get_string_at(constant_pool, loader.u2() - 1);
		interfaces.push(name);
	}
	return interfaces;
}

fn parse_attributes(constant_pool: &Vec<Constant>, loader: &mut Loader) -> Vec<Attribute> {
	let count = loader.u2();
	let mut attributes: Vec<Attribute> = Vec::with_capacity(count as usize);

	for _ in 0..count {
		let name: String;
		if let Data1::String(string) = &constant_pool[(loader.u2() - 1) as usize].data1 {
			name = string.clone();
		}
		else {
			panic!("Attribute name was not found.");
		}

		let length = loader.u4();
		let info = loader.bytes(length as usize);
		attributes.push(Attribute::new(name, info));
	}

	return attributes;
}

fn parse_fields(constant_pool: &Vec<Constant>, loader: &mut Loader) -> Vec<Field> {
	let count = loader.u2();
	let mut fields: Vec<Field> = Vec::with_capacity(count as usize);
	for _ in 0..count {
		let access_flags = loader.u2();
		let name: String;
		let descriptor: String;

		if let Data1::String(string) = &constant_pool[(loader.u2() - 1) as usize].data1 {
			name = string.clone();
		}
		else {
			panic!("Field name not found.");
		}

		if let Data1::String(string) = &constant_pool[(loader.u2() - 1) as usize].data1 {
			descriptor = string.clone();
		}
		else {
			panic!("Descriptor for field {} was not found.", name);
		}

		let attributes = parse_attributes(constant_pool, loader);
		fields.push(Field::new(access_flags, name, descriptor, attributes));
	}

	return fields;
}

#[derive(Clone)]
pub struct Class {
	pub initialized: bool,
	pub major_version: u16,
	pub minor_version: u16,
	pub constant_pool: Vec<Constant>,
	pub access_flags: u16,
	pub this_class: String,
	pub super_class: String,
	pub interfaces: Vec<String>,
	pub fields: Vec<Field>,
	pub methods: Vec<Field>,
	pub attributes: Vec<Attribute>
}
impl Class {
	pub fn new(major_version: u16, minor_version: u16,
	           constant_pool: Vec<Constant>, access_flags: u16,
			   this_class: String, super_class: String,
			   interfaces: Vec<String>, fields: Vec<Field>,
			   methods: Vec<Field>, attributes: Vec<Attribute>) -> Class {
		Class {initialized: false, major_version, minor_version, constant_pool, access_flags,
			this_class, super_class, interfaces, fields, methods, attributes}
	}
}

pub fn load_class(path: &str) -> Class {
	let file = File::open(path).expect("Failed to open file.");
	let mut loader = Loader::new(BufReader::new(file));

	let magic = loader.u4();
	if magic == 0xCAFEBABE {
		println!("Valid Java identifier found.");
	}

	let minor_version = loader.u2();
	let major_version = loader.u2();
	println!("Version {}.{}", major_version, minor_version);

	let constant_pool = parse_constant_pool(&mut loader);

	let access_flags = loader.u2();
	let this_class = get_string_at(&constant_pool, loader.u2() - 1);
	let super_class = get_string_at(&constant_pool, loader.u2() - 1);
	let interfaces = parse_interfaces(&constant_pool, &mut loader);
	let fields = parse_fields(&constant_pool, &mut loader);
	let methods = parse_fields(&constant_pool, &mut loader);
	let attributes = parse_attributes(&constant_pool, &mut loader);

	return Class::new(major_version, minor_version, constant_pool,
	                  access_flags, this_class,
	                  super_class, interfaces, fields,
	                  methods, attributes);
}
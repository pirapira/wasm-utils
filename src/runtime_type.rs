use parity_wasm::{elements, builder};
use self::elements::{ Module, GlobalEntry, External, ExportEntry, GlobalType, ValueType, InitExpr, Opcode, Internal };
use byteorder::{ LittleEndian, ByteOrder };

pub fn inject_runtime_type(module: Module, runtime_type: &[u8], runtime_version: u32) -> Module {
	let runtime_type: u32 = LittleEndian::read_u32(&runtime_type);
	let globals_count: u32 = match module.global_section() {
		Some(ref section) => section.entries().len() as u32,
		None => 0
	};
	let imported_globals_count: u32 = match module.import_section() {
		Some(ref section) => section.entries().iter().filter(|e| match *e.external() {
			External::Global(ref _a) => true,
			_ => false
		}).count() as u32,
		None => 0
	};
	let total_globals_count: u32 = globals_count + imported_globals_count;

	builder::from_module(module)
		.with_global(GlobalEntry::new(GlobalType::new(ValueType::I32, false), InitExpr::new(vec![Opcode::I32Const(runtime_type as i32), Opcode::End])))
		.with_export(ExportEntry::new("RUNTIME_TYPE".into(), Internal::Global(total_globals_count)))
		.with_global(GlobalEntry::new(GlobalType::new(ValueType::I32, false), InitExpr::new(vec![Opcode::I32Const(runtime_version as i32), Opcode::End])))
		.with_export(ExportEntry::new("RUNTIME_VERSION".into(), Internal::Global(total_globals_count + 1)))
	.build()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_injects() {
		let mut module = builder::module()
			.with_global(GlobalEntry::new(GlobalType::new(ValueType::I32, false), InitExpr::new(vec![Opcode::I32Const(42 as i32)])))
		.build();
		module = inject_runtime_type(module, b"emcc", 1);
		let global_section = module.global_section().expect("Global section expected");
		assert_eq!(3, global_section.entries().len());
		let export_section = module.export_section().expect("Export section expected");
		assert!(export_section.entries().iter().find(|e| e.field() == "RUNTIME_TYPE" ).is_some());
		assert!(export_section.entries().iter().find(|e| e.field() == "RUNTIME_VERSION" ).is_some());
	}
}

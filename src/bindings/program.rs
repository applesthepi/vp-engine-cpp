use self::{cxx_program::{ProgramContext, TickResult}, handler::ProgramHandler};

pub static mut PROGRAM_HANDLER: Option<ProgramHandler> = None;
pub const PROGRAM_INIT_ERROR: &'static str = "must be initialized before run";

pub mod handler;
pub mod custom;
pub mod context;

#[cxx::bridge(namespace = "vpe")]
pub mod cxx_program {
	pub struct ProgramContext {
		pub name: String,
	}
	#[derive(Copy, Clone)]
	pub enum TickResult {
		CONTINUE,
		RENDER,
		EXIT,
	}
	extern "Rust" {
		fn initialize(
			program_context: Box<ProgramContext>,
		);
		fn tick_events() -> TickResult;
		fn register_pipeline(pipeline: String) -> u32;
		fn register_object(object: String) -> u32;
		fn create_object_instance(pipeline_id: u32, object_id: u32) -> u32;
		unsafe fn modify_object_block(pipeline_id: u32, object_id: u32, data: *mut u8);
	}
}

pub fn initialize(
	program_context: Box<ProgramContext>,
) {
	let program_handler = ProgramHandler::new(
		program_context,
	);
	unsafe { PROGRAM_HANDLER = Some(program_handler); }
}

pub fn tick_events(
) -> TickResult { unsafe {
	let program_handler = PROGRAM_HANDLER.as_mut().expect(PROGRAM_INIT_ERROR);
	TickResult { repr: program_handler.tick_events() as u8 }
}}

pub fn register_pipeline(
	pipeline: String,
) -> u32 {
	0
}

fn register_object(
	object: String,
) -> u32 {
	0
}

fn create_object_instance(
	pipeline_id: u32,
	object_id: u32,
) -> u32 {
	0
}

unsafe fn modify_object_block(
	pipeline_id: u32,
	object_id: u32,
	data: *mut u8,
) {

}
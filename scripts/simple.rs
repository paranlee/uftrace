use std::os::raw::c_char;
use std::ffi::CStr;

// Information passed during initialization
#[repr(C)] // Ensure C-compatible layout
pub struct UftraceScriptInfo {
	pub api_version: i32,
	pub name: *const c_char, // Use raw pointer for name and version
	pub version: *const c_char,
	pub record: bool,
	pub cmds: *const c_char,  // Use raw pointer for commands string
}

// Base context information passed to script
#[repr(C)] // Ensure C-compatible layout
pub struct UftraceScriptBaseCtx {
	pub tid: i32,
	pub depth: i32,
	pub timestamp: u64,
	pub duration: u64,
	pub address: usize, // Use usize for address on most platforms
	pub name: *const c_char, // Use raw pointer for name
}

#[no_mangle]
pub extern "C" fn uftrace_begin(info: &UftraceScriptInfo) {
	println!("uftrace_begin called {}", unsafe { CStr::from_ptr(info.name) }.to_string_lossy());
}

#[no_mangle]
pub extern "C" fn uftrace_entry(ctx: &UftraceScriptBaseCtx) {
	println!("uftrace_entry called with name: {}", unsafe { CStr::from_ptr(ctx.name) }.to_string_lossy());
}

#[no_mangle]
pub extern "C" fn uftrace_exit(ctx: &UftraceScriptBaseCtx) {
	println!("uftrace_exit called with name: {}", unsafe { CStr::from_ptr(ctx.name) }.to_string_lossy());
}

#[no_mangle]
pub extern "C" fn uftrace_end() {
	println!("uftrace_end called");
}
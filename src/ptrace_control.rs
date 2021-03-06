use std::ptr;
use nix::sys::signal::Signal;
use nix::sys::ptrace::{ptrace, ptrace_getevent, ptrace_setoptions};
use nix::sys::ptrace::ptrace::*;
use nix::libc::{c_void, c_long};
use nix::unistd::Pid;
use nix::Result;

const RIP: u8 = 128;


pub fn trace_children(pid: Pid) -> Result<()> {
    //TODO need to check support.
    let options: PtraceOptions = PTRACE_O_TRACESYSGOOD | PTRACE_O_TRACEEXEC | PTRACE_O_TRACEEXIT |
        PTRACE_O_TRACECLONE | PTRACE_O_TRACEFORK | PTRACE_O_TRACEVFORK;
    ptrace_setoptions(pid, options)
}

pub fn detach_child(pid: Pid) -> Result<c_long> {
    ptrace(PTRACE_DETACH, pid, ptr::null_mut(), ptr::null_mut())
}

pub fn continue_exec(pid: Pid, sig: Option<Signal>) -> Result<c_long> {
    match sig {
        Some(s) => ptrace(PTRACE_CONT, pid, ptr::null_mut(), (s as i32) as * mut c_void),
        None => ptrace(PTRACE_CONT, pid, ptr::null_mut(), ptr::null_mut()),
    }
}

pub fn single_step(pid: Pid) -> Result<c_long> {
    ptrace(PTRACE_SINGLESTEP, pid, ptr::null_mut(), ptr::null_mut())
}

pub fn read_address(pid: Pid, address:u64) -> Result<c_long> {
    ptrace(PTRACE_PEEKDATA, pid, address as * mut c_void, ptr::null_mut())
}

pub fn write_to_address(pid: Pid,
                        address: u64, 
                        data: i64) -> Result<c_long> {
    ptrace(PTRACE_POKEDATA, pid, address as * mut c_void, data as * mut c_void)
}

pub fn current_instruction_pointer(pid: Pid) -> Result<c_long> {
    ptrace(PTRACE_PEEKUSER, pid, RIP as * mut c_void, ptr::null_mut())
}

pub fn set_instruction_pointer(pid: Pid, pc: u64) -> Result<c_long> {
    ptrace(PTRACE_POKEUSER, pid, RIP as * mut c_void, pc as * mut c_void)
}

pub fn request_trace() -> Result<c_long> {
    ptrace(PTRACE_TRACEME, Pid::from_raw(0), ptr::null_mut(), ptr::null_mut())
}

pub fn get_event_data(pid: Pid) -> Result<c_long> {
    ptrace_getevent(pid)
}


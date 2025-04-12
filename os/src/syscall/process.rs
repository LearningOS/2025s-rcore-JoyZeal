//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
    task::TASK_MANAGER,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    match _trace_request {
        0 => {
            
            let ptr = _id as *const u8;
            unsafe { ptr.read_volatile() as isize }
        }
        1 => {
            
            let ptr = _id as *mut u8;
            let value = _data as u8;
            unsafe { ptr.write_volatile(value) };
            0
        }
        2 => {
            
            let mut task_manager = TASK_MANAGER.exclusive_access();
            let current_task_index = task_manager.current_task;
            let current_task = &mut task_manager.tasks[current_task_index];
            let count = current_task.syscall_counts.entry(_id).or_insert(0);
            
            *count as isize
        }
        _ => -1,
    }
}

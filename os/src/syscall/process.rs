//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::translated_byte_buffer, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_count, get_mmap,get_munmap, get_now, suspend_current_and_run_next, TaskStatus
    }, timer:: {get_time_ms, get_time_us}
};
use core:: mem;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    if ts.is_null(){
        return -1;
    }
    let us = get_time_us();
    let size_of_timeval = mem::size_of::<TimeVal>();
    let src =ts  as *const u8;
    let buffers = translated_byte_buffer(current_user_token(), src , size_of_timeval);
    let time_value = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let  time_ptr=&time_value as *const _ as *const u8;
    let mut  offset=0;
    for buffer in buffers {
        buffer.copy_from_slice(unsafe {
            core::slice::from_raw_parts(
                time_ptr.add(offset),
                buffer.len()
            )
        });
        offset+=buffer.len();
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    if ti.is_null(){
        return -1;
    }
    let now_time=get_time_ms();
    let start_time=get_now() as usize;
    let  syscall_count=get_count();
    let size_of_taskval = mem::size_of::<TaskInfo>();
    let src =ti  as *const u8;
    let buffers = translated_byte_buffer(current_user_token(), src , size_of_taskval);

    let task_value = TaskInfo {
        time:now_time-start_time,
        status:TaskStatus::Running,
        syscall_times:syscall_count,

    };
    let  task_ptr=&task_value as *const _ as *const u8;
    let mut  offset=0;
    for buffer in buffers {
        buffer.copy_from_slice(unsafe {
            core::slice::from_raw_parts(
                task_ptr.add(offset),
                buffer.len()
            )
        });
        offset+=buffer.len();
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if start%PAGE_SIZE!=0{
        return -1;
    }
    if port & !0x7 != 0{
        return -1;
    }
    if port & 0x7 == 0{
        return -1;
    }
    get_mmap(start,len,port)
    
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if start%PAGE_SIZE!=0{
        return -1;
    }
    get_munmap(start,len)
    
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

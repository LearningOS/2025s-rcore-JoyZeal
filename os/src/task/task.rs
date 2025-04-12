//! Types related to task management

use super::TaskContext;
use alloc::collections::BTreeMap;

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
///
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// The task control block (TCB) of a task.
#[derive(Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    ///add 12 15 18
    pub syscall_counts: BTreeMap<usize,usize>,
}

impl TaskControlBlock {
///
    pub fn new(kstack_ptr: usize) -> Self {
        Self {
            task_status: TaskStatus::Ready,
            task_cx: TaskContext::goto_restore(kstack_ptr),
            syscall_counts: BTreeMap::new(),
        }
    }
}

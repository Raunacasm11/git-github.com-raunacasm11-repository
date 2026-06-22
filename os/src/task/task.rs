use super::{TaskContext, __switch};
use crate::trap::TrapContext;
use crate::config::CLOCK_FREQ;
use core::cell::RefCell;
use alloc::vec::Vec;
use alloc::sync::Arc;
use lazy_static::lazy_static;

pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub kstack: usize,
}

impl TaskControlBlock {
    pub fn new(entry_addr: usize, kstack_ptr: usize) -> Self {
        let mut task_cx = TaskContext::goto_restore(kstack_ptr);
        unsafe {
            let trap_cx = (kstack_ptr - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
            *trap_cx = TrapContext::app_init_context(entry_addr, kstack_ptr);
        }
        Self {
            task_status: TaskStatus::Ready,
            task_cx,
            kstack: kstack_ptr,
        }
    }
}

pub enum TaskStatus {
    Ready,
    Running,
    Exited,
}

pub struct TaskManager {
    tasks: Vec<Arc<TaskControlBlock>>,
    current_task: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_task: 0,
        }
    }
    
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.tasks.push(task);
    }
    
    pub fn run_next_task(&mut self) {
        if self.tasks.is_empty() {
            panic!("No tasks to run!");
        }
        
        // Find next ready task
        let next_task = loop {
            self.current_task = (self.current_task + 1) % self.tasks.len();
            if self.tasks[self.current_task].task_status == TaskStatus::Ready {
                break self.tasks[self.current_task].clone();
            }
        };
        
        let current_task = self.tasks[self.current_task].clone();
        if current_task.task_status == TaskStatus::Running {
            current_task.task_status = TaskStatus::Ready;
        }
        next_task.task_status = TaskStatus::Running;
        
        unsafe {
            __switch(&mut current_task.task_cx as *mut TaskContext, &next_task.task_cx as *const TaskContext);
        }
    }
}

lazy_static! {
    static ref TASK_MANAGER: RefCell<TaskManager> = RefCell::new(TaskManager::new());
}

pub fn suspend_current_and_run_next() {
    let task_manager = TASK_MANAGER.borrow_mut();
    drop(task_manager);
    // Will implement fully later
}

pub fn exit_current_and_run_next() {
    let task_manager = TASK_MANAGER.borrow_mut();
    drop(task_manager);
    // Will implement fully later
}

pub fn sys_yield() {
    suspend_current_and_run_next();
}

pub fn init() {
    // Will initialize first task
}
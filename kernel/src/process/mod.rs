#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TaskState {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Task {
    pub id: u64,
    pub state: TaskState,
    pub stack: [u8; 16384],
}

pub struct Scheduler {
    pub tasks: [Option<Task>; 32],
    pub current: Option<usize>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            tasks: [None; 32],
            current: None,
        }
    }
}

static mut SCHEDULER: Scheduler = Scheduler::new();

pub fn init() {
    unsafe {
        SCHEDULER = Scheduler::new();
    }
    let task0 = Task {
        id: 0,
        state: TaskState {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, cs: 0, rflags: 0, rsp: 0, ss: 0,
        },
        stack: [0u8; 16384],
    };
    unsafe {
        SCHEDULER.tasks[0] = Some(task0);
        SCHEDULER.current = Some(0);
    }
}
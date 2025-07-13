use ilhook::x64::*;
pub struct Interceptor {
    module_base: usize,
    active_hooks: Vec<HookPoint>,
}

pub struct Context {
    registers: *mut Registers,
}

pub type AttachCallback = fn(ctx: &mut Context);

impl Interceptor {
    pub fn new(base: usize) -> Self {
        Self {
            module_base: base,
            active_hooks: Vec::new(),
        }
    }

    pub fn attach(&mut self, offset: usize, callback: AttachCallback) {
        let hooker = Hooker::new(
            self.module_base + offset,
            HookType::JmpBack(ilhook_trampoline),
            CallbackOption::None,
            callback as usize,
            HookFlags::empty(),
        );

        unsafe {
            if let Ok(hp) = hooker.hook() {
                self.active_hooks.push(hp);
            } else {
                eprintln!("failed to attach to 0x{offset:X}");
            }
        }
    }
}

unsafe extern "win64" fn ilhook_trampoline(regs: *mut Registers, cb_ptr: usize) {
    let f: AttachCallback = std::mem::transmute(cb_ptr);
    f(&mut Context { registers: regs });
}

impl Context {
    pub fn registers(&mut self) -> &mut Registers {
        unsafe { &mut *self.registers }
    }
}

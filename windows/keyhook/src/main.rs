// not work yet

use std::cell::OnceCell;

use winsafe::{
    co::{WH, WM},
    prelude::{user_Hhook, Handle as _},
    HHOOK, HINSTANCE,
};

#[derive(Clone, Copy)]
struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
}

thread_local! {
    pub static HOOK: OnceCell<HHOOK> = const { OnceCell::new() };
}

unsafe fn get_code(lpdata: isize) -> u32 {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);

    kb.vk_code
}

extern "system" fn hook(code: i32, w_param: usize, l_param: isize) -> isize {
    dbg!(code);

    if code >= 0 {
        match unsafe { WM::from_raw(w_param as u32) } {
            WM::KEYDOWN | WM::SYSKEYDOWN => {
                let vkcode = unsafe { get_code(l_param) } as u16;
                dbg!(vkcode);
            }
            WM::KEYUP | WM::SYSKEYUP => {
                let vkcode = unsafe { get_code(l_param) } as u16;
                dbg!(vkcode);
            }
            _ => (),
        }
    }

    HOOK.with(|h| {
        let hook = h.get().unwrap();
        // hook.CallNextHookEx(unsafe { WH::from_raw(code) }, w_param, l_param)
        return 0;
    })
}

fn main() {
    std::thread::spawn(|| {
        let hook = HHOOK::SetWindowsHookEx(WH::KEYBOARD_LL, hook, Some(&HINSTANCE::NULL), Some(0))
            .unwrap();
        HOOK.with(|h| {
            h.set(hook).unwrap();
        });
    });

    loop {
        std::thread::yield_now();
    }
}

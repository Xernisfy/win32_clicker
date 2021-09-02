mod bindings {
    windows::include_bindings!();
}

// bindings from build script
use bindings::Windows::Win32::{
    Foundation::POINT,
    UI::{
        KeyboardAndMouseInput::{SendInput, INPUT, INPUT_TYPE, MOUSEINPUT},
        WindowsAndMessaging::{GetCursorPos, SetCursorPos},
    },
};
// unions and constants
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{
    INPUT_0, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
};

use std::{convert::TryInto, env, mem::size_of, num::ParseIntError};

const MOUSEDOWN: INPUT = INPUT {
    r#type: INPUT_TYPE(0),
    Anonymous: INPUT_0 {
        mi: MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: MOUSEEVENTF_LEFTDOWN,
            time: 0,
            dwExtraInfo: 0,
        },
    },
};
const MOUSEUP: INPUT = INPUT {
    r#type: INPUT_TYPE(0),
    Anonymous: INPUT_0 {
        mi: MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: MOUSEEVENTF_LEFTUP,
            time: 0,
            dwExtraInfo: 0,
        },
    },
};

unsafe fn click() {
    let mut pinputs: [INPUT; 2] = [MOUSEDOWN, MOUSEUP];
    let cinputs: u32 = pinputs.len().try_into().unwrap();
    let cbsize: i32 = size_of::<INPUT>().try_into().unwrap();
    SendInput(cinputs, pinputs.as_mut_ptr(), cbsize);
}

fn main() -> windows::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut oldlppoint: POINT = POINT::default();
    let mut lppoint: POINT = POINT::default();
    match args.len() {
        1 => unsafe {
            click();
        },
        3 => {
            unsafe {
                GetCursorPos(&mut oldlppoint);
            }
            let x: Result<i32, ParseIntError> = args[1].parse();
            if x.is_err() {
                return Err(windows::Error::new(
                    windows::HRESULT(0x80070057),
                    "argument 1 must be an integer",
                ));
            }
            lppoint.x = x.unwrap();
            let y: Result<i32, ParseIntError> = args[2].parse();
            if y.is_err() {
                return Err(windows::Error::new(
                    windows::HRESULT(0x80070057),
                    "argument 2 must be an integer",
                ));
            }
            lppoint.y = y.unwrap();
            unsafe {
                SetCursorPos(lppoint.x, lppoint.y);
                click();
                SetCursorPos(oldlppoint.x, oldlppoint.y);
            }
        }
        _ => {
            return Err(windows::Error::new(
                windows::HRESULT(0x80070057),
                "expected 0 or 2 arguments",
            ));
        }
    }
    Ok(())
}

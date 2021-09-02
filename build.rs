fn main() {
    windows::build! {
        Windows::Win32::{
            Foundation::POINT,
            UI::{
                KeyboardAndMouseInput::{SendInput, INPUT, INPUT_TYPE, MOUSEINPUT},
                WindowsAndMessaging::{GetCursorPos, SetCursorPos},
            },
        }
    };
}

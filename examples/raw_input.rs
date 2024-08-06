use std::mem;

#[cfg(target_os = "macos")]
fn main() {
    use core_graphics::event::{CGEvent, CGEventField, CGEventType};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

    loop {
        let event = CGEvent::new(CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap());
        if let Ok(event) = event {
            //if let CGEventType::MouseMoved = event.get_type() {
            println!(
                "{} {}",
                event.get_double_value_field(40),
                event.get_double_value_field(5)
            );
            //}
        }
    }
}

#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::*, UI::Input::*,
        UI::WindowsAndMessaging::*,
    },
};

#[cfg(target_os = "windows")]
fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        let window_class = w!("window");

        let class = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),

            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,

            lpszClassName: window_class,

            ..Default::default()
        };
        let atom = RegisterClassExW(&class);
        debug_assert!(atom != 0);

        let hwndMain = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("window"),
            w!("Test"),
            WS_OVERLAPPEDWINDOW | WS_HSCROLL | WS_VSCROLL,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            Option::None,
            Option::None,
            instance,
            Option::None,
        )?;

        let mouse = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x02,
            dwFlags: RAWINPUTDEVICE_FLAGS::default(),
            hwndTarget: hwndMain,
        };

        let res = RegisterRawInputDevices(&[mouse], mem::size_of::<RAWINPUTDEVICE>() as u32);
        if let Ok(_) = res {
            println!("ok");
        }

        let _ = ShowWindow(hwndMain, SW_SHOWDEFAULT);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        Ok(())
    }
}

#[cfg(target_os = "windows")]
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                _ = ValidateRect(window, None);
                LRESULT(0)
            }
            WM_INPUT => {
                println!("WM_INPUT");
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}

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
fn main() {
    use windows::Win32::UI::Input;
}

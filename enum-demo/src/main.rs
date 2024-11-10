// Define an enum
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i32, y: i32 }
}

// Function that uses the enum
fn handle_event(event: WebEvent) {

    // Exhaustive matches, all enum values must be covered,
    // or use a catch-all pattern(_)
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::KeyPress(key) => println!("Key pressed: {}", key),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
        WebEvent::PageUnload => println!("Page unloaded"),
        _ => println!("Unknown Event."),
    }
}

enum Color {
    RGB(u8, u8, u8),
    HSV(u8, u8, u8),
}

impl Color {
    fn to_rgb_string(&self) -> String {
        match self {
            Color::RGB(r, g, b) => format!("RGB: ({}, {}, {})", r, g, b),
            Color::HSV(h, s, v) => format!("HSV: ({}, {}, {})", h, s, v),
        }
    }
}

// Using the enum
fn main() {
    let load_event = WebEvent::PageLoad;
    let key_event = WebEvent::KeyPress('x');
    let click_event = WebEvent::Click { x: 10, y: 20 };

    handle_event(load_event);
    handle_event(key_event);
    handle_event(click_event);

    let color1 = Color::RGB(255, 0, 0);
    println!("{}", color1.to_rgb_string());
}
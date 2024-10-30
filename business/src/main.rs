use device_query::{DeviceEvents, DeviceQuery, DeviceState};

fn main() {
    let device_state = DeviceState::new();
    loop {
        let mouse_state = device_state.get_mouse();
        println!("{:?}", mouse_state.coords);
    }
}

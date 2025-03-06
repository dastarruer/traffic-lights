#[derive(Debug)]
enum TrafficLight {
    Green,
    Yellow, 
    Red,
}

fn main() {
    let current_light = TrafficLight::Green;
    println!("Current light: {:?}", current_light);
}

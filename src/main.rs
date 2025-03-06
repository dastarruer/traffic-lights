#[derive(Debug)]
enum TrafficLight {
    Green,
    Yellow, 
    Red,
}

impl TrafficLight {
    fn next(&self) -> Self {
        match self {
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Red => TrafficLight::Green,
        }
    }
}

fn main() {
    let current_light = TrafficLight::Green;
    println!("Current light: {:?}", current_light);
    println!("Next light: {:?}", current_light.next())
}

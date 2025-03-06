use std::thread::sleep;
use std::time::Duration;

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

    fn duration(&self) -> u64 {
        match self {
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Red => 25,
        }
    }
}

fn main() {
    let mut light = TrafficLight::Red;

    loop {
        println!("Current light: {:?}", light);
        println!("Time left: {}", light.duration());
        sleep(Duration::from_secs(light.duration()));
        light = light.next(); 
    }
}

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
    let mut light: TrafficLight = TrafficLight::Red;
    let mut time_passed_secs: u64 = 0;
    let mut time_left_secs: u64;

    loop {
        clear_screen();

        time_left_secs = light.duration() - time_passed_secs;

        println!("Current light: {:?}", light);
        println!("Time left: {}", time_left_secs);

        sleep(Duration::from_secs(1));
        time_passed_secs += 1;

        if time_left_secs == 0 {
            light = light.next();
            time_passed_secs = 0;
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

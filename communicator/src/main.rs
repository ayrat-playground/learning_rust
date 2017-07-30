extern crate communicator;
use communicator::a::series::of::nested_modules;
use communicator::TrafficLight::*;

fn main() {
    communicator::client::connect();

    nested_modules();

    let red = Red;
}

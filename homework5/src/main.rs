use std::ops::Mul;
// 1-------------------------------
#[derive(Debug)]
enum TrafficLight{
    Red,
    Green,
    Yellow,
}
trait Light {
    fn time(&self) -> u32;
}
impl Light for TrafficLight{
    fn time(&self) -> u32{
        return match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn homework1(){
    println!("Traffic Light {:?} duration time is {}", TrafficLight::Red, TrafficLight::Red.time());
    println!("Traffic Light {:?} duration time is {}", TrafficLight::Yellow, TrafficLight::Yellow.time());
    println!("Traffic Light {:?} duration time is {}", TrafficLight::Green, TrafficLight::Green.time());
}

// 2-------------------------------
fn sum_u32_set(t: &[u32]) -> Option<u32>{
    if t.len() > 0{
        let mut sum:u32 = 0;
        for i in t.iter(){
            match sum.checked_add(*i){
                Some(s) => sum = s,
                None => return None,
            }
        }
        Some(sum)
    }else{
        None
    }
}

fn homework2(){
    let u32_set = vec![1,2,3,4,5,6,7,8];
    println!("sume is {}", sum_u32_set(&u32_set).unwrap_or(0));
}

// 3-------------------------------
pub trait Area{
    type Output;
    fn get_area(&self) -> Self::Output;
}

struct Square<T>{
    side: T,
}
struct Circle<T>{
    r: T,
}
struct Triangle<T>{
    bottom: T,
    hight: T,
}
impl <T: Mul<Output = T> + Copy> Area for Square<T>{
    type Output = T;
    fn get_area(&self) ->Self::Output{
        self.side * self.side
    }
}
impl <T: Mul<Output = T> + Copy + Into<f64>> Area for Circle<T>{
    type Output = f64;
    fn get_area(&self) -> Self::Output{
        (self.r * self.r).into() * std::f64::consts::PI
    }
}

impl <T: Mul<Output = T> + Copy + Into<f64>> Area for Triangle<T>{
    type Output = f64;
    fn get_area(&self) -> Self::Output{
        (self.bottom * self.hight).into() * 0.5
    }
}

fn homework3(){
    let square = Square{
        side: 10
    };
    println!("Square area is {}", square.get_area());

    let circle = Circle{
        r: 10
    };
    println!("Circle area is {}", circle.get_area());

    let tri = Triangle{
        bottom: 10,
        hight: 10,
    };
    println!("Triangle area is {}", tri.get_area());
}
fn main() {
    homework1();
    homework2();
    homework3();
}

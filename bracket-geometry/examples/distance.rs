use bracket_geometry::prelude::*;

fn main() {
    let pt1 = Point::new(0,0);
    let pt2 = Point::new(10,20);
    println!("Given the two points:");
    println!("{:#?}\n{:#?}", pt1, pt2);
    println!("");
    println!("Pythagoras Distance: {}", DistanceAlg::Pythagoras.distance2d(pt1, pt2) );
    println!("Pythagoras Squared Distance: {}", DistanceAlg::PythagorasSquared.distance2d(pt1, pt2) );
    println!("Manhattan Distance: {}", DistanceAlg::Manhattan.distance2d(pt1, pt2) );
    println!("Chebyshev Distance: {}", DistanceAlg::Chebyshev.distance2d(pt1, pt2) );
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    FromSouth,
    FromNorth,
    FromWest,
    FromEast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Green,
    Red,
}

/*
The TrafficLight struct represents a traffic light with a single attribute:
color: an instance of the Light enum, which can take on values Green or Red.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficLight {
    pub color: Light,
}

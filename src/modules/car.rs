extern crate rand;

use super::constants::*;
use super::se_base::*;

use rand::Rng;
use sdl2::pixels::Color;

/*
The Car struct represents a car object with the following fields:

x and y: the car's position coordinates
color: the car's color
direction: the car's direction (Left, Right, or Straight)
side: the side of the road the car is coming from (FromEast, FromNorth, etc.)
velocity: the car's speed
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub direction: Direction,
    pub side: Side,
    pub velocity: i32,
}

impl Car {
    /*
    Constructor function that creates a new instance of the Car struct. It takes a side parameter of type Side (an enum representing the direction a car is coming from) and returns a Car object with randomly generated attributes.

    It generates a random number between 0 and 2, which determines the car's direction (left, straight, or right) and color.
    It generates a random velocity between MIN_VELOCITY and MAX_VELOCITY.
    It sets the car's initial position (x and y coordinates) and returns a new Car object with the generated attributes.
    */
    pub fn new(side: Side) -> Car {
        let mut rng = rand::rng();
        let random_number = rng.random_range(0..3);
        let velocity = rng.random_range(MIN_VELOCITY..MAX_VELOCITY);
        let direction: Direction;
        let color: Color;
        match random_number {
            0 => {
                direction = Direction::Left;
                color = CAR_COLOR_LEFT;
            }
            1 => {
                direction = Direction::Straight;
                color = CAR_COLOR_STRAIGHT;
            }
            _ => {
                direction = Direction::Right;
                color = CAR_COLOR_RIGHT;
            }
        }
        match side {
            Side::FromEast => {
                let x = 0;
                let y = OUTPUT_HEIGHT / 2;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
            Side::FromNorth => {
                let x = OUTPUT_WIDTH / 2 - CAR_WIDTH;
                let y = 0;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
            Side::FromSouth => {
                let x = OUTPUT_WIDTH / 2;
                let y = OUTPUT_HEIGHT - CAR_HEIGHT;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
            Side::FromWest => {
                let x = OUTPUT_WIDTH - CAR_WIDTH;
                let y = OUTPUT_HEIGHT / 2 - CAR_HEIGHT;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
        }
    }

    /*
    Generates a random Car object coming from one of four directions (East, North, South, or West) with randomly assigned attributes (direction, color, velocity, etc.) using the Car::new constructor.
    */
    pub fn random_car() -> Car {
        let mut rng = rand::rng();
        let random_number = rng.random_range(0..4);
        match random_number {
            0 => Car::new(Side::FromEast),
            1 => Car::new(Side::FromNorth),
            2 => Car::new(Side::FromSouth),
            _ => Car::new(Side::FromWest),
        }
    }

    /*
    It updates the position of a car based on its current side, direction, velocity, and the state of a traffic light (feu).

    The function uses a match statement to determine the car's side and then applies different movement rules based on that side and the car's direction. The movement rules take into account the car's velocity, the traffic light's color, and the car's position relative to the center of the screen.

    In general, the function allows the car to move forward if the traffic light is green, and stops or redirects the car if the light is red, depending on the car's direction and position.
    */
    pub fn moove(&mut self, feu: TrafficLight) {
        match self.side {
            Side::FromEast => {
                if self.x + self.velocity < OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH {
                    self.x += self.velocity;
                } else if self.x + self.velocity < OUTPUT_WIDTH / 2 - CAR_WIDTH {
                    if feu.color == Light::Green {
                        self.x += self.velocity;
                    } else {
                        self.x = OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.x = OUTPUT_WIDTH / 2;
                            self.y -= self.velocity;
                        }
                        Direction::Right => {
                            self.x = OUTPUT_WIDTH / 2 - CAR_WIDTH;
                            self.y += self.velocity;
                        }
                        Direction::Straight => {
                            self.x += self.velocity;
                        }
                    }
                }
            }
            Side::FromNorth => {
                if self.y + self.velocity < OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT {
                    self.y += self.velocity;
                } else if self.y + self.velocity < OUTPUT_HEIGHT / 2 - CAR_HEIGHT {
                    if feu.color == Light::Green {
                        self.y += self.velocity;
                    } else {
                        self.y = OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.y = OUTPUT_HEIGHT / 2;
                            self.x += self.velocity;
                        }
                        Direction::Right => {
                            self.y = OUTPUT_HEIGHT / 2 - CAR_HEIGHT;
                            self.x -= self.velocity;
                        }
                        Direction::Straight => {
                            self.y += self.velocity;
                        }
                    }
                }
            }
            Side::FromWest => {
                if self.x - self.velocity > OUTPUT_WIDTH / 2 + 2 * CAR_WIDTH {
                    self.x -= self.velocity;
                } else if self.x - self.velocity > OUTPUT_WIDTH / 2 {
                    if feu.color == Light::Green {
                        self.x -= self.velocity;
                    } else {
                        self.x = OUTPUT_WIDTH / 2 + CAR_WIDTH;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.x = OUTPUT_WIDTH / 2 - CAR_WIDTH;
                            self.y += self.velocity;
                        }
                        Direction::Right => {
                            self.x = OUTPUT_WIDTH / 2;
                            self.y -= self.velocity;
                        }
                        Direction::Straight => {
                            self.x -= self.velocity;
                        }
                    }
                }
            }
            Side::FromSouth => {
                if self.y - self.velocity > OUTPUT_HEIGHT / 2 + 2 * CAR_HEIGHT {
                    self.y -= self.velocity;
                } else if self.y - self.velocity > OUTPUT_HEIGHT / 2 {
                    if feu.color == Light::Green {
                        self.y -= self.velocity;
                    } else {
                        self.y = OUTPUT_HEIGHT / 2 + CAR_HEIGHT;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.y = OUTPUT_HEIGHT / 2 - CAR_HEIGHT;
                            self.x -= self.velocity;
                        }
                        Direction::Right => {
                            self.y = OUTPUT_HEIGHT / 2;
                            self.x += self.velocity;
                        }
                        Direction::Straight => {
                            self.y -= self.velocity;
                        }
                    }
                }
            }
        }
    }
}

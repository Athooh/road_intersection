use super::car::*;
use super::constants::*;
use super::se_base::*;

/*
The struct represents a Road object with various fields to track the state of cars and traffic lights at an intersection.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Road {
    pub cars_before_stop_north: Vec<Car>,
    pub cars_before_stop_south: Vec<Car>,
    pub cars_before_stop_east: Vec<Car>,
    pub cars_before_stop_west: Vec<Car>,
    pub cars_in_intersection: Vec<Car>,
    pub cars_after_stop_north: Vec<Car>,
    pub cars_after_stop_south: Vec<Car>,
    pub cars_after_stop_east: Vec<Car>,
    pub cars_after_stop_west: Vec<Car>,
    pub north_lights: TrafficLight,
    pub east_lights: TrafficLight,
    pub south_lights: TrafficLight,
    pub west_lights: TrafficLight,
}

impl Road {
    /*
    The constructor function creates a new instance of the Road struct. It initializes all fields of the struct with default values:

    Eight fields representing lists of cars at different positions on the road are initialized as empty vectors.
    Four fields representing traffic lights at different directions are initialized with a TrafficLight struct having a color field set to Light::Red, indicating that all traffic lights are initially red.
    */
    pub fn new() -> Road {
        Road {
            cars_before_stop_north: vec![],
            cars_before_stop_south: vec![],
            cars_before_stop_east: vec![],
            cars_before_stop_west: vec![],
            cars_in_intersection: vec![],
            cars_after_stop_north: vec![],
            cars_after_stop_south: vec![],
            cars_after_stop_east: vec![],
            cars_after_stop_west: vec![],
            north_lights: TrafficLight { color: Light::Red },
            east_lights: TrafficLight { color: Light::Red },
            south_lights: TrafficLight { color: Light::Red },
            west_lights: TrafficLight { color: Light::Red },
        }
    }

    /*
    Simulation loop for a traffic intersection that simulates the movement of cars through an intersection with traffic lights.

    It checks the state of the traffic lights and updates them based on certain conditions.
    It moves cars through the intersection based on their direction and the state of the traffic lights.
    It checks for collisions and prevents cars from moving if there is a car in front of them.
    It updates the position of cars in the intersection and moves them to the next stage of their journey (e.g., from "before stop" to "in intersection" to "after stop").
    */
    pub fn simulation_loop(&mut self) {
        if self.south_lights.color == Light::Green
            || self.north_lights.color == Light::Green
            || self.east_lights.color == Light::Green
            || self.west_lights.color == Light::Green
        {
            self.east_lights = TrafficLight { color: Light::Red };
            self.north_lights = TrafficLight { color: Light::Red };
            self.south_lights = TrafficLight { color: Light::Red };
            self.west_lights = TrafficLight { color: Light::Red };
        } else {
            let a = self.cars_before_stop_east.len();
            let b = self.cars_before_stop_west.len();
            let c = self.cars_before_stop_north.len();
            let d = self.cars_before_stop_south.len();
            if a >= b && a >= c && a >= d {
                self.east_lights = TrafficLight {
                    color: Light::Green,
                };
                self.north_lights = TrafficLight { color: Light::Red };
                self.south_lights = TrafficLight { color: Light::Red };
                self.west_lights = TrafficLight { color: Light::Red };
            } else {
                if b >= a && b >= c && b >= d {
                    self.east_lights = TrafficLight { color: Light::Red };
                    self.north_lights = TrafficLight { color: Light::Red };
                    self.south_lights = TrafficLight { color: Light::Red };
                    self.west_lights = TrafficLight {
                        color: Light::Green,
                    };
                } else {
                    if c >= a && c >= b && c >= d {
                        self.east_lights = TrafficLight { color: Light::Red };
                        self.north_lights = TrafficLight {
                            color: Light::Green,
                        };
                        self.south_lights = TrafficLight { color: Light::Red };
                        self.west_lights = TrafficLight { color: Light::Red };
                    } else {
                        self.east_lights = TrafficLight { color: Light::Red };
                        self.north_lights = TrafficLight { color: Light::Red };
                        self.south_lights = TrafficLight {
                            color: Light::Green,
                        };
                        self.west_lights = TrafficLight { color: Light::Red };
                    }
                }
            }
        }

        /*
        If there are cars in the intersection, move the first one and check if it has exited the intersection.
        If it has, move it to the next stage of its journey (e.g., from "in intersection" to "after stop").
        If it hasn't, move the next car in the intersection if there is enough space.
        */
        if self.cars_in_intersection.len() != 0 {
            self.cars_in_intersection[0].moove(TrafficLight {
                color: Light::Green,
            });
            if (self.cars_in_intersection[0].x > OUTPUT_WIDTH / 2 + CAR_WIDTH
                || self.cars_in_intersection[0].x < OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH)
                || (self.cars_in_intersection[0].y > OUTPUT_HEIGHT / 2 + CAR_HEIGHT
                    || self.cars_in_intersection[0].y < OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT)
            {
                if (self.cars_in_intersection[0].side == Side::FromEast
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromSouth
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromWest
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_north
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                } else if (self.cars_in_intersection[0].side == Side::FromSouth
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromWest
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromNorth
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_east
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                } else if (self.cars_in_intersection[0].side == Side::FromWest
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromNorth
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromEast
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_south
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                } else if (self.cars_in_intersection[0].side == Side::FromNorth
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromEast
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromSouth
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_west
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                }
            }
        }

        /*
        Move cars that are leaving the intersection after the green light.
        For each car, check if the car in front of it has moved far enough away (i.e., more than the SECURITY_DISTANCE). If so, move the car and update the front car's position.
        */
        if self.cars_after_stop_east.len() > 0 {
            self.cars_after_stop_east[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_east[0].clone();
            for i in 1..self.cars_after_stop_east.len() {
                if self.cars_after_stop_east[i].x - self.cars_after_stop_east[i].velocity
                    > front_car.x + SECURITY_DISTANCE
                {
                    self.cars_after_stop_east[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_east[i].clone();
                }
            }
        }

        /*
        Move cars that are leaving the intersection after the green light.
        For each car, check if the car in front of it has moved far enough away (i.e., more than the SECURITY_DISTANCE). If so, move the car and update the front car's position.
        This simulates the movement of cars in the intersection and ensures that cars do not collide.
        */
        if self.cars_after_stop_north.len() > 0 {
            self.cars_after_stop_north[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_north[0].clone();
            for i in 1..self.cars_after_stop_north.len() {
                if self.cars_after_stop_north[i].y - self.cars_after_stop_north[i].velocity
                    > front_car.y + SECURITY_DISTANCE
                {
                    self.cars_after_stop_north[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_north[i].clone();
                }
            }
        }

        /*
        Move cars that are leaving the intersection after the green light.
        For each car, check if the car in front of it has moved far enough away (i.e., more than the SECURITY_DISTANCE). If so, move the car and update the front car's position.
        This simulates the movement of cars in the intersection and ensures that cars do not collide.
        */
        if self.cars_after_stop_west.len() > 0 {
            self.cars_after_stop_west[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_west[0].clone();
            for i in 1..self.cars_after_stop_west.len() {
                if self.cars_after_stop_west[i].x + self.cars_after_stop_west[i].velocity
                    < front_car.x - SECURITY_DISTANCE
                {
                    self.cars_after_stop_west[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_west[i].clone();
                }
            }
        }

        /*
        Move cars that are leaving the intersection after the green light.
        For each car, check if the car in front of it has moved far enough away (i.e., more than the SECURITY_DISTANCE). If so, move the car and update the front car's position.
        This simulates the movement of cars in the intersection and ensures that cars do not collide.
        */
        if self.cars_after_stop_south.len() > 0 {
            self.cars_after_stop_south[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_south[0].clone();
            for i in 1..self.cars_after_stop_south.len() {
                if self.cars_after_stop_south[i].y + self.cars_after_stop_south[i].velocity
                    < front_car.y - SECURITY_DISTANCE
                {
                    self.cars_after_stop_south[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_south[i].clone();
                }
            }
        }

        if self.cars_before_stop_north.len() > 0 {
            if self.cars_before_stop_north[0].y < (OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT) {
                self.cars_before_stop_north[0].moove(self.north_lights.clone());

                let mut front_car_y = self.cars_before_stop_north[0].y;
                for i in 1..self.cars_before_stop_north.len() {
                    if self.cars_before_stop_north[i].y + self.cars_before_stop_north[i].velocity
                        < front_car_y - SECURITY_DISTANCE
                    {
                        self.cars_before_stop_north[i].moove(self.north_lights.clone());
                        front_car_y = self.cars_before_stop_north[i].y;
                    }
                }
            }
            if self.cars_before_stop_north[0].y >= (OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT)
                && self.north_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_north[0].clone());
                    self.cars_before_stop_north.remove(0);
                }
            }
        }

        if self.cars_before_stop_south.len() > 0 {
            if self.cars_before_stop_south[0].y > (OUTPUT_HEIGHT / 2 + 2 * CAR_HEIGHT) {
                self.cars_before_stop_south[0].moove(self.south_lights.clone());

                let mut front_car_y = self.cars_before_stop_south[0].y;
                for i in 1..self.cars_before_stop_south.len() {
                    if self.cars_before_stop_south[i].y - self.cars_before_stop_south[i].velocity
                        > front_car_y + SECURITY_DISTANCE
                    {
                        self.cars_before_stop_south[i].moove(self.south_lights.clone());
                        front_car_y = self.cars_before_stop_south[i].y;
                    }
                }
            }
            if self.cars_before_stop_south[0].y <= (OUTPUT_HEIGHT / 2 + 2 * CAR_HEIGHT)
                && self.south_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_south[0].clone());
                    self.cars_before_stop_south.remove(0);
                }
            }
        }

        if self.cars_before_stop_east.len() > 0 {
            if self.cars_before_stop_east[0].x < (OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH) {
                self.cars_before_stop_east[0].moove(self.east_lights.clone());

                let mut front_car_x = self.cars_before_stop_east[0].x;
                for i in 1..self.cars_before_stop_east.len() {
                    if self.cars_before_stop_east[i].x + self.cars_before_stop_east[i].velocity
                        < front_car_x - SECURITY_DISTANCE
                    {
                        self.cars_before_stop_east[i].moove(self.east_lights.clone());
                        front_car_x = self.cars_before_stop_east[i].x;
                    }
                }
            }
            if self.cars_before_stop_east[0].x >= (OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH)
                && self.east_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_east[0].clone());
                    self.cars_before_stop_east.remove(0);
                }
            }
        }

        if self.cars_before_stop_west.len() > 0 {
            if self.cars_before_stop_west[0].x > (OUTPUT_WIDTH / 2 + CAR_WIDTH) {
                self.cars_before_stop_west[0].moove(self.west_lights.clone());

                let mut front_car_x = self.cars_before_stop_west[0].x;
                for i in 1..self.cars_before_stop_west.len() {
                    if self.cars_before_stop_west[i].x - self.cars_before_stop_west[i].velocity
                        > front_car_x + SECURITY_DISTANCE
                    {
                        self.cars_before_stop_west[i].moove(self.west_lights.clone());
                        front_car_x = self.cars_before_stop_west[i].x;
                    }
                }
            }
            if self.cars_before_stop_west[0].x <= (OUTPUT_WIDTH / 2 + CAR_WIDTH)
                && self.west_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_west[0].clone());
                    self.cars_before_stop_west.remove(0);
                }
            }
        }
    }
}

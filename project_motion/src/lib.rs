#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<ThrowObject> {
        self.time += 1.0;
        let actual_distance = Object {
            x: round_two(self.init_position.x + self.init_velocity.x * self.time),
            y: round_two(
                self.init_position.y + self.init_velocity.y * self.time
                    - 9.8 * self.time * self.time / 2.0,
            ),
        };

        let actual_velocity = Object {
            x: round_two(self.init_velocity.x),
            y: round_two(self.init_velocity.y - 9.8 * self.time),
        };

        if actual_distance.y < 0.0 {
            return None;
        } else {
            return Some(ThrowObject {
                init_position: self.init_position.clone(),
                init_velocity: self.init_velocity.clone(),
                actual_position: actual_distance,
                actual_velocity: actual_velocity,
                time: self.time,
            });
        }
    }
}

fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}

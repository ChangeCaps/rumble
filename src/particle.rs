use crate::map;
use nalgebra::*;
use rand::Rng;

#[derive(Clone)]
pub enum Particle {
    Solid {
        color: Vector4<f32>,
    },
    Fluid {
        color: Vector4<f32>,
        second_color: Vector4<f32>,
        spread: u32,
        lerp: f32,
    },
    Sand {
        color: Vector4<f32>,
    },
    Particle {
        velocity: Vector2<f32>,
        sub_position: Vector2<f32>,
        to_move: Vector2<f32>,
        base: Box<Particle>,
    },
}

impl Particle {
    pub fn fluid() -> Self {
        let mut rng = rand::thread_rng();

        Self::Fluid {
            color: Vector4::new(0.0, 0.2, 1.0, 0.5),
            second_color: Vector4::new(0.0, 0.25, 0.85, 0.5),
            spread: 4,
            lerp: rng.gen(),
        }
    }

    pub fn is_fluid(&self) -> bool {
        if let Particle::Fluid { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn solid() -> Self {
        Self::Solid {
            color: Vector4::new(0.3, 0.3, 0.3, 1.0),
        }
    }

    pub fn sand() -> Self {
        let mut rng = rand::thread_rng();

        Self::Sand {
            color: Vector4::new(
                0.7 + 0.1 * rng.gen::<f32>(),
                0.5 + 0.1 * rng.gen::<f32>(),
                0.1,
                1.0,
            ),
        }
    }

    pub fn particle(self) -> Self {
        Self::Particle {
            velocity: Vector2::new(0.0, 0.0),
            base: Box::new(self),
            to_move: Vector2::new(0.0, 0.0),
            sub_position: Vector2::new(0.0, 0.0),
        }
    }

    pub fn update_state(&mut self, position: Vector2<i32>, map: &map::Map) {
        match self {
            Particle::Fluid { lerp, .. } => *lerp = (*lerp + 0.05) % 2.0,
            Particle::Particle {
                velocity,
                base,
                sub_position,
                to_move,
            } => {
                velocity.y -= 0.1;

                *sub_position += *velocity;
                *to_move = *sub_position;

                sub_position.x = sub_position.x % 1.0;
                sub_position.y = sub_position.y % 1.0;

                base.update_state(position, map);

                let norm = velocity.normalize() * 1.8;
                let step = Vector2::new(norm.x.round() as i32, norm.y.round() as i32);
                let pos = position + step;

                if let Some(particle) = map.get(pos) {
                    match particle {
                        Particle::Particle { .. } => (),
                        _ => *self = (**base).clone(),
                    }
                } else {
                    if !map.void(pos) {
                        *self = (**base).clone();
                    }
                }
            }
            _ => (),
        }
    }

    pub fn update_position(&self, position: Vector2<i32>, map: &map::Map) -> Option<Vector2<i32>> {
        match self {
            Particle::Fluid { spread, .. } => {
                let mut rng = rand::thread_rng();

                let dir = if rng.gen::<u64>() % 2 == 0 { 1 } else { -1 };
                //let dir = -1;

                if map.void(position - Vector2::new(0, 1)) {
                    return Some(position - Vector2::new(0, 1));
                }

                let check = |pos: Vector2<i32>| {
                    /*
                    let d = if pos.x > 0 { 1 } else { -1 };

                    for i in 1..pos.x.abs() {
                        if !map.void(position - Vector2::new(i * d, pos.y)) {
                            return position - Vector2::new((i - 1) * d, pos.y);
                        }
                    }
                    */

                    position - pos
                };

                if map.surrounded(position) {
                    return None;
                }

                for j in (0..2).rev() {
                    for i in (1..*spread as i32 + 1).rev() {
                        if map.void(position - Vector2::new(i * dir, j)) {
                            return Some(check(Vector2::new(i * dir, j)));
                        } else if map.void(position - Vector2::new(i * -dir, j)) {
                            return Some(check(Vector2::new(i * -dir, j)));
                        }
                    }
                }

                None
            }

            Particle::Sand { .. } => {
                if map.void(position - Vector2::new(0, 1))
                    || map.is_fluid(position - Vector2::new(0, 1))
                {
                    Some(position - Vector2::new(0, 1))
                } else if map.void(position - Vector2::new(-1, 1))
                    || map.is_fluid(position - Vector2::new(-1, 1))
                {
                    Some(position - Vector2::new(-1, 1))
                } else if map.void(position - Vector2::new(1, 1))
                    || map.is_fluid(position - Vector2::new(1, 1))
                {
                    Some(position - Vector2::new(1, 1))
                } else {
                    None
                }
            }

            Particle::Particle { to_move, .. } => {
                if to_move.magnitude() < 0.1 {
                    return None;
                }

                let norm = to_move.normalize();
                let mut prev_position = position;

                for i in 1..to_move.magnitude().floor() as i32 {
                    let step = norm * i as f32;
                    let step = Vector2::new(step.x.round() as i32, step.y.round() as i32);

                    if step == position {
                        continue;
                    }

                    let check = position + step;

                    if !map.void(check) {
                        return Some(prev_position);
                    }

                    prev_position = check;
                }

                Some(prev_position)
            }

            _ => None,
        }
    }

    pub fn color(&self) -> Vector4<f32> {
        match self {
            Particle::Fluid {
                color,
                second_color,
                lerp,
                ..
            } => color.lerp(second_color, (*lerp - 1.0).abs()),
            Particle::Solid { color, .. } => *color,
            Particle::Particle { base, .. } => base.color(),
            Particle::Sand { color, .. } => *color,
        }
    }
}

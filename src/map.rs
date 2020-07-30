use crate::particle;
use nalgebra::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Map {
    particles: Vec<Vec<Option<particle::Particle>>>,
    dimensions: Vector2<usize>,
    flip: bool,
}

impl Map {
    pub fn empty(dimensions: Vector2<usize>) -> Self {
        let mut particles: Vec<Vec<Option<particle::Particle>>> =
            vec![vec![None; dimensions.y]; dimensions.x];

        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                if y > 250 && x > 250 {
                    //particles[x][y] = Some(particle::Particle::fluid().particle());
                }

                if x > 120 && x < 180 && y > 250 {
                    particles[x][y] = Some(particle::Particle::fluid().particle());
                }

                if x > y + 300 {
                    particles[x][y] = Some(particle::Particle::solid());
                }
            }
        }

        Self {
            particles,
            dimensions,
            flip: false,
        }
    }

    pub fn update_state(&mut self) {
        let old_map = self.clone();

        self.particles
            .iter_mut()
            .enumerate()
            .for_each(|(x, column)| {
                column.iter_mut().enumerate().for_each(|(y, particle)| {
                    if let Some(particle) = particle {
                        particle.update_state(Vector2::new(x as i32, y as i32), &old_map)
                    }
                });
            });
    }

    pub fn update_position(&mut self) {
        for y in 0..self.dimensions.y {
            for mut x in 0..self.dimensions.x {
                if self.flip {
                    x = self.dimensions.x - x - 1;
                }

                if let Some(particle) = self[x][y].clone() {
                    let pos = Vector2::new(x as i32, y as i32);

                    if let Some(new_pos) = particle.update_position(pos, &self) {
                        let particle = self[new_pos.x as usize][new_pos.y as usize].clone();

                        self[new_pos.x as usize][new_pos.y as usize] =
                            self[pos.x as usize][pos.y as usize].clone();

                        self[pos.x as usize][pos.y as usize] = particle;
                    }
                }
            }
        }

        self.flip = !self.flip;

        /*for (new_pos, particles) in moves {
            let index = rng.gen::<usize>() % particles.len();

            let pos = particles[index];

            let particle = self[new_pos.x as usize][new_pos.y as usize].clone();

            self[new_pos.x as usize][new_pos.y as usize] =
                self[pos.x as usize][pos.y as usize].clone();

            self[pos.x as usize][pos.y as usize] = particle;
        }*/
    }

    pub fn update(&mut self) {
        self.update_state();
        self.update_position();
    }

    pub fn surrounded(&self, position: Vector2<i32>) -> bool {
        !self.void(position + Vector2::new(-1, 0))
            && !self.void(position + Vector2::new(-1, -1))
            && !self.void(position + Vector2::new(0, -1))
            && !self.void(position + Vector2::new(1, -1))
            && !self.void(position + Vector2::new(1, 0))
            && !self.void(position + Vector2::new(1, 1))
            && !self.void(position + Vector2::new(0, 1))
            && !self.void(position + Vector2::new(-1, 1))
    }

    pub fn get(&self, position: Vector2<i32>) -> &Option<particle::Particle> {
        if position.x >= 0
            && position.x < self.dimensions.x as i32
            && position.y >= 0
            && position.y < self.dimensions.y as i32
        {
            &self[position.x as usize][position.y as usize]
        } else {
            &None
        }
    }

    pub fn is_fluid(&self, position: Vector2<i32>) -> bool {
        match self.get(position) {
            Some(particle) => particle.is_fluid(),
            None => false,
        }
    }

    pub fn void(&self, position: Vector2<i32>) -> bool {
        if position.x >= 0
            && position.x < self.dimensions.x as i32
            && position.y >= 0
            && position.y < self.dimensions.y as i32
        {
            self[position.x as usize][position.y as usize].is_none()
        } else {
            false
        }
    }

    pub fn to_texture(&self, facade: &impl glium::backend::Facade) -> glium::texture::Texture2d {
        let mut data = Vec::with_capacity(self.dimensions.x * self.dimensions.y * 4);

        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                if let Some(particle) = &self[x][y] {
                    let color = particle.color();

                    data.push(color.x);
                    data.push(color.y);
                    data.push(color.z);
                    data.push(color.w);
                } else {
                    data.push(0.0);
                    data.push(0.0);
                    data.push(0.0);
                    data.push(0.0);
                }
            }
        }

        let dims = self.dimensions;
        let raw_image =
            glium::texture::RawImage2d::from_raw_rgba(data, (dims.x as u32, dims.y as u32));

        glium::texture::Texture2d::new(facade, raw_image).unwrap()
    }
}

impl std::ops::Index<usize> for Map {
    type Output = Vec<Option<particle::Particle>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.particles[index]
    }
}

impl std::ops::IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.particles[index]
    }
}

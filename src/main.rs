mod map;
mod particle;

use glium::{glutin, uniform, Surface};
use nalgebra::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

glium::implement_vertex!(Vertex, position, tex_coord);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let contex_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, contex_builder, &event_loop).unwrap();

    let program = glium::program::Program::from_source(
        &display,
        include_str!("vertex.glsl"),
        include_str!("fragment.glsl"),
        None,
    )
    .unwrap();

    let vertex_buffer = glium::VertexBuffer::new(
        &display,
        &[
            Vertex {
                position: [-1.0, -1.0],
                tex_coord: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                tex_coord: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
                tex_coord: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0],
                tex_coord: [1.0, 1.0],
            },
        ],
    )
    .unwrap();

    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u32, 1, 2, 1, 2, 3],
    )
    .unwrap();

    let mut map = map::Map::empty(nalgebra::Vector2::new(450, 300));

    let mut next_frame_time = std::time::Instant::now();
    let frame_time = std::time::Duration::from_secs_f32(1.0 / 60.0);

    event_loop.run(move |event, _, control_flow| {
        if std::time::Instant::now() >= next_frame_time {
            next_frame_time = std::time::Instant::now() + frame_time;
        }

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            }

            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::Init => (),

                glutin::event::StartCause::ResumeTimeReached { .. } => (),

                _ => return,
            },
            _ => return,
        }

        for i in 0..10 {
            for j in 0..1 {
                if map[100 + i][250 + j].is_none() {
                    map[100 + i][250 + j] = Some(particle::Particle::Particle {
                        base: Box::new(particle::Particle::sand()),
                        velocity: Vector2::new(0.0, -2.0),
                        sub_position: Vector2::new(0.0, 0.0),
                        to_move: Vector2::new(0.0, 0.0),
                    });
                }
            }
        }

        map.update();

        let mut frame = display.draw();

        let map_texture = map.to_texture(&display);

        let uniforms = uniform! {
            map_texture: &map_texture
        };

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
    });
}

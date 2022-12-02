use std::f32::consts::PI;

use glam::Vec2;
use renderer::{Scene, Camera, Sphere, Shape, SpotLight, Light, get_cursor_world_position, Material, DirectionalLight};
use windowing::WindowingState;
use winit::event::{WindowEvent, Event};
use winit::event_loop::{ControlFlow, EventLoop};

mod windowing;
mod renderer;


fn main() {

    let event_loop = EventLoop::new();

    let dpi = 1;
    let mut windowing_state = windowing::WindowingState::new(&event_loop, dpi);
    let mut cursor_position = Vec2::new(0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        
        match event {
            Event::WindowEvent { event, .. } => match event {

                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(new_size) => {
                    windowing_state.resize(new_size);
                },
                WindowEvent::ScaleFactorChanged  { new_inner_size, ..} => {
                    windowing_state.resize(*new_inner_size);
                },

                WindowEvent::CursorMoved { position, .. } => {
                    let logical_position = position.to_logical(dpi as f64);
                    cursor_position = glam::vec2(logical_position.x, logical_position.y);
                },
                /*
                WindowEvent::KeyboardInput { input, ..} => {
                    inputs.on_keyboard_input(input.virtual_keycode, input.state);
                    engine.set_inputs(inputs.clone());
                },
                WindowEvent::MouseInput { state, button, ..} => {
                    inputs.on_mouse_input(button, state);
                    engine.set_inputs(inputs.clone());
                },*/

                _ => (),
            },

            Event::MainEventsCleared => {
                windowing_state.window.request_redraw();
            },

            Event::RedrawRequested(_) => {

                let WindowingState { context, size, .. } = &mut windowing_state;
                let buffer = context.get_frame_mut();

                let scene = simple_scene(cursor_position, Vec2::new(size.width as f32, size.height as f32));

                renderer::render(&scene, buffer, glam::UVec2::new(size.width, size.height));

                windowing_state.render();
            }

            _ => ()
        }
    });
}



fn simple_scene(cursor_position: Vec2, screen_size: Vec2) -> Scene {

    let camera = Camera { 
        position: glam::vec3(0.0, 0.0, 0.0), 
        rotation: glam::Quat::default(), 
        fov_y: 90.0 / 360.0 * PI,
        near_z: 0.1,
    };

    let mut objects = Vec::new();
    let mut lights = Vec::new();

    let blue_material = Material { color: glam::vec4(0.0, 0.4, 1.0, 1.0) };
    let orange_material = Material { color: glam::vec4(1.0, 0.7, 0.2, 1.0) };
    let green_material = Material { color: glam::vec4(0.4, 1.0, 0.6, 1.0) };
    let white_material = Material { color: glam::vec4(1.0, 1.0, 1.0, 1.0) };


    objects.push((0, Box::new(Sphere { position: glam::vec3(2.0, 2.0, 100.0), radius: 40.0 }) as Box<dyn Shape>, white_material));

    objects.push((0, Box::new(Sphere { position: glam::vec3(2.0, 2.0, 10.0), radius: 1.0 }) as Box<dyn Shape>, blue_material));
    objects.push((1, Box::new(Sphere { position: glam::vec3(-2.0, 2.0, 10.0), radius: 1.0 }) as Box<dyn Shape>, orange_material));

    objects.push((2, Box::new(Sphere { position: glam::vec3(6.0, -3.0, 15.0), radius: 0.7 }) as Box<dyn Shape>, orange_material));
    objects.push((3, Box::new(Sphere { position: glam::vec3(3.0, -5.0, 15.0), radius: 0.7 }) as Box<dyn Shape>, white_material));
    objects.push((4, Box::new(Sphere { position: glam::vec3(0.0, 0.0, 15.0), radius: 0.7 }) as Box<dyn Shape>, blue_material));
    objects.push((5, Box::new(Sphere { position: glam::vec3(-3.0, -5.0, 15.0), radius: 0.7 }) as Box<dyn Shape>, green_material));
    objects.push((6, Box::new(Sphere { position: glam::vec3(-6.0, -3.0, 15.0), radius: 0.7 }) as Box<dyn Shape>, orange_material));


    let cursor_position = get_cursor_world_position(cursor_position, &camera, screen_size, 10.0);
    lights.push(Box::new(SpotLight { position: cursor_position, intensity: 1.0 }) as Box<dyn Light>);

    lights.push(Box::new(SpotLight { position: glam::vec3(0.0, 5.0, 10.0), intensity: 1.0 }) as Box<dyn Light>);

    lights.push(Box::new(DirectionalLight { direction: glam::vec3(1.0, 0.5, 5.0), intensity: 1.0 }) as Box<dyn Light>);


    Scene { 
        objects, 
        lights, 
        camera,
    }
}
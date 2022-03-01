mod util;

use std::mem;
use std::os::raw::c_float;
use glow::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use cgmath::{Vector3};

fn main() {
    unsafe {
        let (gl, shader_version, window,
            event_loop) = {
            let event_loop = glutin::event_loop::EventLoop::new();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Triangle")
                .with_inner_size(glutin::dpi::LogicalSize::new(1020.0,
                                                               756.0));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            let gl =
                glow::Context::from_loader_function(|s|
                    window.get_proc_address(s) as *const _);
            (gl, "#version 410", window, event_loop)
        };

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");



        let verts: Vec<Vector3<f32>> = vec![
            Vector3::new(-0.5, -0.5, 0.0),
            Vector3::new(0.5, -0.5, 0.0),
            Vector3::new(0.0, 0.5, 0.0),
        ];

        let u8_verts = util::vert_to_u8(verts);

        let vbo = gl.create_buffer().unwrap();
        let vao = gl.create_vertex_array().unwrap();

        // ------------0. COPY VERTICES ARRAY IN A BUFFER FOR OPENGL USE-----------------------------
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, u8_verts, STATIC_DRAW);

        // ------------1. VERTEX ATTRIBUTE POINTERS-----------------------------
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT,
                                     false, 0, 0);
        gl.enable_vertex_attrib_array(0);
        // ------------2. SHADERS-----------------------------

        let (vertex_shader_source, fragment_shader_source) = (
            r#"
            layout (location = 0) in vec3 aPos;

            void main()
            {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
            "#,

            r#"
            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
            "#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        // shader Compiling
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader!");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        // -----------------------------------------

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }


        // USE SHADER PROGRAM
        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        gl.bind_vertex_array(Some(vao));

        // 3.0  DRAW THE TRIANGLE
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    window.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        gl.delete_program(program);
                        gl.delete_vertex_array(vertex_array);
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

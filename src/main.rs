use glow::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

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

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[4] = vec2[4](
                vec2(0.5f, 0.5f),   // top right
                vec2(0.5f, -0.5f),  // bottom right
                vec2(-0.5f, -0.5f), // bottom left
                vec2(-0.5f, 0.5f)   // top left
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert, 0.4, 1.0);
            }"#,

            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(1.0, 1.0, 1.0, 1.0);
            }"#,
        );

        // ebo attempt
        let indices: [u8; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        // try to mimic Brook's buffer addIndexbuffer method (change the indices)


        let ebo = gl.create_buffer().expect("ebo cannot be created");
        gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
        // messing up here
        gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER,&indices, STATIC_DRAW);

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

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

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        // gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

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
                    gl.draw_elements(TRIANGLES, 6, UNSIGNED_INT, 0);
                    // gl.draw_arrays(glow::TRIANGLES, 0, 3);
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

use glium::{glutin, implement_vertex, uniform, Surface};

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("StoneBlock by Adamekka");
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Failed to create display");

    // Build the vertex buffer
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5],
                    color: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [0.0, 0.5],
                    color: [1.0, 0.0, 0.0],
                },
            ],
        )
        .unwrap()
    };

    // Build the index buffer
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u16, 1, 2],
    )
    .unwrap();

    // Compile shaders and link them together
    let triangle_vertex = std::fs::read_to_string("src/shaders/triangle_vertex.glsl")
        .expect("Failed to read triangle_vertex.glsl");
    let triangle_fragment = std::fs::read_to_string("src/shaders/triangle_fragment.glsl")
        .expect("Failed to read triangle_fragment.glsl");

    let program =
        glium::Program::from_source(&display, &triangle_vertex, &triangle_fragment, None).unwrap();

    // Draw black background and triangle
    let draw = move || {
        // Build the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0,0.0,0.0,0.0],
                [0.0,1.0,0.0,0.0],
                [0.0,0.0,1.0,0.0],
                [0.0,0.0,0.0,1.0f32],
            ]
        };

        // Draw a frame
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
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
    };

    // Draw the triangle
    draw();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Close the window if the escape key or the exit button is pressed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::Resized(..) => {
                    draw();
                    glutin::event_loop::ControlFlow::Poll
                }
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}

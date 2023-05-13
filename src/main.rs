use glium::{glutin, implement_vertex, index::NoIndices, uniform, Surface, VertexBuffer};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("StoneBlock by Adamekka");
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Failed to create display");

    let shape: [Vertex; 3] = [
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
    ];

    implement_vertex!(Vertex, position, color);

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Build the index buffer
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Compile shaders and link them together
    let triangle_vertex = std::fs::read_to_string("src/shaders/triangle.vert")
        .expect("Failed to read triangle_vertex.glsl");
    let triangle_fragment = std::fs::read_to_string("src/shaders/triangle.frag")
        .expect("Failed to read triangle_fragment.glsl");

    let program =
        glium::Program::from_source(&display, &triangle_vertex, &triangle_fragment, None).unwrap();

    let mut step: f32 = -0.5;

    // Draw the triangle
    draw(&display, &vertex_buffer, &indices, &program, step);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Close the window if the escape key or the exit button is pressed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::Resized(..) => glutin::event_loop::ControlFlow::Poll,
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };

        // Wait until the next frame
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        step += 0.0002;
        if step > 0.5 {
            step = -0.5;
        }

        draw(&display, &vertex_buffer, &indices, &program, step);
    });
}

/// Draw a frame\
/// Draw black background and triangle
fn draw(
    display: &glium::Display,
    vertex_buffer: &VertexBuffer<Vertex>,
    index_buffer: &NoIndices,
    program: &glium::Program,
    step: f32,
) {
    let uniforms = uniform! {
        matrix: [
            [step.cos(), step.sin(), 0.0, 0.0],
            [-step.sin(), step.cos(), 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    };

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame
        .draw(
            vertex_buffer,
            index_buffer,
            program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
    frame.finish().unwrap();
}

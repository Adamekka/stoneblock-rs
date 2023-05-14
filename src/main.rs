mod teapot;

use glium::{glutin, uniform, Surface, VertexBuffer};
use teapot::{Normal, Vertex};

// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 2],
//     // color: [f32; 3],
//     tex_coords: [f32; 2],
// }

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("StoneBlock by Adamekka");
    let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Failed to create display");

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    // Load image
    // let image = image::load(
    //     std::io::Cursor::new(&include_bytes!("../src/images/ferris.png")),
    //     image::ImageFormat::Png,
    // )
    // .unwrap()
    // .to_rgba8();
    // let image_dimensions = image.dimensions();
    // let image =
    //     glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    // let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    // let shape: [Vertex; 3] = [
    //     Vertex {
    //         position: [-0.5, -0.5],
    //         tex_coords: [0.0, 0.0],
    //         // color: [0.0, 1.0, 0.0],
    //     },
    //     Vertex {
    //         position: [0.5, -0.5],
    //         tex_coords: [1.0, 0.0],
    //         // color: [0.0, 0.0, 1.0],
    //     },
    //     Vertex {
    //         position: [0.0, 0.5],
    //         tex_coords: [0.0, 1.0],
    //         // color: [1.0, 0.0, 0.0],
    //     },
    // ];

    // implement_vertex!(Vertex, position, tex_coords);

    // let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Build the index buffer
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Compile shaders and link them together
    // let triangle_vertex = std::fs::read_to_string("src/shaders/triangle.vert")
    //     .expect("Failed to read triangle.vert");
    // let triangle_fragment = std::fs::read_to_string("src/shaders/triangle.frag")
    //     .expect("Failed to read triangle.frag");
    let teapot_vertex =
        std::fs::read_to_string("src/shaders/teapot.vert").expect("Failed to read teapot.vert");
    let teapot_fragment =
        std::fs::read_to_string("src/shaders/teapot.frag").expect("Failed to read teapot.frag");

    let program =
        glium::Program::from_source(&display, &teapot_vertex, &teapot_fragment, None).unwrap();

    let mut step: f32 = -0.25;

    // Draw the triangle
    draw(
        &display,
        &positions,
        &normals,
        &index_buffer,
        &program,
        step,
    );

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
        if step > 0.75 {
            step = -0.25;
        }

        draw(
            &display,
            &positions,
            &normals,
            &index_buffer,
            &program,
            step,
        );
    });
}

/// Draw a frame
fn draw(
    display: &glium::Display,
    positions: &VertexBuffer<Vertex>,
    normals: &VertexBuffer<Normal>,
    index_buffer: &glium::IndexBuffer<u16>,
    // vertex_buffer: &VertexBuffer<Vertex>,
    // index_buffer: &NoIndices,
    program: &glium::Program,
    // texture: &glium::texture::Texture2d,
    step: f32,
) {
    let mut frame = display.draw();
    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let perspective: [[f32; 4]; 4] = {
        let (width, height) = frame.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = std::f32::consts::PI / 3.0;
        let zfar: f32 = 1024.0;
        let znear: f32 = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (zfar + znear) / (zfar - znear),
                -(2.0 * zfar * znear) / (zfar - znear),
            ],
            [0.0, 0.0, 1.0, 0.0],
        ]
    };

    let uniforms = uniform! {
        matrix: [
            [step.cos() / 100.0, -step.sin() / 100.0, 0.0, 0.0],
            [step.sin() / 100.0, step.cos() / 100.0, 0.0, 0.0],
            [0.0, 0.0, 0.01, 2.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        // tex: texture,
        u_light: [-1.0, 0.4, 0.9f32],
        perspective: perspective,
    };

    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    frame
        .draw(
            (positions, normals),
            index_buffer,
            program,
            &uniforms,
            &draw_parameters,
        )
        .unwrap();
    frame.finish().unwrap();
}

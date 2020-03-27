use crate::scene::scene::Scene;
use crate::util::camera::Camera;
use crate::util::vector::Vector;
use glium::uniform;
use glium::{Display, VertexBuffer, implement_vertex, Program, Surface};
use glium::index::{IndexBuffer, PrimitiveType};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event::{Event, WindowEvent};
use log::debug;

mod teapot;

use teapot::Vertex;
use teapot::Normal;

const VERTEX_SHADER: &'static str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;
    out vec3 v_normal;
    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;
    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 150

    in vec3 v_normal;
    out vec4 color;
    uniform vec3 u_light;
    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = vec3(0.6, 0.0, 0.0);
        vec3 regular_color = vec3(1.0, 0.0, 0.0);
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
"#;

// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 3],
//     texcoords: [f32; 2],
// }
//
//
// #[derive(Copy, Clone)]
// struct Normal {
//     normal: [f32; 3],
// }
//
// implement_vertex!(Vertex, position, texcoords);
// implement_vertex!(Normal, normal);

#[derive(Debug)]
pub enum PreviewerError {
    CreationError(String),
}

/// Provides an opengl based preview to get the camera position before a render.
pub struct Previewer<'p> {
    scene: &'p Scene<'p>,
    display: Display,

    // index_buffer: IndexBuffer<u32>,
    index_buffer: IndexBuffer<u16>,
    vertex_buffer: VertexBuffer<Vertex>,
    normal_buffer: VertexBuffer<Normal>,

    event_loop: EventLoop<()>,

    program: Program,
}


fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

impl<'p> Previewer<'p> {
    pub fn new(scene: &'p Scene) -> Result<Self, PreviewerError> {
        debug!("Initializing previewer");

        let event_loop = EventLoop::new();

        let wb = glium::glutin::window::WindowBuilder::new()
            // .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
            .with_title("Rusttracer Previewer");


        let cb = glium::glutin::ContextBuilder::new()
            .with_depth_buffer(24);

        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        // let vertices: Vec<Vertex> = scene.vertices().zip(scene.texture_coordinates()).map(|(vertex, texcoord)| {
        //     Vertex {
        //         position: [vertex.x as f32, vertex.y as f32, vertex.z as f32],
        //         texcoords: [texcoord.u as f32, texcoord.v as f32],
        //     }
        // }).collect();
        //
        // let normals: Vec<Normal> = scene.normals().map(|normal| {
        //     Normal {
        //         normal: [normal.x as f32, normal.y as f32, normal.z as f32],
        //     }
        // }).collect();

        // let indices: Vec<u32> = scene.triangles().flat_map(|triangle| {
        //     vec![triangle.index_a(), triangle.index_b(), triangle.index_c()].into_iter()
        // }).map(|i: usize| i as u32).collect();

        // let vertex_buffer = VertexBuffer::new(&display, &vertices)
        //     .map_err(|_| PreviewerError::CreationError("Couldn't create vertex buffer".into()))?;
        // let normal_buffer = VertexBuffer::new(&display, &normals)
        //     .map_err(|_| PreviewerError::CreationError("Couldn't create vertex buffer".into()))?;
        //
        // let index_buffer: IndexBuffer<u32> = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices)
        //     .map_err(|_| PreviewerError::CreationError("Couldn't create index buffer".into()))?;

        let vertex_buffer = VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
        let normal_buffer = VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
        let index_buffer = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                              &teapot::INDICES).unwrap();

        // compiling shaders and linking them together
        let program = Program::from_source(&display,
            VERTEX_SHADER,
            FRAGMENT_SHADER,
            None
        ).unwrap();


        Ok(Self {
            display,
            scene,

            index_buffer,
            vertex_buffer,
            normal_buffer,

            event_loop,

            program
        })
    }

    /// Waits before the user confirms the camera position, and then returns the current position.
    pub fn get_camera(self) -> Camera {
        self.run();


        Camera::new(Vector::new(0., 1.0, 3.), 1000, 1000, 60f64)
    }

    fn run(self) {

        let Previewer{event_loop, display, index_buffer, vertex_buffer, program, normal_buffer, ..} = self;




        event_loop.run(move |ev, _, control_flow| {

            debug!("Frame");

            let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = ControlFlow::WaitUntil(next_frame_time);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                NewEvents(cause) => match cause {
                    ResumeTimeReached { .. } => (),
                    Init => (),
                    _ => return,
                },
                _ => return,
            }


            let mut frame = display.draw();

            let perspective = {
                let (width, height) = frame.get_dimensions();
                let aspect_ratio = height as f32 / width as f32;

                let fov: f32 = 3.141592 / 3.0;
                let zfar = 1024.0;
                let znear = 0.1;

                let f = 1.0 / (fov / 2.0).tan();

                [
                    [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                    [         0.0         ,     f ,              0.0              ,   0.0],
                    [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                    [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                ]
            };


            let model = [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 2.0, 1.0f32]
            ];

            let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

            let light = [-1.0, 0.4, 0.9f32];

            // frame.clear_color(3./255., 165./255., 252./255., 1.);
            frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            let uniforms = uniform! {
                matrix: view,
                perspective: perspective,
                model: model,
                light: light
            };

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                .. Default::default()
            };

            frame.draw((&vertex_buffer, &normal_buffer), &index_buffer, &program, &uniforms, &params);

            frame.finish().unwrap();

        });

        // loop {
        //     let mut frame = self.display.draw();
        //
        //     let uniforms = uniform! {
        //         color: [0f32, 0f32, 0f32]
        //     };
        //
        //     frame.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &Default::default());
        //
        //     frame.finish();
        // }
    }
}
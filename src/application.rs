use context::ApplicationContext;
use glium::{
    Display, Surface,
    glutin::surface::WindowSurface,
    winit::{event::WindowEvent, window::Window},
};

use crate::{
    layer::{Layer, Layers, Vertex},
    objects::camera::Camera,
};

pub mod context;

pub struct Application {
    pub program: glium::Program,
    pub camera: Camera,
    pub layers: Layers,
}

impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "Erbium";

    fn new(display: &Display<WindowSurface>, _window: &Window) -> Self {
        let shaders = crate::utils::shaders::load_shaders();

        let program = glium::Program::from_source(
            display,
            &shaders.vertex_shader,
            &shaders.fragment_shader,
            None,
        )
        .unwrap();

        let mut layers = Layers::default();

        let piramid = Layer::new(
            vec![
                Vertex::new(0.0, 0.5, 0.0),
                Vertex::new(-0.5, -0.5, 0.5),
                Vertex::new(0.5, -0.5, 0.5),
                Vertex::new(0.5, -0.5, -0.5),
                Vertex::new(-0.5, -0.5, -0.5),
            ],
            vec![
                0, 1, 2, // front
                0, 2, 3, // right
                0, 3, 4, // back
                0, 4, 1, // left
                1, 2, 3, // bottom
                1, 3, 4,
            ],
        );

        let triangle = Layer::new(
            vec![
                Vertex::new(0.0, 0.5, 0.0),
                Vertex::new(-0.5, -0.5, 0.5),
                Vertex::new(0.5, -0.5, 0.5),
            ],
            vec![0, 1, 2],
        );

        layers.add_layer(piramid);
        layers.add_layer(triangle);

        Self {
            program,
            layers,
            camera: Camera::new(),
        }
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        let uniforms = uniform! {
            persp_matrix: self.camera.get_perspective(),
            view_matrix: self.camera.get_view(),
        };

        let vertex_buffer = self.layers.to_vertex_buffer(display);
        let index_buffer = self.layers.to_index_buffer(display);

        frame.clear_color(0.01, 0.01, 0.01, 1.0);
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
    }

    fn handle_window_event(&mut self, event: &WindowEvent, window: &Window) {
        self.camera.process_input(event, window);
    }

    fn update(&mut self) {
        self.camera.update();
    }
}

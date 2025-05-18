use glium::glutin::surface::WindowSurface;

#[derive(Default)]
pub struct Layers(pub Vec<Layer>);

impl Layers {
    pub fn to_vertex_buffer(
        &self,
        display: &glium::Display<WindowSurface>,
    ) -> glium::VertexBuffer<Vertex> {
        let vertices: Vec<Vertex> = self
            .0
            .iter()
            .flat_map(|layer| layer.vertex_buffer.to_vec())
            .collect();
        glium::VertexBuffer::new(display, &vertices).unwrap()
    }

    pub fn to_index_buffer(
        &self,
        display: &glium::Display<WindowSurface>,
    ) -> glium::IndexBuffer<u32> {
        let indices: Vec<u32> = self
            .0
            .iter()
            .flat_map(|layer| layer.index_buffer.to_vec())
            .collect();
        glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap()
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.0.push(layer);
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub translation: [f32; 3],
}
implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            translation: [0.0, 0.0, 0.0],
        }
    }

    pub fn with_translation(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.translation = [x, y, z];

        self
    }

    pub fn with_position(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.position = [x, y, z];

        self
    }

    pub fn build(self) -> Self {
        self
    }
}

pub struct Layer {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>,
}

impl Layer {
    pub fn new(vertex_buffer: Vec<Vertex>, index_buffer: Vec<u32>) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

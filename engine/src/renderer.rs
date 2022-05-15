use glium::{
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniforms, Display, Program, Surface, VertexBuffer,
};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

// helper function
fn create_vertices() -> Vec<Vertex> {
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        color: [0.0, 1.0, 0.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        color: [0.0, 0.0, 1.0],
    };
    vec![vertex1, vertex2, vertex3]
}

pub struct Renderer {
    vertex_buffer: VertexBuffer<Vertex>,
    indices: NoIndices,
    program: Program,
}

impl Renderer {
    pub fn new(display: &Display) -> Self {
        let vertex_buffer = VertexBuffer::new(display, &create_vertices()).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        let vertex_source = include_str!("../../assets/shader/vertex_shader.vert");
        let fragment_source = include_str!("../../assets/shader/fragment_shader.frag");
        let program = Program::from_source(display, vertex_source, fragment_source, None).unwrap();

        Self {
            vertex_buffer,
            indices,
            program,
        }
    }

    pub fn draw(&self, display: &Display) {
        let mut target = display.draw();

        target.clear_color(0.3, 0.5, 0.7, 1.0);
        target
            .draw(
                &self.vertex_buffer,
                self.indices,
                &self.program,
                &uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }
}

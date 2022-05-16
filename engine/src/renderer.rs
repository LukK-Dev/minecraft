use glium::{
    implement_vertex, index::PrimitiveType, uniforms, Display, DrawParameters, IndexBuffer,
    PolygonMode, Program, Surface, VertexBuffer,
};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

// helper function
fn create_vertices() -> Vec<Vertex> {
    let vertex1 = Vertex {
        position: [0.0, 0.5, 1.0],
        color: [1.0, 0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [-0.5, -0.5, 1.0],
        color: [0.0, 1.0, 0.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5, 1.0],
        color: [0.0, 0.0, 1.0],
    };
    let vertex4 = Vertex {
        position: [-0.5, 0.25, 1.0],
        color: [0.0, 0.0, 1.0],
    };
    let vertex5 = Vertex {
        position: [0.0, -0.75, 1.0],
        color: [1.0, 0.0, 0.0],
    };
    let vertex6 = Vertex {
        position: [0.5, 0.25, 1.0],
        color: [0.0, 1.0, 0.0],
    };
    vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]
}

fn create_indices() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5]
}

pub struct Renderer<'a> {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    draw_parameters: DrawParameters<'a>,
    program: Program,
}

impl Renderer<'_> {
    pub fn new(display: &Display) -> Self {
        let vertex_buffer = VertexBuffer::new(display, &create_vertices()).unwrap();
        let index_buffer =
            IndexBuffer::new(display, PrimitiveType::TrianglesList, &create_indices()).unwrap();

        let draw_parameters = DrawParameters {
            line_width: None,
            point_size: None,
            polygon_mode: PolygonMode::Fill,
            ..Default::default()
        };

        let vertex_source = include_str!("../../assets/shader/vertex_shader.vert");
        let fragment_source = include_str!("../../assets/shader/fragment_shader.frag");
        let program = Program::from_source(display, vertex_source, fragment_source, None).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            draw_parameters,
            program,
        }
    }

    pub fn draw(&self, display: &Display) {
        let mut target = display.draw();

        target.clear_color(0.3, 0.5, 0.7, 1.0);

        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms::EmptyUniforms,
                &self.draw_parameters,
            )
            .unwrap();

        target.finish().unwrap();
    }
}

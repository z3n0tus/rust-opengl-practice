use glium::{Frame, Display, Surface, Program, VertexBuffer};
use glium::index::NoIndices;

// A Vertex struct to hold each individual vertex of our shape.
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2]
}

// OpenGL doesn't use pixel co-ordinates. It considers a window to have a width and height of 2 units, with its origin in the center.
// So our vertices need to work with that.
// These three vertices will draw the following shape: https://raw.githubusercontent.com/glium/glium/master/book/tuto-02-triangle-coords.svg?sanitize=true
fn get_triangle_vertices() -> Vec<Vertex> {
    let vertex1 = Vertex {
        position: [-0.5, -0.5]
    };

    let vertex2 = Vertex {
        position: [0.0, 0.5]
    };

    let vertex3 = Vertex {
        position: [0.5, -0.25]
    };

    vec![vertex1, vertex2, vertex3]
}

/**
    This is the src code for our vertex shader, which is responsible for translating
    our vertices to their real co-ordinates on screen. We need to do this calculation
    because OpenGL currently only knows about our co-ordinates in the 0.5, -0.5, etc
    format, not in pixel format.
**/
fn vertex_shader() -> &'static str {
    r#"
        #version 140

        // This tells the shader that we expect a position variable to be passed to it. We say it's a vec2 because our Vertex struct
        // holds a Vec struct with 2 values in it, our x & y co-ordinates.
        in vec2 position;

        // The main function is called once per vertex, and for each vertex the position value will be equal to the 2d vertex array, e.g. [0.5, -0.5]
        // This function then does some conversion to screen space. In this case, it's doing no calculation at all, and will just draw it at
        // whatever pixel position corresponds to the float co-ordinates.
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#
}

/**
    The fragment shader works out what colour each vertex should be.
    Colours are in rgba format.
**/
fn fragment_shader() -> &'static str {
    r#"
        #version 140

        // Out is like a return statement. This shader will return a colour.
        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#
}

pub fn get_vertex_buffer(display: &Display) -> VertexBuffer<Vertex> {
    // Implement the necessary functions for our Vertex struct (handled by glium).
    glium::implement_vertex!(Vertex, position);

    let triangle_vertices: Vec<Vertex> = get_triangle_vertices();

    // Create a vertex buffer, which just makes pushing memory to our video card faster.
    glium::VertexBuffer::new(display, &triangle_vertices).unwrap()
}

// We don't need to pass any indices for a very simple shape so we just pass this placeholder.
pub fn get_index_buffer() -> NoIndices {
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
}

pub fn get_shader_program(display: &Display) -> Program {
    // Example of how the graphics pipeline works before we continue to create a shader program:
    // https://github.com/glium/glium/blob/master/book/tuto-02-triangle.md#program
    let vertex_shader = vertex_shader();
    let fragment_shader = fragment_shader();

    // Create a shader program, which combines the shaders into something that the gpu will execute on our vertices.
    glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap()
}
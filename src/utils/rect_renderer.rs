use crate::gl;
use crate::types::Rect;
use crate::utils::{ArrayBuffer, ShaderProgram, ShaderSource};

fn compute_viewport_matrix(size: (f32, f32)) -> nalgebra_glm::TMat4<f32> {
    nalgebra_glm::ortho(0.0, size.0, size.1, 0.0, -1.0, 1.0)
}

fn color_rgba_from_u32(col: u32, alpha: f32) -> nalgebra_glm::TVec4<f32> {
    nalgebra_glm::vec4(
        ((col & 0xff0000) >> 16) as f32 / 255.0,
        ((col & 0x00ff00) >> 8) as f32 / 255.0,
        (col & 0x0000ff) as f32 / 255.0,
        alpha,
    )
}

struct NineSlice {
    horiz: [f32; 4],
    vert: [f32; 4],
}

impl NineSlice {
    pub fn new(horiz: [f32; 4], vert: [f32; 4]) -> Self {
        Self { horiz, vert }
    }

    pub fn rect(&self, col: usize, row: usize) -> Rect {
        Rect {
            x: self.horiz[col],
            w: self.horiz[col + 1] - self.horiz[col],
            y: self.vert[row],
            h: self.vert[row + 1] - self.vert[row],
        }
    }

    pub fn edge_rect(&self, edge: usize, width: f32) -> Rect {
        match edge {
            0 => self.rect(1, 0).edge(0, width),
            1 => self.rect(2, 1).edge(1, width),
            2 => self.rect(1, 2).edge(2, width),
            3 => self.rect(0, 1).edge(3, width),
            _ => panic!("unknown edge"),
        }
    }

    pub fn corner_rect(&self, corner: usize) -> Rect {
        match corner {
            0 => self.rect(0, 0).hvflip(),
            1 => self.rect(2, 0).vflip(),
            2 => self.rect(2, 2),
            3 => self.rect(0, 2).hflip(),
            _ => panic!("unknown corner"),
        }
    }
}

pub struct RectRendererSpec {
    pub viewport_size: (f32, f32),
    pub rect: Rect,
    pub col: u32,
    pub border_col: u32,
    pub border_width: f32,
    pub corner_radius: f32,
    pub borders: [bool; 4],
    pub alpha: f32,
}

/// Render rectangles.
pub struct RectRenderer {
    buf: ArrayBuffer,

    round_program: ShaderProgram,
    round_mvp: i32,
    round_vertex: i32,
    round_pos: i32,
    round_size: i32,
    round_col: i32,
    round_smoothness: i32,
    round_inner_rad: i32,

    square_program: ShaderProgram,
    square_mvp: i32,
    square_vertex: i32,
    square_pos: i32,
    square_size: i32,
    square_col: i32,
}

impl RectRenderer {
    /// Create a RectRenderer
    pub fn new() -> Self {
        let vertex_shader_source = "
            #version 300 es
            precision mediump float;
            uniform vec2 pos,size;
            uniform mat4 mvp;
            in vec2 vertex;
            out vec2 frag_tex_coord;

            void main() {
                gl_Position=mvp*vec4(pos+size*vertex, 0.0, 1.0);
                frag_tex_coord=vertex;
            }
        "
        .to_string();

        let round_program = ShaderProgram::new(vec![
            ShaderSource::VertexShader(vertex_shader_source.clone()),
            ShaderSource::FragmentShader(
                "
                #version 300 es
                precision mediump float;
                uniform vec4 col;
                uniform float smoothness;
                uniform float inner_rad;
                in vec2 frag_tex_coord;
                out vec4 frag_col;

                float map(float value, float inMin, float inMax, float outMin, float outMax) {
                    return outMin + (outMax - outMin) * (value - inMin) / (inMax - inMin);
                }

                void main() {
                    float l=length(frag_tex_coord);
                    float hs=smoothness/2.0;
                    float or=clamp(map(l,1.0f-hs,1.0f+hs,1.0f,0.0f),0.0f,1.0f);
                    float ir=clamp(map(l,inner_rad-hs,inner_rad+hs,1.0f,0.0f),0.0f,1.0f);
                    frag_col = vec4(col.r, col.g, col.b, col.a*(or-ir));
                }
            "
                .to_string(),
            ),
        ]);

        let square_program = ShaderProgram::new(vec![
            ShaderSource::VertexShader(vertex_shader_source.clone()),
            ShaderSource::FragmentShader(
                "
                #version 300 es
                precision mediump float;
                uniform vec4 col;
                out vec4 frag_col;

                void main() {
                    frag_col = vec4(col.r, col.g, col.b, col.a);
                }
            "
                .to_string(),
            ),
        ]);

        let mut buf = ArrayBuffer::new(2);
        buf.set_data(vec![1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0]);

        RectRenderer {
            buf,

            round_smoothness: round_program.get_uniform_location("smoothness"),
            round_mvp: round_program.get_uniform_location("mvp"),
            round_pos: round_program.get_uniform_location("pos"),
            round_size: round_program.get_uniform_location("size"),
            round_col: round_program.get_uniform_location("col"),
            round_vertex: round_program.get_attrib_location("vertex"),
            round_inner_rad: round_program.get_uniform_location("inner_rad"),
            round_program,

            square_mvp: square_program.get_uniform_location("mvp"),
            square_pos: square_program.get_uniform_location("pos"),
            square_size: square_program.get_uniform_location("size"),
            square_col: square_program.get_uniform_location("col"),
            square_vertex: square_program.get_attrib_location("vertex"),
            square_program,
        }
    }

    fn round(&self, spec: &RectRendererSpec, r: Rect, col: u32, inner: f32) {
        let m = compute_viewport_matrix(spec.viewport_size);
        let c = color_rgba_from_u32(col, spec.alpha);
        self.round_program.use_program();
        self.buf.bind(self.round_vertex, 0, 2);

        let size=(r.w.abs()+r.h.abs())/2.0;
        let smoothness = 1.0 / size;
        let inner_rad = inner / size;

        unsafe {
            gl::Uniform1f(self.round_smoothness, smoothness);
            gl::Uniform1f(self.round_inner_rad, inner_rad);
            gl::Uniform2f(self.round_pos, r.x, r.y);
            gl::Uniform2f(self.round_size, r.w, r.h);
            gl::Uniform4fv(self.round_col, 1, c.as_ptr());
            gl::UniformMatrix4fv(self.round_mvp, 1, gl::FALSE, m.as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.buf.len() as i32);
        }
    }

    fn square(&self, spec: &RectRendererSpec, r: Rect, col: u32) {
        let m = compute_viewport_matrix(spec.viewport_size);
        let c = color_rgba_from_u32(col, spec.alpha);
        self.square_program.use_program();
        self.buf.bind(self.square_vertex, 0, 2);

        unsafe {
            gl::Uniform2f(self.square_pos, r.x, r.y);
            gl::Uniform2f(self.square_size, r.w, r.h);
            gl::Uniform4fv(self.square_col, 1, c.as_ptr());
            gl::UniformMatrix4fv(self.square_mvp, 1, gl::FALSE, m.as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.buf.len() as i32);
        }
    }

    /// Draw a rect, specified by orientation and size.
    pub fn draw(&self, spec: &RectRendererSpec) {
        let r = &spec.rect;
        let cr = spec.corner_radius;
        let bw = spec.border_width;
        let ir = spec.corner_radius - spec.border_width;
        let n = NineSlice::new(
            [r.x, r.x + cr, r.x + r.w - cr, r.x + r.w],
            [r.y, r.y + cr, r.y + r.h - cr, r.y + r.h],
        );

        // Main
        self.round(spec, n.rect(0, 0).hvflip(), spec.col, 0.0);
        self.square(spec, n.rect(1, 0), spec.col);
        self.round(spec, n.rect(2, 0).vflip(), spec.col, 0.0);
        self.square(spec, n.rect(0, 1), spec.col);
        self.square(spec, n.rect(1, 1), spec.col);
        self.square(spec, n.rect(2, 1), spec.col);
        self.round(spec, n.rect(0, 2).hflip(), spec.col, 0.0);
        self.square(spec, n.rect(1, 2), spec.col);
        self.round(spec, n.rect(2, 2), spec.col, 0.0);

        // Corner borders
        for i in 0..4 {
            if spec.borders[i] && spec.borders[(i + 3) % 4] {
                self.round(spec, n.corner_rect(i), spec.border_col, ir);
            }
        }

        // Edge borders
        for i in 0..4 {
            if spec.borders[i] {
                self.square(spec, n.edge_rect(i, bw), spec.border_col);
            }
        }
    }
}

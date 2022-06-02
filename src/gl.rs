#![allow(bad_style)]
#![allow(unused)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]

use std::{error::Error, ffi::CStr, fmt::Display, mem::transmute, os::raw::*};

#[cfg(all(feature = "tracing", feature = "trace-calls"))]
use tracing::{error, trace};

pub type Result<T, E = LoadError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct LoadError {
    pub name: String,
    pub pointer: usize,
}

impl Error for LoadError {}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to load function \"{}\", expected a valid pointer instead got {}",
            self.name, self.pointer
        )
    }
}

pub use types::*;
pub mod types {
    use std::os::raw::*;
    pub type GLvoid = c_void;
    pub type GLbyte = c_char;
    pub type GLubyte = c_uchar;
    pub type GLchar = c_char;
    pub type GLboolean = c_uchar;
    pub type GLshort = c_short;
    pub type GLushort = c_ushort;
    pub type GLint = c_int;
    pub type GLuint = c_uint;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;
    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;
    pub type GLsizei = GLint;
    pub type GLclampx = c_int;
    pub type GLfixed = GLint;
    pub type GLhalf = c_ushort;
    pub type GLhalfNV = c_ushort;
    pub type GLhalfARB = c_ushort;
    pub type GLenum = c_uint;
    pub type GLbitfield = c_uint;
    pub type GLfloat = c_float;
    pub type GLdouble = c_double;
    pub type GLclampf = c_float;
    pub type GLclampd = c_double;
    pub type GLcharARB = c_char;
    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = c_uint;
    pub enum __GLsync {}
    pub type GLsync = *const __GLsync;
    pub enum _cl_context {}
    pub enum _cl_event {}
    pub type GLvdpauSurfaceNV = GLintptr;
    pub type GLeglClientBufferEXT = *const c_void;
    pub type GLeglImageOES = *const c_void;
    pub type GLDEBUGPROC = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut c_void,
    );
    pub type GLDEBUGPROCARB = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut c_void,
    );
    pub type GLDEBUGPROCKHR = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLDEBUGPROCAMD = extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLVULKANPROCNV = extern "system" fn();
}

pub use enums::*;
pub mod enums {
    use super::*;
    pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
    pub const GL_STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;
    pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
    pub const GL_DYNAMIC_STORAGE_BIT: GLbitfield = 0x0100;
    pub const GL_CLIENT_STORAGE_BIT: GLbitfield = 0x0200;
    pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLbitfield = 0x00000001;
    pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = 0x00000002;
    pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLbitfield = 0x00000004;
    pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: GLbitfield = 0x00000008;
    pub const GL_CONTEXT_CORE_PROFILE_BIT: GLbitfield = 0x00000001;
    pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GLbitfield = 0x00000002;
    pub const GL_MAP_READ_BIT: GLbitfield = 0x0001;
    pub const GL_MAP_WRITE_BIT: GLbitfield = 0x0002;
    pub const GL_MAP_INVALIDATE_RANGE_BIT: GLbitfield = 0x0004;
    pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLbitfield = 0x0008;
    pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLbitfield = 0x0010;
    pub const GL_MAP_UNSYNCHRONIZED_BIT: GLbitfield = 0x0020;
    pub const GL_MAP_PERSISTENT_BIT: GLbitfield = 0x0040;
    pub const GL_MAP_COHERENT_BIT: GLbitfield = 0x0080;
    pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLbitfield = 0x00000001;
    pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GLbitfield = 0x00000002;
    pub const GL_UNIFORM_BARRIER_BIT: GLbitfield = 0x00000004;
    pub const GL_TEXTURE_FETCH_BARRIER_BIT: GLbitfield = 0x00000008;
    pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLbitfield = 0x00000020;
    pub const GL_COMMAND_BARRIER_BIT: GLbitfield = 0x00000040;
    pub const GL_PIXEL_BUFFER_BARRIER_BIT: GLbitfield = 0x00000080;
    pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GLbitfield = 0x00000100;
    pub const GL_BUFFER_UPDATE_BARRIER_BIT: GLbitfield = 0x00000200;
    pub const GL_FRAMEBUFFER_BARRIER_BIT: GLbitfield = 0x00000400;
    pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLbitfield = 0x00000800;
    pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GLbitfield = 0x00001000;
    pub const GL_SHADER_STORAGE_BARRIER_BIT: GLbitfield = 0x00002000;
    pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLbitfield = 0x00004000;
    pub const GL_QUERY_BUFFER_BARRIER_BIT: GLbitfield = 0x00008000;
    pub const GL_ALL_BARRIER_BITS: GLbitfield = 0xFFFFFFFF;
    pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLbitfield = 0x00000001;
    pub const GL_VERTEX_SHADER_BIT: GLbitfield = 0x00000001;
    pub const GL_FRAGMENT_SHADER_BIT: GLbitfield = 0x00000002;
    pub const GL_GEOMETRY_SHADER_BIT: GLbitfield = 0x00000004;
    pub const GL_TESS_CONTROL_SHADER_BIT: GLbitfield = 0x00000008;
    pub const GL_TESS_EVALUATION_SHADER_BIT: GLbitfield = 0x00000010;
    pub const GL_COMPUTE_SHADER_BIT: GLbitfield = 0x00000020;
    pub const GL_ALL_SHADER_BITS: GLbitfield = 0xFFFFFFFF;
    pub const GL_FALSE: GLenum = 0;
    pub const GL_NO_ERROR: GLenum = 0;
    pub const GL_ZERO: GLenum = 0;
    pub const GL_NONE: GLenum = 0;
    pub const GL_TRUE: GLenum = 1;
    pub const GL_ONE: GLenum = 1;
    pub const GL_INVALID_INDEX: GLenum = 0xFFFFFFFF;
    pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const GL_POINTS: GLenum = 0x0000;
    pub const GL_LINES: GLenum = 0x0001;
    pub const GL_LINE_LOOP: GLenum = 0x0002;
    pub const GL_LINE_STRIP: GLenum = 0x0003;
    pub const GL_TRIANGLES: GLenum = 0x0004;
    pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
    pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
    pub const GL_QUADS: GLenum = 0x0007;
    pub const GL_LINES_ADJACENCY: GLenum = 0x000A;
    pub const GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B;
    pub const GL_TRIANGLES_ADJACENCY: GLenum = 0x000C;
    pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
    pub const GL_PATCHES: GLenum = 0x000E;
    pub const GL_NEVER: GLenum = 0x0200;
    pub const GL_LESS: GLenum = 0x0201;
    pub const GL_EQUAL: GLenum = 0x0202;
    pub const GL_LEQUAL: GLenum = 0x0203;
    pub const GL_GREATER: GLenum = 0x0204;
    pub const GL_NOTEQUAL: GLenum = 0x0205;
    pub const GL_GEQUAL: GLenum = 0x0206;
    pub const GL_ALWAYS: GLenum = 0x0207;
    pub const GL_SRC_COLOR: GLenum = 0x0300;
    pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
    pub const GL_SRC_ALPHA: GLenum = 0x0302;
    pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
    pub const GL_DST_ALPHA: GLenum = 0x0304;
    pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
    pub const GL_DST_COLOR: GLenum = 0x0306;
    pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
    pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;
    pub const GL_FRONT_LEFT: GLenum = 0x0400;
    pub const GL_FRONT_RIGHT: GLenum = 0x0401;
    pub const GL_BACK_LEFT: GLenum = 0x0402;
    pub const GL_BACK_RIGHT: GLenum = 0x0403;
    pub const GL_FRONT: GLenum = 0x0404;
    pub const GL_BACK: GLenum = 0x0405;
    pub const GL_LEFT: GLenum = 0x0406;
    pub const GL_RIGHT: GLenum = 0x0407;
    pub const GL_FRONT_AND_BACK: GLenum = 0x0408;
    pub const GL_INVALID_ENUM: GLenum = 0x0500;
    pub const GL_INVALID_VALUE: GLenum = 0x0501;
    pub const GL_INVALID_OPERATION: GLenum = 0x0502;
    pub const GL_STACK_OVERFLOW: GLenum = 0x0503;
    pub const GL_STACK_UNDERFLOW: GLenum = 0x0504;
    pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;
    pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
    pub const GL_CONTEXT_LOST: GLenum = 0x0507;
    pub const GL_CW: GLenum = 0x0900;
    pub const GL_CCW: GLenum = 0x0901;
    pub const GL_POINT_SIZE: GLenum = 0x0B11;
    pub const GL_POINT_SIZE_RANGE: GLenum = 0x0B12;
    pub const GL_SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
    pub const GL_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
    pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
    pub const GL_LINE_SMOOTH: GLenum = 0x0B20;
    pub const GL_LINE_WIDTH: GLenum = 0x0B21;
    pub const GL_LINE_WIDTH_RANGE: GLenum = 0x0B22;
    pub const GL_SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
    pub const GL_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
    pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
    pub const GL_POLYGON_MODE: GLenum = 0x0B40;
    pub const GL_POLYGON_SMOOTH: GLenum = 0x0B41;
    pub const GL_CULL_FACE: GLenum = 0x0B44;
    pub const GL_CULL_FACE_MODE: GLenum = 0x0B45;
    pub const GL_FRONT_FACE: GLenum = 0x0B46;
    pub const GL_DEPTH_RANGE: GLenum = 0x0B70;
    pub const GL_DEPTH_TEST: GLenum = 0x0B71;
    pub const GL_DEPTH_WRITEMASK: GLenum = 0x0B72;
    pub const GL_DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
    pub const GL_DEPTH_FUNC: GLenum = 0x0B74;
    pub const GL_STENCIL_TEST: GLenum = 0x0B90;
    pub const GL_STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
    pub const GL_STENCIL_FUNC: GLenum = 0x0B92;
    pub const GL_STENCIL_VALUE_MASK: GLenum = 0x0B93;
    pub const GL_STENCIL_FAIL: GLenum = 0x0B94;
    pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
    pub const GL_STENCIL_REF: GLenum = 0x0B97;
    pub const GL_STENCIL_WRITEMASK: GLenum = 0x0B98;
    pub const GL_VIEWPORT: GLenum = 0x0BA2;
    pub const GL_DITHER: GLenum = 0x0BD0;
    pub const GL_BLEND_DST: GLenum = 0x0BE0;
    pub const GL_BLEND_SRC: GLenum = 0x0BE1;
    pub const GL_BLEND: GLenum = 0x0BE2;
    pub const GL_LOGIC_OP_MODE: GLenum = 0x0BF0;
    pub const GL_COLOR_LOGIC_OP: GLenum = 0x0BF2;
    pub const GL_DRAW_BUFFER: GLenum = 0x0C01;
    pub const GL_READ_BUFFER: GLenum = 0x0C02;
    pub const GL_SCISSOR_BOX: GLenum = 0x0C10;
    pub const GL_SCISSOR_TEST: GLenum = 0x0C11;
    pub const GL_COLOR_CLEAR_VALUE: GLenum = 0x0C22;
    pub const GL_COLOR_WRITEMASK: GLenum = 0x0C23;
    pub const GL_DOUBLEBUFFER: GLenum = 0x0C32;
    pub const GL_STEREO: GLenum = 0x0C33;
    pub const GL_LINE_SMOOTH_HINT: GLenum = 0x0C52;
    pub const GL_POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
    pub const GL_UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
    pub const GL_UNPACK_LSB_FIRST: GLenum = 0x0CF1;
    pub const GL_UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
    pub const GL_UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
    pub const GL_UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
    pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5;
    pub const GL_PACK_SWAP_BYTES: GLenum = 0x0D00;
    pub const GL_PACK_LSB_FIRST: GLenum = 0x0D01;
    pub const GL_PACK_ROW_LENGTH: GLenum = 0x0D02;
    pub const GL_PACK_SKIP_ROWS: GLenum = 0x0D03;
    pub const GL_PACK_SKIP_PIXELS: GLenum = 0x0D04;
    pub const GL_PACK_ALIGNMENT: GLenum = 0x0D05;
    pub const GL_MAX_CLIP_DISTANCES: GLenum = 0x0D32;
    pub const GL_MAX_TEXTURE_SIZE: GLenum = 0x0D33;
    pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
    pub const GL_SUBPIXEL_BITS: GLenum = 0x0D50;
    pub const GL_TEXTURE_1D: GLenum = 0x0DE0;
    pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
    pub const GL_TEXTURE_WIDTH: GLenum = 0x1000;
    pub const GL_TEXTURE_HEIGHT: GLenum = 0x1001;
    pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
    pub const GL_TEXTURE_BORDER_COLOR: GLenum = 0x1004;
    pub const GL_TEXTURE_TARGET: GLenum = 0x1006;
    pub const GL_DONT_CARE: GLenum = 0x1100;
    pub const GL_FASTEST: GLenum = 0x1101;
    pub const GL_NICEST: GLenum = 0x1102;
    pub const GL_BYTE: GLenum = 0x1400;
    pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
    pub const GL_SHORT: GLenum = 0x1402;
    pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
    pub const GL_INT: GLenum = 0x1404;
    pub const GL_UNSIGNED_INT: GLenum = 0x1405;
    pub const GL_FLOAT: GLenum = 0x1406;
    pub const GL_DOUBLE: GLenum = 0x140A;
    pub const GL_HALF_FLOAT: GLenum = 0x140B;
    pub const GL_FIXED: GLenum = 0x140C;
    pub const GL_CLEAR: GLenum = 0x1500;
    pub const GL_AND: GLenum = 0x1501;
    pub const GL_AND_REVERSE: GLenum = 0x1502;
    pub const GL_COPY: GLenum = 0x1503;
    pub const GL_AND_INVERTED: GLenum = 0x1504;
    pub const GL_NOOP: GLenum = 0x1505;
    pub const GL_XOR: GLenum = 0x1506;
    pub const GL_OR: GLenum = 0x1507;
    pub const GL_NOR: GLenum = 0x1508;
    pub const GL_EQUIV: GLenum = 0x1509;
    pub const GL_INVERT: GLenum = 0x150A;
    pub const GL_OR_REVERSE: GLenum = 0x150B;
    pub const GL_COPY_INVERTED: GLenum = 0x150C;
    pub const GL_OR_INVERTED: GLenum = 0x150D;
    pub const GL_NAND: GLenum = 0x150E;
    pub const GL_SET: GLenum = 0x150F;
    pub const GL_TEXTURE: GLenum = 0x1702;
    pub const GL_COLOR: GLenum = 0x1800;
    pub const GL_DEPTH: GLenum = 0x1801;
    pub const GL_STENCIL: GLenum = 0x1802;
    pub const GL_STENCIL_INDEX: GLenum = 0x1901;
    pub const GL_DEPTH_COMPONENT: GLenum = 0x1902;
    pub const GL_RED: GLenum = 0x1903;
    pub const GL_GREEN: GLenum = 0x1904;
    pub const GL_BLUE: GLenum = 0x1905;
    pub const GL_ALPHA: GLenum = 0x1906;
    pub const GL_RGB: GLenum = 0x1907;
    pub const GL_RGBA: GLenum = 0x1908;
    pub const GL_POINT: GLenum = 0x1B00;
    pub const GL_LINE: GLenum = 0x1B01;
    pub const GL_FILL: GLenum = 0x1B02;
    pub const GL_KEEP: GLenum = 0x1E00;
    pub const GL_REPLACE: GLenum = 0x1E01;
    pub const GL_INCR: GLenum = 0x1E02;
    pub const GL_DECR: GLenum = 0x1E03;
    pub const GL_VENDOR: GLenum = 0x1F00;
    pub const GL_RENDERER: GLenum = 0x1F01;
    pub const GL_VERSION: GLenum = 0x1F02;
    pub const GL_EXTENSIONS: GLenum = 0x1F03;
    pub const GL_NEAREST: GLenum = 0x2600;
    pub const GL_LINEAR: GLenum = 0x2601;
    pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
    pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
    pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
    pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
    pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
    pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
    pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
    pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
    pub const GL_REPEAT: GLenum = 0x2901;
    pub const GL_POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
    pub const GL_POLYGON_OFFSET_POINT: GLenum = 0x2A01;
    pub const GL_POLYGON_OFFSET_LINE: GLenum = 0x2A02;
    pub const GL_R3_G3_B2: GLenum = 0x2A10;
    pub const GL_CLIP_DISTANCE0: GLenum = 0x3000;
    pub const GL_CLIP_DISTANCE1: GLenum = 0x3001;
    pub const GL_CLIP_DISTANCE2: GLenum = 0x3002;
    pub const GL_CLIP_DISTANCE3: GLenum = 0x3003;
    pub const GL_CLIP_DISTANCE4: GLenum = 0x3004;
    pub const GL_CLIP_DISTANCE5: GLenum = 0x3005;
    pub const GL_CLIP_DISTANCE6: GLenum = 0x3006;
    pub const GL_CLIP_DISTANCE7: GLenum = 0x3007;
    pub const GL_CONSTANT_COLOR: GLenum = 0x8001;
    pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
    pub const GL_CONSTANT_ALPHA: GLenum = 0x8003;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
    pub const GL_BLEND_COLOR: GLenum = 0x8005;
    pub const GL_FUNC_ADD: GLenum = 0x8006;
    pub const GL_MIN: GLenum = 0x8007;
    pub const GL_MAX: GLenum = 0x8008;
    pub const GL_BLEND_EQUATION: GLenum = 0x8009;
    pub const GL_BLEND_EQUATION_RGB: GLenum = 0x8009;
    pub const GL_FUNC_SUBTRACT: GLenum = 0x800A;
    pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
    pub const GL_UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
    pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
    pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
    pub const GL_UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
    pub const GL_UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
    pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;
    pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
    pub const GL_RGB4: GLenum = 0x804F;
    pub const GL_RGB5: GLenum = 0x8050;
    pub const GL_RGB8: GLenum = 0x8051;
    pub const GL_RGB10: GLenum = 0x8052;
    pub const GL_RGB12: GLenum = 0x8053;
    pub const GL_RGB16: GLenum = 0x8054;
    pub const GL_RGBA2: GLenum = 0x8055;
    pub const GL_RGBA4: GLenum = 0x8056;
    pub const GL_RGB5_A1: GLenum = 0x8057;
    pub const GL_RGBA8: GLenum = 0x8058;
    pub const GL_RGB10_A2: GLenum = 0x8059;
    pub const GL_RGBA12: GLenum = 0x805A;
    pub const GL_RGBA16: GLenum = 0x805B;
    pub const GL_TEXTURE_RED_SIZE: GLenum = 0x805C;
    pub const GL_TEXTURE_GREEN_SIZE: GLenum = 0x805D;
    pub const GL_TEXTURE_BLUE_SIZE: GLenum = 0x805E;
    pub const GL_TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
    pub const GL_PROXY_TEXTURE_1D: GLenum = 0x8063;
    pub const GL_PROXY_TEXTURE_2D: GLenum = 0x8064;
    pub const GL_TEXTURE_BINDING_1D: GLenum = 0x8068;
    pub const GL_TEXTURE_BINDING_2D: GLenum = 0x8069;
    pub const GL_TEXTURE_BINDING_3D: GLenum = 0x806A;
    pub const GL_PACK_SKIP_IMAGES: GLenum = 0x806B;
    pub const GL_PACK_IMAGE_HEIGHT: GLenum = 0x806C;
    pub const GL_UNPACK_SKIP_IMAGES: GLenum = 0x806D;
    pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
    pub const GL_TEXTURE_3D: GLenum = 0x806F;
    pub const GL_PROXY_TEXTURE_3D: GLenum = 0x8070;
    pub const GL_TEXTURE_DEPTH: GLenum = 0x8071;
    pub const GL_TEXTURE_WRAP_R: GLenum = 0x8072;
    pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
    pub const GL_VERTEX_ARRAY: GLenum = 0x8074;
    pub const GL_MULTISAMPLE: GLenum = 0x809D;
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
    pub const GL_SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
    pub const GL_SAMPLE_COVERAGE: GLenum = 0x80A0;
    pub const GL_SAMPLE_BUFFERS: GLenum = 0x80A8;
    pub const GL_SAMPLES: GLenum = 0x80A9;
    pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
    pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
    pub const GL_BLEND_DST_RGB: GLenum = 0x80C8;
    pub const GL_BLEND_SRC_RGB: GLenum = 0x80C9;
    pub const GL_BLEND_DST_ALPHA: GLenum = 0x80CA;
    pub const GL_BLEND_SRC_ALPHA: GLenum = 0x80CB;
    pub const GL_BGR: GLenum = 0x80E0;
    pub const GL_BGRA: GLenum = 0x80E1;
    pub const GL_MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
    pub const GL_MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
    pub const GL_PARAMETER_BUFFER: GLenum = 0x80EE;
    pub const GL_PARAMETER_BUFFER_BINDING: GLenum = 0x80EF;
    pub const GL_POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
    pub const GL_CLAMP_TO_BORDER: GLenum = 0x812D;
    pub const GL_CLAMP_TO_EDGE: GLenum = 0x812F;
    pub const GL_TEXTURE_MIN_LOD: GLenum = 0x813A;
    pub const GL_TEXTURE_MAX_LOD: GLenum = 0x813B;
    pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
    pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
    pub const GL_DEPTH_COMPONENT16: GLenum = 0x81A5;
    pub const GL_DEPTH_COMPONENT24: GLenum = 0x81A6;
    pub const GL_DEPTH_COMPONENT32: GLenum = 0x81A7;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
    pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
    pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
    pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
    pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
    pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
    pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
    pub const GL_FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
    pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
    pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
    pub const GL_MAJOR_VERSION: GLenum = 0x821B;
    pub const GL_MINOR_VERSION: GLenum = 0x821C;
    pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;
    pub const GL_CONTEXT_FLAGS: GLenum = 0x821E;
    pub const GL_BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
    pub const GL_BUFFER_STORAGE_FLAGS: GLenum = 0x8220;
    pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
    pub const GL_COMPRESSED_RED: GLenum = 0x8225;
    pub const GL_COMPRESSED_RG: GLenum = 0x8226;
    pub const GL_RG: GLenum = 0x8227;
    pub const GL_RG_INTEGER: GLenum = 0x8228;
    pub const GL_R8: GLenum = 0x8229;
    pub const GL_R16: GLenum = 0x822A;
    pub const GL_RG8: GLenum = 0x822B;
    pub const GL_RG16: GLenum = 0x822C;
    pub const GL_R16F: GLenum = 0x822D;
    pub const GL_R32F: GLenum = 0x822E;
    pub const GL_RG16F: GLenum = 0x822F;
    pub const GL_RG32F: GLenum = 0x8230;
    pub const GL_R8I: GLenum = 0x8231;
    pub const GL_R8UI: GLenum = 0x8232;
    pub const GL_R16I: GLenum = 0x8233;
    pub const GL_R16UI: GLenum = 0x8234;
    pub const GL_R32I: GLenum = 0x8235;
    pub const GL_R32UI: GLenum = 0x8236;
    pub const GL_RG8I: GLenum = 0x8237;
    pub const GL_RG8UI: GLenum = 0x8238;
    pub const GL_RG16I: GLenum = 0x8239;
    pub const GL_RG16UI: GLenum = 0x823A;
    pub const GL_RG32I: GLenum = 0x823B;
    pub const GL_RG32UI: GLenum = 0x823C;
    pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
    pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
    pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
    pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
    pub const GL_DEBUG_SOURCE_API: GLenum = 0x8246;
    pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
    pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
    pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
    pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
    pub const GL_DEBUG_SOURCE_OTHER: GLenum = 0x824B;
    pub const GL_DEBUG_TYPE_ERROR: GLenum = 0x824C;
    pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
    pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
    pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
    pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
    pub const GL_DEBUG_TYPE_OTHER: GLenum = 0x8251;
    pub const GL_LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
    pub const GL_GUILTY_CONTEXT_RESET: GLenum = 0x8253;
    pub const GL_INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
    pub const GL_UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
    pub const GL_RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
    pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
    pub const GL_PROGRAM_SEPARABLE: GLenum = 0x8258;
    pub const GL_ACTIVE_PROGRAM: GLenum = 0x8259;
    pub const GL_PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
    pub const GL_MAX_VIEWPORTS: GLenum = 0x825B;
    pub const GL_VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
    pub const GL_VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
    pub const GL_LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
    pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
    pub const GL_UNDEFINED_VERTEX: GLenum = 0x8260;
    pub const GL_NO_RESET_NOTIFICATION: GLenum = 0x8261;
    pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
    pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
    pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
    pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
    pub const GL_DEBUG_TYPE_MARKER: GLenum = 0x8268;
    pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
    pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
    pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
    pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
    pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
    pub const GL_MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
    pub const GL_INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
    pub const GL_INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
    pub const GL_INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
    pub const GL_INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
    pub const GL_INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
    pub const GL_INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
    pub const GL_INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
    pub const GL_INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
    pub const GL_INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
    pub const GL_INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
    pub const GL_INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
    pub const GL_INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
    pub const GL_INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
    pub const GL_INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
    pub const GL_INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
    pub const GL_MAX_WIDTH: GLenum = 0x827E;
    pub const GL_MAX_HEIGHT: GLenum = 0x827F;
    pub const GL_MAX_DEPTH: GLenum = 0x8280;
    pub const GL_MAX_LAYERS: GLenum = 0x8281;
    pub const GL_MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
    pub const GL_COLOR_COMPONENTS: GLenum = 0x8283;
    pub const GL_DEPTH_COMPONENTS: GLenum = 0x8284;
    pub const GL_STENCIL_COMPONENTS: GLenum = 0x8285;
    pub const GL_COLOR_RENDERABLE: GLenum = 0x8286;
    pub const GL_DEPTH_RENDERABLE: GLenum = 0x8287;
    pub const GL_STENCIL_RENDERABLE: GLenum = 0x8288;
    pub const GL_FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
    pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
    pub const GL_FRAMEBUFFER_BLEND: GLenum = 0x828B;
    pub const GL_READ_PIXELS: GLenum = 0x828C;
    pub const GL_READ_PIXELS_FORMAT: GLenum = 0x828D;
    pub const GL_READ_PIXELS_TYPE: GLenum = 0x828E;
    pub const GL_TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
    pub const GL_TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
    pub const GL_GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
    pub const GL_GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
    pub const GL_MIPMAP: GLenum = 0x8293;
    pub const GL_MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
    pub const GL_AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
    pub const GL_COLOR_ENCODING: GLenum = 0x8296;
    pub const GL_SRGB_READ: GLenum = 0x8297;
    pub const GL_SRGB_WRITE: GLenum = 0x8298;
    pub const GL_FILTER: GLenum = 0x829A;
    pub const GL_VERTEX_TEXTURE: GLenum = 0x829B;
    pub const GL_TESS_CONTROL_TEXTURE: GLenum = 0x829C;
    pub const GL_TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
    pub const GL_GEOMETRY_TEXTURE: GLenum = 0x829E;
    pub const GL_FRAGMENT_TEXTURE: GLenum = 0x829F;
    pub const GL_COMPUTE_TEXTURE: GLenum = 0x82A0;
    pub const GL_TEXTURE_SHADOW: GLenum = 0x82A1;
    pub const GL_TEXTURE_GATHER: GLenum = 0x82A2;
    pub const GL_TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
    pub const GL_SHADER_IMAGE_LOAD: GLenum = 0x82A4;
    pub const GL_SHADER_IMAGE_STORE: GLenum = 0x82A5;
    pub const GL_SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
    pub const GL_IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
    pub const GL_IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
    pub const GL_IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
    pub const GL_IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
    pub const GL_CLEAR_BUFFER: GLenum = 0x82B4;
    pub const GL_TEXTURE_VIEW: GLenum = 0x82B5;
    pub const GL_VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
    pub const GL_FULL_SUPPORT: GLenum = 0x82B7;
    pub const GL_CAVEAT_SUPPORT: GLenum = 0x82B8;
    pub const GL_IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
    pub const GL_IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
    pub const GL_IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
    pub const GL_IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
    pub const GL_IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
    pub const GL_IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
    pub const GL_IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
    pub const GL_IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
    pub const GL_IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
    pub const GL_IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
    pub const GL_IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
    pub const GL_VIEW_CLASS_128_BITS: GLenum = 0x82C4;
    pub const GL_VIEW_CLASS_96_BITS: GLenum = 0x82C5;
    pub const GL_VIEW_CLASS_64_BITS: GLenum = 0x82C6;
    pub const GL_VIEW_CLASS_48_BITS: GLenum = 0x82C7;
    pub const GL_VIEW_CLASS_32_BITS: GLenum = 0x82C8;
    pub const GL_VIEW_CLASS_24_BITS: GLenum = 0x82C9;
    pub const GL_VIEW_CLASS_16_BITS: GLenum = 0x82CA;
    pub const GL_VIEW_CLASS_8_BITS: GLenum = 0x82CB;
    pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
    pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
    pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
    pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
    pub const GL_VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
    pub const GL_VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
    pub const GL_VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
    pub const GL_VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
    pub const GL_VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
    pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
    pub const GL_VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
    pub const GL_VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
    pub const GL_VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
    pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
    pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
    pub const GL_TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
    pub const GL_TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
    pub const GL_TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
    pub const GL_TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
    pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
    pub const GL_BUFFER: GLenum = 0x82E0;
    pub const GL_SHADER: GLenum = 0x82E1;
    pub const GL_PROGRAM: GLenum = 0x82E2;
    pub const GL_QUERY: GLenum = 0x82E3;
    pub const GL_PROGRAM_PIPELINE: GLenum = 0x82E4;
    pub const GL_MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
    pub const GL_SAMPLER: GLenum = 0x82E6;
    pub const GL_MAX_LABEL_LENGTH: GLenum = 0x82E8;
    pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
    pub const GL_QUERY_TARGET: GLenum = 0x82EA;
    pub const GL_TRANSFORM_FEEDBACK_OVERFLOW: GLenum = 0x82EC;
    pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GLenum = 0x82ED;
    pub const GL_VERTICES_SUBMITTED: GLenum = 0x82EE;
    pub const GL_PRIMITIVES_SUBMITTED: GLenum = 0x82EF;
    pub const GL_VERTEX_SHADER_INVOCATIONS: GLenum = 0x82F0;
    pub const GL_TESS_CONTROL_SHADER_PATCHES: GLenum = 0x82F1;
    pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS: GLenum = 0x82F2;
    pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED: GLenum = 0x82F3;
    pub const GL_FRAGMENT_SHADER_INVOCATIONS: GLenum = 0x82F4;
    pub const GL_COMPUTE_SHADER_INVOCATIONS: GLenum = 0x82F5;
    pub const GL_CLIPPING_INPUT_PRIMITIVES: GLenum = 0x82F6;
    pub const GL_CLIPPING_OUTPUT_PRIMITIVES: GLenum = 0x82F7;
    pub const GL_MAX_CULL_DISTANCES: GLenum = 0x82F9;
    pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;
    pub const GL_CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
    pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;
    pub const GL_UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
    pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
    pub const GL_UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
    pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
    pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
    pub const GL_UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
    pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
    pub const GL_MIRRORED_REPEAT: GLenum = 0x8370;
    pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
    pub const GL_TEXTURE0: GLenum = 0x84C0;
    pub const GL_TEXTURE1: GLenum = 0x84C1;
    pub const GL_TEXTURE2: GLenum = 0x84C2;
    pub const GL_TEXTURE3: GLenum = 0x84C3;
    pub const GL_TEXTURE4: GLenum = 0x84C4;
    pub const GL_TEXTURE5: GLenum = 0x84C5;
    pub const GL_TEXTURE6: GLenum = 0x84C6;
    pub const GL_TEXTURE7: GLenum = 0x84C7;
    pub const GL_TEXTURE8: GLenum = 0x84C8;
    pub const GL_TEXTURE9: GLenum = 0x84C9;
    pub const GL_TEXTURE10: GLenum = 0x84CA;
    pub const GL_TEXTURE11: GLenum = 0x84CB;
    pub const GL_TEXTURE12: GLenum = 0x84CC;
    pub const GL_TEXTURE13: GLenum = 0x84CD;
    pub const GL_TEXTURE14: GLenum = 0x84CE;
    pub const GL_TEXTURE15: GLenum = 0x84CF;
    pub const GL_TEXTURE16: GLenum = 0x84D0;
    pub const GL_TEXTURE17: GLenum = 0x84D1;
    pub const GL_TEXTURE18: GLenum = 0x84D2;
    pub const GL_TEXTURE19: GLenum = 0x84D3;
    pub const GL_TEXTURE20: GLenum = 0x84D4;
    pub const GL_TEXTURE21: GLenum = 0x84D5;
    pub const GL_TEXTURE22: GLenum = 0x84D6;
    pub const GL_TEXTURE23: GLenum = 0x84D7;
    pub const GL_TEXTURE24: GLenum = 0x84D8;
    pub const GL_TEXTURE25: GLenum = 0x84D9;
    pub const GL_TEXTURE26: GLenum = 0x84DA;
    pub const GL_TEXTURE27: GLenum = 0x84DB;
    pub const GL_TEXTURE28: GLenum = 0x84DC;
    pub const GL_TEXTURE29: GLenum = 0x84DD;
    pub const GL_TEXTURE30: GLenum = 0x84DE;
    pub const GL_TEXTURE31: GLenum = 0x84DF;
    pub const GL_ACTIVE_TEXTURE: GLenum = 0x84E0;
    pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
    pub const GL_COMPRESSED_RGB: GLenum = 0x84ED;
    pub const GL_COMPRESSED_RGBA: GLenum = 0x84EE;
    pub const GL_TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
    pub const GL_TEXTURE_RECTANGLE: GLenum = 0x84F5;
    pub const GL_TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
    pub const GL_PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
    pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
    pub const GL_DEPTH_STENCIL: GLenum = 0x84F9;
    pub const GL_UNSIGNED_INT_24_8: GLenum = 0x84FA;
    pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
    pub const GL_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FE;
    pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FF;
    pub const GL_TEXTURE_LOD_BIAS: GLenum = 0x8501;
    pub const GL_INCR_WRAP: GLenum = 0x8507;
    pub const GL_DECR_WRAP: GLenum = 0x8508;
    pub const GL_TEXTURE_CUBE_MAP: GLenum = 0x8513;
    pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
    pub const GL_PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
    pub const GL_SRC1_ALPHA: GLenum = 0x8589;
    pub const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
    pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
    pub const GL_VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
    pub const GL_PROGRAM_POINT_SIZE: GLenum = 0x8642;
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
    pub const GL_DEPTH_CLAMP: GLenum = 0x864F;
    pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
    pub const GL_TEXTURE_COMPRESSED: GLenum = 0x86A1;
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
    pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
    pub const GL_PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
    pub const GL_MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;
    pub const GL_VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
    pub const GL_BUFFER_SIZE: GLenum = 0x8764;
    pub const GL_BUFFER_USAGE: GLenum = 0x8765;
    pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
    pub const GL_PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
    pub const GL_STENCIL_BACK_FUNC: GLenum = 0x8800;
    pub const GL_STENCIL_BACK_FAIL: GLenum = 0x8801;
    pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
    pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
    pub const GL_RGBA32F: GLenum = 0x8814;
    pub const GL_RGB32F: GLenum = 0x8815;
    pub const GL_RGBA16F: GLenum = 0x881A;
    pub const GL_RGB16F: GLenum = 0x881B;
    pub const GL_MAX_DRAW_BUFFERS: GLenum = 0x8824;
    pub const GL_DRAW_BUFFER0: GLenum = 0x8825;
    pub const GL_DRAW_BUFFER1: GLenum = 0x8826;
    pub const GL_DRAW_BUFFER2: GLenum = 0x8827;
    pub const GL_DRAW_BUFFER3: GLenum = 0x8828;
    pub const GL_DRAW_BUFFER4: GLenum = 0x8829;
    pub const GL_DRAW_BUFFER5: GLenum = 0x882A;
    pub const GL_DRAW_BUFFER6: GLenum = 0x882B;
    pub const GL_DRAW_BUFFER7: GLenum = 0x882C;
    pub const GL_DRAW_BUFFER8: GLenum = 0x882D;
    pub const GL_DRAW_BUFFER9: GLenum = 0x882E;
    pub const GL_DRAW_BUFFER10: GLenum = 0x882F;
    pub const GL_DRAW_BUFFER11: GLenum = 0x8830;
    pub const GL_DRAW_BUFFER12: GLenum = 0x8831;
    pub const GL_DRAW_BUFFER13: GLenum = 0x8832;
    pub const GL_DRAW_BUFFER14: GLenum = 0x8833;
    pub const GL_DRAW_BUFFER15: GLenum = 0x8834;
    pub const GL_BLEND_EQUATION_ALPHA: GLenum = 0x883D;
    pub const GL_TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
    pub const GL_TEXTURE_COMPARE_MODE: GLenum = 0x884C;
    pub const GL_TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
    pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
    pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
    pub const GL_QUERY_COUNTER_BITS: GLenum = 0x8864;
    pub const GL_CURRENT_QUERY: GLenum = 0x8865;
    pub const GL_QUERY_RESULT: GLenum = 0x8866;
    pub const GL_QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
    pub const GL_MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
    pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
    pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
    pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
    pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
    pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
    pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
    pub const GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894;
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
    pub const GL_READ_ONLY: GLenum = 0x88B8;
    pub const GL_WRITE_ONLY: GLenum = 0x88B9;
    pub const GL_READ_WRITE: GLenum = 0x88BA;
    pub const GL_BUFFER_ACCESS: GLenum = 0x88BB;
    pub const GL_BUFFER_MAPPED: GLenum = 0x88BC;
    pub const GL_BUFFER_MAP_POINTER: GLenum = 0x88BD;
    pub const GL_TIME_ELAPSED: GLenum = 0x88BF;
    pub const GL_STREAM_DRAW: GLenum = 0x88E0;
    pub const GL_STREAM_READ: GLenum = 0x88E1;
    pub const GL_STREAM_COPY: GLenum = 0x88E2;
    pub const GL_STATIC_DRAW: GLenum = 0x88E4;
    pub const GL_STATIC_READ: GLenum = 0x88E5;
    pub const GL_STATIC_COPY: GLenum = 0x88E6;
    pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
    pub const GL_DYNAMIC_READ: GLenum = 0x88E9;
    pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;
    pub const GL_PIXEL_PACK_BUFFER: GLenum = 0x88EB;
    pub const GL_PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
    pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
    pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
    pub const GL_DEPTH24_STENCIL8: GLenum = 0x88F0;
    pub const GL_TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
    pub const GL_SRC1_COLOR: GLenum = 0x88F9;
    pub const GL_ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
    pub const GL_ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
    pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
    pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
    pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
    pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
    pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
    pub const GL_SAMPLES_PASSED: GLenum = 0x8914;
    pub const GL_GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
    pub const GL_GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
    pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
    pub const GL_SAMPLER_BINDING: GLenum = 0x8919;
    pub const GL_CLAMP_READ_COLOR: GLenum = 0x891C;
    pub const GL_FIXED_ONLY: GLenum = 0x891D;
    pub const GL_UNIFORM_BUFFER: GLenum = 0x8A11;
    pub const GL_UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
    pub const GL_UNIFORM_BUFFER_START: GLenum = 0x8A29;
    pub const GL_UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
    pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
    pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
    pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
    pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
    pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
    pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
    pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
    pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
    pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
    pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
    pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
    pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
    pub const GL_UNIFORM_TYPE: GLenum = 0x8A37;
    pub const GL_UNIFORM_SIZE: GLenum = 0x8A38;
    pub const GL_UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
    pub const GL_UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
    pub const GL_UNIFORM_OFFSET: GLenum = 0x8A3B;
    pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
    pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
    pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
    pub const GL_UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
    pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
    pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
    pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
    pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
    pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
    pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
    pub const GL_MAX_VARYING_FLOATS: GLenum = 0x8B4B;
    pub const GL_MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
    pub const GL_SHADER_TYPE: GLenum = 0x8B4F;
    pub const GL_FLOAT_VEC2: GLenum = 0x8B50;
    pub const GL_FLOAT_VEC3: GLenum = 0x8B51;
    pub const GL_FLOAT_VEC4: GLenum = 0x8B52;
    pub const GL_INT_VEC2: GLenum = 0x8B53;
    pub const GL_INT_VEC3: GLenum = 0x8B54;
    pub const GL_INT_VEC4: GLenum = 0x8B55;
    pub const GL_BOOL: GLenum = 0x8B56;
    pub const GL_BOOL_VEC2: GLenum = 0x8B57;
    pub const GL_BOOL_VEC3: GLenum = 0x8B58;
    pub const GL_BOOL_VEC4: GLenum = 0x8B59;
    pub const GL_FLOAT_MAT2: GLenum = 0x8B5A;
    pub const GL_FLOAT_MAT3: GLenum = 0x8B5B;
    pub const GL_FLOAT_MAT4: GLenum = 0x8B5C;
    pub const GL_SAMPLER_1D: GLenum = 0x8B5D;
    pub const GL_SAMPLER_2D: GLenum = 0x8B5E;
    pub const GL_SAMPLER_3D: GLenum = 0x8B5F;
    pub const GL_SAMPLER_CUBE: GLenum = 0x8B60;
    pub const GL_SAMPLER_1D_SHADOW: GLenum = 0x8B61;
    pub const GL_SAMPLER_2D_SHADOW: GLenum = 0x8B62;
    pub const GL_SAMPLER_2D_RECT: GLenum = 0x8B63;
    pub const GL_SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
    pub const GL_FLOAT_MAT2x3: GLenum = 0x8B65;
    pub const GL_FLOAT_MAT2x4: GLenum = 0x8B66;
    pub const GL_FLOAT_MAT3x2: GLenum = 0x8B67;
    pub const GL_FLOAT_MAT3x4: GLenum = 0x8B68;
    pub const GL_FLOAT_MAT4x2: GLenum = 0x8B69;
    pub const GL_FLOAT_MAT4x3: GLenum = 0x8B6A;
    pub const GL_DELETE_STATUS: GLenum = 0x8B80;
    pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
    pub const GL_LINK_STATUS: GLenum = 0x8B82;
    pub const GL_VALIDATE_STATUS: GLenum = 0x8B83;
    pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
    pub const GL_ATTACHED_SHADERS: GLenum = 0x8B85;
    pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
    pub const GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
    pub const GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
    pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
    pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;
    pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
    pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
    pub const GL_TEXTURE_RED_TYPE: GLenum = 0x8C10;
    pub const GL_TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
    pub const GL_TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
    pub const GL_TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
    pub const GL_TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
    pub const GL_UNSIGNED_NORMALIZED: GLenum = 0x8C17;
    pub const GL_TEXTURE_1D_ARRAY: GLenum = 0x8C18;
    pub const GL_PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
    pub const GL_TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
    pub const GL_PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
    pub const GL_TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
    pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
    pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
    pub const GL_TEXTURE_BUFFER: GLenum = 0x8C2A;
    pub const GL_TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
    pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
    pub const GL_TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
    pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
    pub const GL_ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
    pub const GL_SAMPLE_SHADING: GLenum = 0x8C36;
    pub const GL_MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
    pub const GL_R11F_G11F_B10F: GLenum = 0x8C3A;
    pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
    pub const GL_RGB9_E5: GLenum = 0x8C3D;
    pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
    pub const GL_TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
    pub const GL_SRGB: GLenum = 0x8C40;
    pub const GL_SRGB8: GLenum = 0x8C41;
    pub const GL_SRGB_ALPHA: GLenum = 0x8C42;
    pub const GL_SRGB8_ALPHA8: GLenum = 0x8C43;
    pub const GL_COMPRESSED_SRGB: GLenum = 0x8C48;
    pub const GL_COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
    pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
    pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
    pub const GL_PRIMITIVES_GENERATED: GLenum = 0x8C87;
    pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
    pub const GL_RASTERIZER_DISCARD: GLenum = 0x8C89;
    pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
    pub const GL_INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
    pub const GL_SEPARATE_ATTRIBS: GLenum = 0x8C8D;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
    pub const GL_POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
    pub const GL_LOWER_LEFT: GLenum = 0x8CA1;
    pub const GL_UPPER_LEFT: GLenum = 0x8CA2;
    pub const GL_STENCIL_BACK_REF: GLenum = 0x8CA3;
    pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
    pub const GL_STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
    pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
    pub const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
    pub const GL_RENDERBUFFER_BINDING: GLenum = 0x8CA7;
    pub const GL_READ_FRAMEBUFFER: GLenum = 0x8CA8;
    pub const GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
    pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
    pub const GL_RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
    pub const GL_DEPTH_COMPONENT32F: GLenum = 0x8CAC;
    pub const GL_DEPTH32F_STENCIL8: GLenum = 0x8CAD;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
    pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
    pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
    pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
    pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
    pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
    pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
    pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
    pub const GL_COLOR_ATTACHMENT1: GLenum = 0x8CE1;
    pub const GL_COLOR_ATTACHMENT2: GLenum = 0x8CE2;
    pub const GL_COLOR_ATTACHMENT3: GLenum = 0x8CE3;
    pub const GL_COLOR_ATTACHMENT4: GLenum = 0x8CE4;
    pub const GL_COLOR_ATTACHMENT5: GLenum = 0x8CE5;
    pub const GL_COLOR_ATTACHMENT6: GLenum = 0x8CE6;
    pub const GL_COLOR_ATTACHMENT7: GLenum = 0x8CE7;
    pub const GL_COLOR_ATTACHMENT8: GLenum = 0x8CE8;
    pub const GL_COLOR_ATTACHMENT9: GLenum = 0x8CE9;
    pub const GL_COLOR_ATTACHMENT10: GLenum = 0x8CEA;
    pub const GL_COLOR_ATTACHMENT11: GLenum = 0x8CEB;
    pub const GL_COLOR_ATTACHMENT12: GLenum = 0x8CEC;
    pub const GL_COLOR_ATTACHMENT13: GLenum = 0x8CED;
    pub const GL_COLOR_ATTACHMENT14: GLenum = 0x8CEE;
    pub const GL_COLOR_ATTACHMENT15: GLenum = 0x8CEF;
    pub const GL_COLOR_ATTACHMENT16: GLenum = 0x8CF0;
    pub const GL_COLOR_ATTACHMENT17: GLenum = 0x8CF1;
    pub const GL_COLOR_ATTACHMENT18: GLenum = 0x8CF2;
    pub const GL_COLOR_ATTACHMENT19: GLenum = 0x8CF3;
    pub const GL_COLOR_ATTACHMENT20: GLenum = 0x8CF4;
    pub const GL_COLOR_ATTACHMENT21: GLenum = 0x8CF5;
    pub const GL_COLOR_ATTACHMENT22: GLenum = 0x8CF6;
    pub const GL_COLOR_ATTACHMENT23: GLenum = 0x8CF7;
    pub const GL_COLOR_ATTACHMENT24: GLenum = 0x8CF8;
    pub const GL_COLOR_ATTACHMENT25: GLenum = 0x8CF9;
    pub const GL_COLOR_ATTACHMENT26: GLenum = 0x8CFA;
    pub const GL_COLOR_ATTACHMENT27: GLenum = 0x8CFB;
    pub const GL_COLOR_ATTACHMENT28: GLenum = 0x8CFC;
    pub const GL_COLOR_ATTACHMENT29: GLenum = 0x8CFD;
    pub const GL_COLOR_ATTACHMENT30: GLenum = 0x8CFE;
    pub const GL_COLOR_ATTACHMENT31: GLenum = 0x8CFF;
    pub const GL_DEPTH_ATTACHMENT: GLenum = 0x8D00;
    pub const GL_STENCIL_ATTACHMENT: GLenum = 0x8D20;
    pub const GL_FRAMEBUFFER: GLenum = 0x8D40;
    pub const GL_RENDERBUFFER: GLenum = 0x8D41;
    pub const GL_RENDERBUFFER_WIDTH: GLenum = 0x8D42;
    pub const GL_RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
    pub const GL_STENCIL_INDEX1: GLenum = 0x8D46;
    pub const GL_STENCIL_INDEX4: GLenum = 0x8D47;
    pub const GL_STENCIL_INDEX8: GLenum = 0x8D48;
    pub const GL_STENCIL_INDEX16: GLenum = 0x8D49;
    pub const GL_RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
    pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
    pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
    pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
    pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
    pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
    pub const GL_MAX_SAMPLES: GLenum = 0x8D57;
    pub const GL_RGB565: GLenum = 0x8D62;
    pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
    pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
    pub const GL_MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
    pub const GL_RGBA32UI: GLenum = 0x8D70;
    pub const GL_RGB32UI: GLenum = 0x8D71;
    pub const GL_RGBA16UI: GLenum = 0x8D76;
    pub const GL_RGB16UI: GLenum = 0x8D77;
    pub const GL_RGBA8UI: GLenum = 0x8D7C;
    pub const GL_RGB8UI: GLenum = 0x8D7D;
    pub const GL_RGBA32I: GLenum = 0x8D82;
    pub const GL_RGB32I: GLenum = 0x8D83;
    pub const GL_RGBA16I: GLenum = 0x8D88;
    pub const GL_RGB16I: GLenum = 0x8D89;
    pub const GL_RGBA8I: GLenum = 0x8D8E;
    pub const GL_RGB8I: GLenum = 0x8D8F;
    pub const GL_RED_INTEGER: GLenum = 0x8D94;
    pub const GL_GREEN_INTEGER: GLenum = 0x8D95;
    pub const GL_BLUE_INTEGER: GLenum = 0x8D96;
    pub const GL_RGB_INTEGER: GLenum = 0x8D98;
    pub const GL_RGBA_INTEGER: GLenum = 0x8D99;
    pub const GL_BGR_INTEGER: GLenum = 0x8D9A;
    pub const GL_BGRA_INTEGER: GLenum = 0x8D9B;
    pub const GL_INT_2_10_10_10_REV: GLenum = 0x8D9F;
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
    pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
    pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
    pub const GL_FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
    pub const GL_COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
    pub const GL_COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
    pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
    pub const GL_SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
    pub const GL_SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
    pub const GL_SAMPLER_BUFFER: GLenum = 0x8DC2;
    pub const GL_SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
    pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
    pub const GL_SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
    pub const GL_UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
    pub const GL_UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
    pub const GL_UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
    pub const GL_INT_SAMPLER_1D: GLenum = 0x8DC9;
    pub const GL_INT_SAMPLER_2D: GLenum = 0x8DCA;
    pub const GL_INT_SAMPLER_3D: GLenum = 0x8DCB;
    pub const GL_INT_SAMPLER_CUBE: GLenum = 0x8DCC;
    pub const GL_INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
    pub const GL_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
    pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
    pub const GL_INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
    pub const GL_UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
    pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
    pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
    pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
    pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
    pub const GL_GEOMETRY_SHADER: GLenum = 0x8DD9;
    pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
    pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
    pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
    pub const GL_ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
    pub const GL_MAX_SUBROUTINES: GLenum = 0x8DE7;
    pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
    pub const GL_LOW_FLOAT: GLenum = 0x8DF0;
    pub const GL_MEDIUM_FLOAT: GLenum = 0x8DF1;
    pub const GL_HIGH_FLOAT: GLenum = 0x8DF2;
    pub const GL_LOW_INT: GLenum = 0x8DF3;
    pub const GL_MEDIUM_INT: GLenum = 0x8DF4;
    pub const GL_HIGH_INT: GLenum = 0x8DF5;
    pub const GL_SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
    pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
    pub const GL_SHADER_COMPILER: GLenum = 0x8DFA;
    pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
    pub const GL_MAX_VARYING_VECTORS: GLenum = 0x8DFC;
    pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
    pub const GL_QUERY_WAIT: GLenum = 0x8E13;
    pub const GL_QUERY_NO_WAIT: GLenum = 0x8E14;
    pub const GL_QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
    pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
    pub const GL_QUERY_WAIT_INVERTED: GLenum = 0x8E17;
    pub const GL_QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
    pub const GL_QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
    pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;
    pub const GL_POLYGON_OFFSET_CLAMP: GLenum = 0x8E1B;
    pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
    pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
    pub const GL_TRANSFORM_FEEDBACK: GLenum = 0x8E22;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
    pub const GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
    pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
    pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
    pub const GL_TIMESTAMP: GLenum = 0x8E28;
    pub const GL_TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
    pub const GL_TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
    pub const GL_TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
    pub const GL_TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
    pub const GL_TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
    pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
    pub const GL_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
    pub const GL_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
    pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
    pub const GL_FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
    pub const GL_LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
    pub const GL_PROVOKING_VERTEX: GLenum = 0x8E4F;
    pub const GL_SAMPLE_POSITION: GLenum = 0x8E50;
    pub const GL_SAMPLE_MASK: GLenum = 0x8E51;
    pub const GL_SAMPLE_MASK_VALUE: GLenum = 0x8E52;
    pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
    pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
    pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
    pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
    pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
    pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
    pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
    pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;
    pub const GL_MAX_VERTEX_STREAMS: GLenum = 0x8E71;
    pub const GL_PATCH_VERTICES: GLenum = 0x8E72;
    pub const GL_PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
    pub const GL_PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
    pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
    pub const GL_TESS_GEN_MODE: GLenum = 0x8E76;
    pub const GL_TESS_GEN_SPACING: GLenum = 0x8E77;
    pub const GL_TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
    pub const GL_TESS_GEN_POINT_MODE: GLenum = 0x8E79;
    pub const GL_ISOLINES: GLenum = 0x8E7A;
    pub const GL_FRACTIONAL_ODD: GLenum = 0x8E7B;
    pub const GL_FRACTIONAL_EVEN: GLenum = 0x8E7C;
    pub const GL_MAX_PATCH_VERTICES: GLenum = 0x8E7D;
    pub const GL_MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
    pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
    pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
    pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
    pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
    pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
    pub const GL_MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
    pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
    pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
    pub const GL_TESS_EVALUATION_SHADER: GLenum = 0x8E87;
    pub const GL_TESS_CONTROL_SHADER: GLenum = 0x8E88;
    pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
    pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;
    pub const GL_COPY_READ_BUFFER: GLenum = 0x8F36;
    pub const GL_COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
    pub const GL_COPY_WRITE_BUFFER: GLenum = 0x8F37;
    pub const GL_COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
    pub const GL_MAX_IMAGE_UNITS: GLenum = 0x8F38;
    pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
    pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
    pub const GL_IMAGE_BINDING_NAME: GLenum = 0x8F3A;
    pub const GL_IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
    pub const GL_IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
    pub const GL_IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
    pub const GL_IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
    pub const GL_DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
    pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
    pub const GL_DOUBLE_MAT2: GLenum = 0x8F46;
    pub const GL_DOUBLE_MAT3: GLenum = 0x8F47;
    pub const GL_DOUBLE_MAT4: GLenum = 0x8F48;
    pub const GL_DOUBLE_MAT2x3: GLenum = 0x8F49;
    pub const GL_DOUBLE_MAT2x4: GLenum = 0x8F4A;
    pub const GL_DOUBLE_MAT3x2: GLenum = 0x8F4B;
    pub const GL_DOUBLE_MAT3x4: GLenum = 0x8F4C;
    pub const GL_DOUBLE_MAT4x2: GLenum = 0x8F4D;
    pub const GL_DOUBLE_MAT4x3: GLenum = 0x8F4E;
    pub const GL_VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
    pub const GL_R8_SNORM: GLenum = 0x8F94;
    pub const GL_RG8_SNORM: GLenum = 0x8F95;
    pub const GL_RGB8_SNORM: GLenum = 0x8F96;
    pub const GL_RGBA8_SNORM: GLenum = 0x8F97;
    pub const GL_R16_SNORM: GLenum = 0x8F98;
    pub const GL_RG16_SNORM: GLenum = 0x8F99;
    pub const GL_RGB16_SNORM: GLenum = 0x8F9A;
    pub const GL_RGBA16_SNORM: GLenum = 0x8F9B;
    pub const GL_SIGNED_NORMALIZED: GLenum = 0x8F9C;
    pub const GL_PRIMITIVE_RESTART: GLenum = 0x8F9D;
    pub const GL_PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
    pub const GL_DOUBLE_VEC2: GLenum = 0x8FFC;
    pub const GL_DOUBLE_VEC3: GLenum = 0x8FFD;
    pub const GL_DOUBLE_VEC4: GLenum = 0x8FFE;
    pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
    pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
    pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
    pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
    pub const GL_IMAGE_1D: GLenum = 0x904C;
    pub const GL_IMAGE_2D: GLenum = 0x904D;
    pub const GL_IMAGE_3D: GLenum = 0x904E;
    pub const GL_IMAGE_2D_RECT: GLenum = 0x904F;
    pub const GL_IMAGE_CUBE: GLenum = 0x9050;
    pub const GL_IMAGE_BUFFER: GLenum = 0x9051;
    pub const GL_IMAGE_1D_ARRAY: GLenum = 0x9052;
    pub const GL_IMAGE_2D_ARRAY: GLenum = 0x9053;
    pub const GL_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
    pub const GL_IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
    pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
    pub const GL_INT_IMAGE_1D: GLenum = 0x9057;
    pub const GL_INT_IMAGE_2D: GLenum = 0x9058;
    pub const GL_INT_IMAGE_3D: GLenum = 0x9059;
    pub const GL_INT_IMAGE_2D_RECT: GLenum = 0x905A;
    pub const GL_INT_IMAGE_CUBE: GLenum = 0x905B;
    pub const GL_INT_IMAGE_BUFFER: GLenum = 0x905C;
    pub const GL_INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
    pub const GL_INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
    pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
    pub const GL_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
    pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
    pub const GL_UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
    pub const GL_UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
    pub const GL_UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
    pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
    pub const GL_UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
    pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
    pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
    pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
    pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
    pub const GL_MAX_IMAGE_SAMPLES: GLenum = 0x906D;
    pub const GL_IMAGE_BINDING_FORMAT: GLenum = 0x906E;
    pub const GL_RGB10_A2UI: GLenum = 0x906F;
    pub const GL_MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
    pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
    pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
    pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
    pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
    pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
    pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
    pub const GL_SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
    pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
    pub const GL_SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
    pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
    pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
    pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
    pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
    pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
    pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
    pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
    pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
    pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
    pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
    pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
    pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
    pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
    pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
    pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
    pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
    pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
    pub const GL_TEXTURE_SAMPLES: GLenum = 0x9106;
    pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
    pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
    pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
    pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
    pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
    pub const GL_MAX_INTEGER_SAMPLES: GLenum = 0x9110;
    pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
    pub const GL_OBJECT_TYPE: GLenum = 0x9112;
    pub const GL_SYNC_CONDITION: GLenum = 0x9113;
    pub const GL_SYNC_STATUS: GLenum = 0x9114;
    pub const GL_SYNC_FLAGS: GLenum = 0x9115;
    pub const GL_SYNC_FENCE: GLenum = 0x9116;
    pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
    pub const GL_UNSIGNALED: GLenum = 0x9118;
    pub const GL_SIGNALED: GLenum = 0x9119;
    pub const GL_ALREADY_SIGNALED: GLenum = 0x911A;
    pub const GL_TIMEOUT_EXPIRED: GLenum = 0x911B;
    pub const GL_CONDITION_SATISFIED: GLenum = 0x911C;
    pub const GL_WAIT_FAILED: GLenum = 0x911D;
    pub const GL_BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
    pub const GL_BUFFER_MAP_LENGTH: GLenum = 0x9120;
    pub const GL_BUFFER_MAP_OFFSET: GLenum = 0x9121;
    pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
    pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
    pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
    pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
    pub const GL_CONTEXT_PROFILE_MASK: GLenum = 0x9126;
    pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
    pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
    pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
    pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
    pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
    pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
    pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
    pub const GL_PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
    pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
    pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
    pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
    pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
    pub const GL_DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
    pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
    pub const GL_DEBUG_SEVERITY_LOW: GLenum = 0x9148;
    pub const GL_QUERY_BUFFER: GLenum = 0x9192;
    pub const GL_QUERY_BUFFER_BINDING: GLenum = 0x9193;
    pub const GL_QUERY_RESULT_NO_WAIT: GLenum = 0x9194;
    pub const GL_TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
    pub const GL_TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
    pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
    pub const GL_COMPUTE_SHADER: GLenum = 0x91B9;
    pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
    pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
    pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
    pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
    pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
    pub const GL_COMPRESSED_R11_EAC: GLenum = 0x9270;
    pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
    pub const GL_COMPRESSED_RG11_EAC: GLenum = 0x9272;
    pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
    pub const GL_COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
    pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
    pub const GL_ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
    pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
    pub const GL_ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
    pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
    pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
    pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
    pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
    pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
    pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
    pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
    pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
    pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
    pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
    pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
    pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
    pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
    pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
    pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
    pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
    pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
    pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
    pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
    pub const GL_DEBUG_OUTPUT: GLenum = 0x92E0;
    pub const GL_UNIFORM: GLenum = 0x92E1;
    pub const GL_UNIFORM_BLOCK: GLenum = 0x92E2;
    pub const GL_PROGRAM_INPUT: GLenum = 0x92E3;
    pub const GL_PROGRAM_OUTPUT: GLenum = 0x92E4;
    pub const GL_BUFFER_VARIABLE: GLenum = 0x92E5;
    pub const GL_SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
    pub const GL_IS_PER_PATCH: GLenum = 0x92E7;
    pub const GL_VERTEX_SUBROUTINE: GLenum = 0x92E8;
    pub const GL_TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
    pub const GL_TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
    pub const GL_GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
    pub const GL_FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
    pub const GL_COMPUTE_SUBROUTINE: GLenum = 0x92ED;
    pub const GL_VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
    pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
    pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
    pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
    pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
    pub const GL_COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
    pub const GL_TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
    pub const GL_ACTIVE_RESOURCES: GLenum = 0x92F5;
    pub const GL_MAX_NAME_LENGTH: GLenum = 0x92F6;
    pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
    pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
    pub const GL_NAME_LENGTH: GLenum = 0x92F9;
    pub const GL_TYPE: GLenum = 0x92FA;
    pub const GL_ARRAY_SIZE: GLenum = 0x92FB;
    pub const GL_OFFSET: GLenum = 0x92FC;
    pub const GL_BLOCK_INDEX: GLenum = 0x92FD;
    pub const GL_ARRAY_STRIDE: GLenum = 0x92FE;
    pub const GL_MATRIX_STRIDE: GLenum = 0x92FF;
    pub const GL_IS_ROW_MAJOR: GLenum = 0x9300;
    pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
    pub const GL_BUFFER_BINDING: GLenum = 0x9302;
    pub const GL_BUFFER_DATA_SIZE: GLenum = 0x9303;
    pub const GL_NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
    pub const GL_ACTIVE_VARIABLES: GLenum = 0x9305;
    pub const GL_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
    pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
    pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
    pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
    pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
    pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
    pub const GL_TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
    pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
    pub const GL_LOCATION: GLenum = 0x930E;
    pub const GL_LOCATION_INDEX: GLenum = 0x930F;
    pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
    pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
    pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
    pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
    pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
    pub const GL_MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
    pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
    pub const GL_MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
    pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
    pub const GL_LOCATION_COMPONENT: GLenum = 0x934A;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;
    pub const GL_CLIP_ORIGIN: GLenum = 0x935C;
    pub const GL_CLIP_DEPTH_MODE: GLenum = 0x935D;
    pub const GL_NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
    pub const GL_ZERO_TO_ONE: GLenum = 0x935F;
    pub const GL_CLEAR_TEXTURE: GLenum = 0x9365;
    pub const GL_NUM_SAMPLE_COUNTS: GLenum = 0x9380;
    pub const GL_SHADER_BINARY_FORMAT_SPIR_V: GLenum = 0x9551;
    pub const GL_SPIR_V_BINARY: GLenum = 0x9552;
    pub const GL_SPIR_V_EXTENSIONS: GLenum = 0x9553;
    pub const GL_NUM_SPIR_V_EXTENSIONS: GLenum = 0x9554;
}

pub struct Gl {
    glActiveShaderProgram: extern "system" fn(GLuint, GLuint),
    glActiveTexture: extern "system" fn(GLenum),
    glAttachShader: extern "system" fn(GLuint, GLuint),
    glBeginConditionalRender: extern "system" fn(GLuint, GLenum),
    glBeginQuery: extern "system" fn(GLenum, GLuint),
    glBeginQueryIndexed: extern "system" fn(GLenum, GLuint, GLuint),
    glBeginTransformFeedback: extern "system" fn(GLenum),
    glBindAttribLocation: extern "system" fn(GLuint, GLuint, *const GLchar),
    glBindBuffer: extern "system" fn(GLenum, GLuint),
    glBindBufferBase: extern "system" fn(GLenum, GLuint, GLuint),
    glBindBufferRange: extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr),
    glBindBuffersBase: extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint),
    glBindBuffersRange: extern "system" fn(
        GLenum,
        GLuint,
        GLsizei,
        *const GLuint,
        *const GLintptr,
        *const GLsizeiptr,
    ),
    glBindFragDataLocation: extern "system" fn(GLuint, GLuint, *const GLchar),
    glBindFragDataLocationIndexed: extern "system" fn(GLuint, GLuint, GLuint, *const GLchar),
    glBindFramebuffer: extern "system" fn(GLenum, GLuint),
    glBindImageTexture: extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum),
    glBindImageTextures: extern "system" fn(GLuint, GLsizei, *const GLuint),
    glBindProgramPipeline: extern "system" fn(GLuint),
    glBindRenderbuffer: extern "system" fn(GLenum, GLuint),
    glBindSampler: extern "system" fn(GLuint, GLuint),
    glBindSamplers: extern "system" fn(GLuint, GLsizei, *const GLuint),
    glBindTexture: extern "system" fn(GLenum, GLuint),
    glBindTextureUnit: extern "system" fn(GLuint, GLuint),
    glBindTextures: extern "system" fn(GLuint, GLsizei, *const GLuint),
    glBindTransformFeedback: extern "system" fn(GLenum, GLuint),
    glBindVertexArray: extern "system" fn(GLuint),
    glBindVertexBuffer: extern "system" fn(GLuint, GLuint, GLintptr, GLsizei),
    glBindVertexBuffers:
        extern "system" fn(GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei),
    glBlendColor: extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat),
    glBlendEquation: extern "system" fn(GLenum),
    glBlendEquationSeparate: extern "system" fn(GLenum, GLenum),
    glBlendEquationSeparatei: extern "system" fn(GLuint, GLenum, GLenum),
    glBlendEquationi: extern "system" fn(GLuint, GLenum),
    glBlendFunc: extern "system" fn(GLenum, GLenum),
    glBlendFuncSeparate: extern "system" fn(GLenum, GLenum, GLenum, GLenum),
    glBlendFuncSeparatei: extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum),
    glBlendFunci: extern "system" fn(GLuint, GLenum, GLenum),
    glBlitFramebuffer: extern "system" fn(
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLbitfield,
        GLenum,
    ),
    glBlitNamedFramebuffer: extern "system" fn(
        GLuint,
        GLuint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLbitfield,
        GLenum,
    ),
    glBufferData: extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum),
    glBufferStorage: extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield),
    glBufferSubData: extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void),
    glCheckFramebufferStatus: extern "system" fn(GLenum) -> GLenum,
    glCheckNamedFramebufferStatus: extern "system" fn(GLuint, GLenum) -> GLenum,
    glClampColor: extern "system" fn(GLenum, GLenum),
    glClear: extern "system" fn(GLbitfield),
    glClearBufferData: extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const c_void),
    glClearBufferSubData:
        extern "system" fn(GLenum, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void),
    glClearBufferfi: extern "system" fn(GLenum, GLint, GLfloat, GLint),
    glClearBufferfv: extern "system" fn(GLenum, GLint, *const GLfloat),
    glClearBufferiv: extern "system" fn(GLenum, GLint, *const GLint),
    glClearBufferuiv: extern "system" fn(GLenum, GLint, *const GLuint),
    glClearColor: extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat),
    glClearDepth: extern "system" fn(GLdouble),
    glClearDepthf: extern "system" fn(GLfloat),
    glClearNamedBufferData: extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const c_void),
    glClearNamedBufferSubData:
        extern "system" fn(GLuint, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void),
    glClearNamedFramebufferfi: extern "system" fn(GLuint, GLenum, GLint, GLfloat, GLint),
    glClearNamedFramebufferfv: extern "system" fn(GLuint, GLenum, GLint, *const GLfloat),
    glClearNamedFramebufferiv: extern "system" fn(GLuint, GLenum, GLint, *const GLint),
    glClearNamedFramebufferuiv: extern "system" fn(GLuint, GLenum, GLint, *const GLuint),
    glClearStencil: extern "system" fn(GLint),
    glClearTexImage: extern "system" fn(GLuint, GLint, GLenum, GLenum, *const c_void),
    glClearTexSubImage: extern "system" fn(
        GLuint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *const c_void,
    ),
    glClientWaitSync: extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum,
    glClipControl: extern "system" fn(GLenum, GLenum),
    glColorMask: extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean),
    glColorMaski: extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean),
    glCompileShader: extern "system" fn(GLuint),
    glCompressedTexImage1D:
        extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void),
    glCompressedTexImage2D:
        extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const c_void),
    glCompressedTexImage3D: extern "system" fn(
        GLenum,
        GLint,
        GLenum,
        GLsizei,
        GLsizei,
        GLsizei,
        GLint,
        GLsizei,
        *const c_void,
    ),
    glCompressedTexSubImage1D:
        extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void),
    glCompressedTexSubImage2D: extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLenum,
        GLsizei,
        *const c_void,
    ),
    glCompressedTexSubImage3D: extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLsizei,
        *const c_void,
    ),
    glCompressedTextureSubImage1D:
        extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void),
    glCompressedTextureSubImage2D: extern "system" fn(
        GLuint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLenum,
        GLsizei,
        *const c_void,
    ),
    glCompressedTextureSubImage3D: extern "system" fn(
        GLuint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLsizei,
        *const c_void,
    ),
    glCopyBufferSubData: extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr),
    glCopyImageSubData: extern "system" fn(
        GLuint,
        GLenum,
        GLint,
        GLint,
        GLint,
        GLint,
        GLuint,
        GLenum,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
    ),
    glCopyNamedBufferSubData: extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr),
    glCopyTexImage1D: extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint),
    glCopyTexImage2D:
        extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint),
    glCopyTexSubImage1D: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei),
    glCopyTexSubImage2D:
        extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei),
    glCopyTexSubImage3D:
        extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei),
    glCopyTextureSubImage1D: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei),
    glCopyTextureSubImage2D:
        extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei),
    glCopyTextureSubImage3D:
        extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei),
    glCreateBuffers: extern "system" fn(GLsizei, *mut GLuint),
    glCreateFramebuffers: extern "system" fn(GLsizei, *mut GLuint),
    glCreateProgram: extern "system" fn() -> GLuint,
    glCreateProgramPipelines: extern "system" fn(GLsizei, *mut GLuint),
    glCreateQueries: extern "system" fn(GLenum, GLsizei, *mut GLuint),
    glCreateRenderbuffers: extern "system" fn(GLsizei, *mut GLuint),
    glCreateSamplers: extern "system" fn(GLsizei, *mut GLuint),
    glCreateShader: extern "system" fn(GLenum) -> GLuint,
    glCreateShaderProgramv: extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint,
    glCreateTextures: extern "system" fn(GLenum, GLsizei, *mut GLuint),
    glCreateTransformFeedbacks: extern "system" fn(GLsizei, *mut GLuint),
    glCreateVertexArrays: extern "system" fn(GLsizei, *mut GLuint),
    glCullFace: extern "system" fn(GLenum),
    glDebugMessageCallback: extern "system" fn(GLDEBUGPROC, *const c_void),
    glDebugMessageControl:
        extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean),
    glDebugMessageInsert:
        extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar),
    glDeleteBuffers: extern "system" fn(GLsizei, *const GLuint),
    glDeleteFramebuffers: extern "system" fn(GLsizei, *const GLuint),
    glDeleteProgram: extern "system" fn(GLuint),
    glDeleteProgramPipelines: extern "system" fn(GLsizei, *const GLuint),
    glDeleteQueries: extern "system" fn(GLsizei, *const GLuint),
    glDeleteRenderbuffers: extern "system" fn(GLsizei, *const GLuint),
    glDeleteSamplers: extern "system" fn(GLsizei, *const GLuint),
    glDeleteShader: extern "system" fn(GLuint),
    glDeleteSync: extern "system" fn(GLsync),
    glDeleteTextures: extern "system" fn(GLsizei, *const GLuint),
    glDeleteTransformFeedbacks: extern "system" fn(GLsizei, *const GLuint),
    glDeleteVertexArrays: extern "system" fn(GLsizei, *const GLuint),
    glDepthFunc: extern "system" fn(GLenum),
    glDepthMask: extern "system" fn(GLboolean),
    glDepthRange: extern "system" fn(GLdouble, GLdouble),
    glDepthRangeArrayv: extern "system" fn(GLuint, GLsizei, *const GLdouble),
    glDepthRangeIndexed: extern "system" fn(GLuint, GLdouble, GLdouble),
    glDepthRangef: extern "system" fn(GLfloat, GLfloat),
    glDetachShader: extern "system" fn(GLuint, GLuint),
    glDisable: extern "system" fn(GLenum),
    glDisableVertexArrayAttrib: extern "system" fn(GLuint, GLuint),
    glDisableVertexAttribArray: extern "system" fn(GLuint),
    glDisablei: extern "system" fn(GLenum, GLuint),
    glDispatchCompute: extern "system" fn(GLuint, GLuint, GLuint),
    glDispatchComputeIndirect: extern "system" fn(GLintptr),
    glDrawArrays: extern "system" fn(GLenum, GLint, GLsizei),
    glDrawArraysIndirect: extern "system" fn(GLenum, *const c_void),
    glDrawArraysInstanced: extern "system" fn(GLenum, GLint, GLsizei, GLsizei),
    glDrawArraysInstancedBaseInstance: extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint),
    glDrawBuffer: extern "system" fn(GLenum),
    glDrawBuffers: extern "system" fn(GLsizei, *const GLenum),
    glDrawElements: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void),
    glDrawElementsBaseVertex: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint),
    glDrawElementsIndirect: extern "system" fn(GLenum, GLenum, *const c_void),
    glDrawElementsInstanced: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei),
    glDrawElementsInstancedBaseInstance:
        extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLuint),
    glDrawElementsInstancedBaseVertex:
        extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint),
    glDrawElementsInstancedBaseVertexBaseInstance:
        extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint, GLuint),
    glDrawRangeElements: extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void),
    glDrawRangeElementsBaseVertex:
        extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint),
    glDrawTransformFeedback: extern "system" fn(GLenum, GLuint),
    glDrawTransformFeedbackInstanced: extern "system" fn(GLenum, GLuint, GLsizei),
    glDrawTransformFeedbackStream: extern "system" fn(GLenum, GLuint, GLuint),
    glDrawTransformFeedbackStreamInstanced: extern "system" fn(GLenum, GLuint, GLuint, GLsizei),
    glEnable: extern "system" fn(GLenum),
    glEnableVertexArrayAttrib: extern "system" fn(GLuint, GLuint),
    glEnableVertexAttribArray: extern "system" fn(GLuint),
    glEnablei: extern "system" fn(GLenum, GLuint),
    glEndConditionalRender: extern "system" fn(),
    glEndQuery: extern "system" fn(GLenum),
    glEndQueryIndexed: extern "system" fn(GLenum, GLuint),
    glEndTransformFeedback: extern "system" fn(),
    glFenceSync: extern "system" fn(GLenum, GLbitfield) -> GLsync,
    glFinish: extern "system" fn(),
    glFlush: extern "system" fn(),
    glFlushMappedBufferRange: extern "system" fn(GLenum, GLintptr, GLsizeiptr),
    glFlushMappedNamedBufferRange: extern "system" fn(GLuint, GLintptr, GLsizeiptr),
    glFramebufferParameteri: extern "system" fn(GLenum, GLenum, GLint),
    glFramebufferRenderbuffer: extern "system" fn(GLenum, GLenum, GLenum, GLuint),
    glFramebufferTexture: extern "system" fn(GLenum, GLenum, GLuint, GLint),
    glFramebufferTexture1D: extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint),
    glFramebufferTexture2D: extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint),
    glFramebufferTexture3D: extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint),
    glFramebufferTextureLayer: extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint),
    glFrontFace: extern "system" fn(GLenum),
    glGenBuffers: extern "system" fn(GLsizei, *mut GLuint),
    glGenFramebuffers: extern "system" fn(GLsizei, *mut GLuint),
    glGenProgramPipelines: extern "system" fn(GLsizei, *mut GLuint),
    glGenQueries: extern "system" fn(GLsizei, *mut GLuint),
    glGenRenderbuffers: extern "system" fn(GLsizei, *mut GLuint),
    glGenSamplers: extern "system" fn(GLsizei, *mut GLuint),
    glGenTextures: extern "system" fn(GLsizei, *mut GLuint),
    glGenTransformFeedbacks: extern "system" fn(GLsizei, *mut GLuint),
    glGenVertexArrays: extern "system" fn(GLsizei, *mut GLuint),
    glGenerateMipmap: extern "system" fn(GLenum),
    glGenerateTextureMipmap: extern "system" fn(GLuint),
    glGetActiveAtomicCounterBufferiv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint),
    glGetActiveAttrib: extern "system" fn(
        GLuint,
        GLuint,
        GLsizei,
        *mut GLsizei,
        *mut GLint,
        *mut GLenum,
        *mut GLchar,
    ),
    glGetActiveSubroutineName:
        extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetActiveSubroutineUniformName:
        extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetActiveSubroutineUniformiv: extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint),
    glGetActiveUniform: extern "system" fn(
        GLuint,
        GLuint,
        GLsizei,
        *mut GLsizei,
        *mut GLint,
        *mut GLenum,
        *mut GLchar,
    ),
    glGetActiveUniformBlockName:
        extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetActiveUniformBlockiv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint),
    glGetActiveUniformName: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetActiveUniformsiv: extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint),
    glGetAttachedShaders: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint),
    glGetAttribLocation: extern "system" fn(GLuint, *const GLchar) -> GLint,
    glGetBooleani_v: extern "system" fn(GLenum, GLuint, *mut GLboolean),
    glGetBooleanv: extern "system" fn(GLenum, *mut GLboolean),
    glGetBufferParameteri64v: extern "system" fn(GLenum, GLenum, *mut GLint64),
    glGetBufferParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint),
    glGetBufferPointerv: extern "system" fn(GLenum, GLenum, *mut *mut c_void),
    glGetBufferSubData: extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void),
    glGetCompressedTexImage: extern "system" fn(GLenum, GLint, *mut c_void),
    glGetCompressedTextureImage: extern "system" fn(GLuint, GLint, GLsizei, *mut c_void),
    glGetCompressedTextureSubImage: extern "system" fn(
        GLuint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLsizei,
        *mut c_void,
    ),
    glGetDebugMessageLog: extern "system" fn(
        GLuint,
        GLsizei,
        *mut GLenum,
        *mut GLenum,
        *mut GLuint,
        *mut GLenum,
        *mut GLsizei,
        *mut GLchar,
    ) -> GLuint,
    glGetDoublei_v: extern "system" fn(GLenum, GLuint, *mut GLdouble),
    glGetDoublev: extern "system" fn(GLenum, *mut GLdouble),
    glGetError: extern "system" fn() -> GLenum,
    glGetFloati_v: extern "system" fn(GLenum, GLuint, *mut GLfloat),
    glGetFloatv: extern "system" fn(GLenum, *mut GLfloat),
    glGetFragDataIndex: extern "system" fn(GLuint, *const GLchar) -> GLint,
    glGetFragDataLocation: extern "system" fn(GLuint, *const GLchar) -> GLint,
    glGetFramebufferAttachmentParameteriv: extern "system" fn(GLenum, GLenum, GLenum, *mut GLint),
    glGetFramebufferParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint),
    glGetGraphicsResetStatus: extern "system" fn() -> GLenum,
    glGetInteger64i_v: extern "system" fn(GLenum, GLuint, *mut GLint64),
    glGetInteger64v: extern "system" fn(GLenum, *mut GLint64),
    glGetIntegeri_v: extern "system" fn(GLenum, GLuint, *mut GLint),
    glGetIntegerv: extern "system" fn(GLenum, *mut GLint),
    glGetInternalformati64v: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64),
    glGetInternalformativ: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint),
    glGetMultisamplefv: extern "system" fn(GLenum, GLuint, *mut GLfloat),
    glGetNamedBufferParameteri64v: extern "system" fn(GLuint, GLenum, *mut GLint64),
    glGetNamedBufferParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetNamedBufferPointerv: extern "system" fn(GLuint, GLenum, *mut *mut c_void),
    glGetNamedBufferSubData: extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut c_void),
    glGetNamedFramebufferAttachmentParameteriv:
        extern "system" fn(GLuint, GLenum, GLenum, *mut GLint),
    glGetNamedFramebufferParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetNamedRenderbufferParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetObjectLabel: extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetObjectPtrLabel: extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar),
    glGetPointerv: extern "system" fn(GLenum, *mut *mut c_void),
    glGetProgramBinary: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void),
    glGetProgramInfoLog: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetProgramInterfaceiv: extern "system" fn(GLuint, GLenum, GLenum, *mut GLint),
    glGetProgramPipelineInfoLog: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetProgramPipelineiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetProgramResourceIndex: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
    glGetProgramResourceLocation: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
    glGetProgramResourceLocationIndex: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
    glGetProgramResourceName:
        extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetProgramResourceiv: extern "system" fn(
        GLuint,
        GLenum,
        GLuint,
        GLsizei,
        *const GLenum,
        GLsizei,
        *mut GLsizei,
        *mut GLint,
    ),
    glGetProgramStageiv: extern "system" fn(GLuint, GLenum, GLenum, *mut GLint),
    glGetProgramiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetQueryBufferObjecti64v: extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
    glGetQueryBufferObjectiv: extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
    glGetQueryBufferObjectui64v: extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
    glGetQueryBufferObjectuiv: extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
    glGetQueryIndexediv: extern "system" fn(GLenum, GLuint, GLenum, *mut GLint),
    glGetQueryObjecti64v: extern "system" fn(GLuint, GLenum, *mut GLint64),
    glGetQueryObjectiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetQueryObjectui64v: extern "system" fn(GLuint, GLenum, *mut GLuint64),
    glGetQueryObjectuiv: extern "system" fn(GLuint, GLenum, *mut GLuint),
    glGetQueryiv: extern "system" fn(GLenum, GLenum, *mut GLint),
    glGetRenderbufferParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint),
    glGetSamplerParameterIiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetSamplerParameterIuiv: extern "system" fn(GLuint, GLenum, *mut GLuint),
    glGetSamplerParameterfv: extern "system" fn(GLuint, GLenum, *mut GLfloat),
    glGetSamplerParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetShaderInfoLog: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetShaderPrecisionFormat: extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint),
    glGetShaderSource: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    glGetShaderiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetString: extern "system" fn(GLenum) -> *const GLubyte,
    glGetStringi: extern "system" fn(GLenum, GLuint) -> *const GLubyte,
    glGetSubroutineIndex: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
    glGetSubroutineUniformLocation: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
    glGetSynciv: extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint),
    glGetTexImage: extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void),
    glGetTexLevelParameterfv: extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat),
    glGetTexLevelParameteriv: extern "system" fn(GLenum, GLint, GLenum, *mut GLint),
    glGetTexParameterIiv: extern "system" fn(GLenum, GLenum, *mut GLint),
    glGetTexParameterIuiv: extern "system" fn(GLenum, GLenum, *mut GLuint),
    glGetTexParameterfv: extern "system" fn(GLenum, GLenum, *mut GLfloat),
    glGetTexParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint),
    glGetTextureImage: extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut c_void),
    glGetTextureLevelParameterfv: extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat),
    glGetTextureLevelParameteriv: extern "system" fn(GLuint, GLint, GLenum, *mut GLint),
    glGetTextureParameterIiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetTextureParameterIuiv: extern "system" fn(GLuint, GLenum, *mut GLuint),
    glGetTextureParameterfv: extern "system" fn(GLuint, GLenum, *mut GLfloat),
    glGetTextureParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetTextureSubImage: extern "system" fn(
        GLuint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        GLsizei,
        *mut c_void,
    ),
    glGetTransformFeedbackVarying: extern "system" fn(
        GLuint,
        GLuint,
        GLsizei,
        *mut GLsizei,
        *mut GLsizei,
        *mut GLenum,
        *mut GLchar,
    ),
    glGetTransformFeedbacki64_v: extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64),
    glGetTransformFeedbacki_v: extern "system" fn(GLuint, GLenum, GLuint, *mut GLint),
    glGetTransformFeedbackiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetUniformBlockIndex: extern "system" fn(GLuint, *const GLchar) -> GLuint,
    glGetUniformIndices: extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint),
    glGetUniformLocation: extern "system" fn(GLuint, *const GLchar) -> GLint,
    glGetUniformSubroutineuiv: extern "system" fn(GLenum, GLint, *mut GLuint),
    glGetUniformdv: extern "system" fn(GLuint, GLint, *mut GLdouble),
    glGetUniformfv: extern "system" fn(GLuint, GLint, *mut GLfloat),
    glGetUniformiv: extern "system" fn(GLuint, GLint, *mut GLint),
    glGetUniformuiv: extern "system" fn(GLuint, GLint, *mut GLuint),
    glGetVertexArrayIndexed64iv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64),
    glGetVertexArrayIndexediv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint),
    glGetVertexArrayiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetVertexAttribIiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetVertexAttribIuiv: extern "system" fn(GLuint, GLenum, *mut GLuint),
    glGetVertexAttribLdv: extern "system" fn(GLuint, GLenum, *mut GLdouble),
    glGetVertexAttribPointerv: extern "system" fn(GLuint, GLenum, *mut *mut c_void),
    glGetVertexAttribdv: extern "system" fn(GLuint, GLenum, *mut GLdouble),
    glGetVertexAttribfv: extern "system" fn(GLuint, GLenum, *mut GLfloat),
    glGetVertexAttribiv: extern "system" fn(GLuint, GLenum, *mut GLint),
    glGetnCompressedTexImage: extern "system" fn(GLenum, GLint, GLsizei, *mut c_void),
    glGetnTexImage: extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut c_void),
    glGetnUniformdv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble),
    glGetnUniformfv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat),
    glGetnUniformiv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLint),
    glGetnUniformuiv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint),
    glHint: extern "system" fn(GLenum, GLenum),
    glInvalidateBufferData: extern "system" fn(GLuint),
    glInvalidateBufferSubData: extern "system" fn(GLuint, GLintptr, GLsizeiptr),
    glInvalidateFramebuffer: extern "system" fn(GLenum, GLsizei, *const GLenum),
    glInvalidateNamedFramebufferData: extern "system" fn(GLuint, GLsizei, *const GLenum),
    glInvalidateNamedFramebufferSubData:
        extern "system" fn(GLuint, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei),
    glInvalidateSubFramebuffer:
        extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei),
    glInvalidateTexImage: extern "system" fn(GLuint, GLint),
    glInvalidateTexSubImage:
        extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei),
    glIsBuffer: extern "system" fn(GLuint) -> GLboolean,
    glIsEnabled: extern "system" fn(GLenum) -> GLboolean,
    glIsEnabledi: extern "system" fn(GLenum, GLuint) -> GLboolean,
    glIsFramebuffer: extern "system" fn(GLuint) -> GLboolean,
    glIsProgram: extern "system" fn(GLuint) -> GLboolean,
    glIsProgramPipeline: extern "system" fn(GLuint) -> GLboolean,
    glIsQuery: extern "system" fn(GLuint) -> GLboolean,
    glIsRenderbuffer: extern "system" fn(GLuint) -> GLboolean,
    glIsSampler: extern "system" fn(GLuint) -> GLboolean,
    glIsShader: extern "system" fn(GLuint) -> GLboolean,
    glIsSync: extern "system" fn(GLsync) -> GLboolean,
    glIsTexture: extern "system" fn(GLuint) -> GLboolean,
    glIsTransformFeedback: extern "system" fn(GLuint) -> GLboolean,
    glIsVertexArray: extern "system" fn(GLuint) -> GLboolean,
    glLineWidth: extern "system" fn(GLfloat),
    glLinkProgram: extern "system" fn(GLuint),
    glLogicOp: extern "system" fn(GLenum),
    glMapBuffer: extern "system" fn(GLenum, GLenum),
    glMapBufferRange: extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield),
    glMapNamedBuffer: extern "system" fn(GLuint, GLenum),
    glMapNamedBufferRange: extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield),
    glMemoryBarrier: extern "system" fn(GLbitfield),
    glMemoryBarrierByRegion: extern "system" fn(GLbitfield),
    glMinSampleShading: extern "system" fn(GLfloat),
    glMultiDrawArrays: extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei),
    glMultiDrawArraysIndirect: extern "system" fn(GLenum, *const c_void, GLsizei, GLsizei),
    glMultiDrawArraysIndirectCount:
        extern "system" fn(GLenum, *const c_void, GLintptr, GLsizei, GLsizei),
    glMultiDrawElements:
        extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei),
    glMultiDrawElementsBaseVertex: extern "system" fn(
        GLenum,
        *const GLsizei,
        GLenum,
        *const *const c_void,
        GLsizei,
        *const GLint,
    ),
    glMultiDrawElementsIndirect:
        extern "system" fn(GLenum, GLenum, *const c_void, GLsizei, GLsizei),
    glMultiDrawElementsIndirectCount:
        extern "system" fn(GLenum, GLenum, *const c_void, GLintptr, GLsizei, GLsizei),
    glNamedBufferData: extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLenum),
    glNamedBufferStorage: extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLbitfield),
    glNamedBufferSubData: extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const c_void),
    glNamedFramebufferDrawBuffer: extern "system" fn(GLuint, GLenum),
    glNamedFramebufferDrawBuffers: extern "system" fn(GLuint, GLsizei, *const GLenum),
    glNamedFramebufferParameteri: extern "system" fn(GLuint, GLenum, GLint),
    glNamedFramebufferReadBuffer: extern "system" fn(GLuint, GLenum),
    glNamedFramebufferRenderbuffer: extern "system" fn(GLuint, GLenum, GLenum, GLuint),
    glNamedFramebufferTexture: extern "system" fn(GLuint, GLenum, GLuint, GLint),
    glNamedFramebufferTextureLayer: extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint),
    glNamedRenderbufferStorage: extern "system" fn(GLuint, GLenum, GLsizei, GLsizei),
    glNamedRenderbufferStorageMultisample:
        extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei),
    glObjectLabel: extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar),
    glObjectPtrLabel: extern "system" fn(*const c_void, GLsizei, *const GLchar),
    glPatchParameterfv: extern "system" fn(GLenum, *const GLfloat),
    glPatchParameteri: extern "system" fn(GLenum, GLint),
    glPauseTransformFeedback: extern "system" fn(),
    glPixelStoref: extern "system" fn(GLenum, GLfloat),
    glPixelStorei: extern "system" fn(GLenum, GLint),
    glPointParameterf: extern "system" fn(GLenum, GLfloat),
    glPointParameterfv: extern "system" fn(GLenum, *const GLfloat),
    glPointParameteri: extern "system" fn(GLenum, GLint),
    glPointParameteriv: extern "system" fn(GLenum, *const GLint),
    glPointSize: extern "system" fn(GLfloat),
    glPolygonMode: extern "system" fn(GLenum, GLenum),
    glPolygonOffset: extern "system" fn(GLfloat, GLfloat),
    glPolygonOffsetClamp: extern "system" fn(GLfloat, GLfloat, GLfloat),
    glPopDebugGroup: extern "system" fn(),
    glPrimitiveRestartIndex: extern "system" fn(GLuint),
    glProgramBinary: extern "system" fn(GLuint, GLenum, *const c_void, GLsizei),
    glProgramParameteri: extern "system" fn(GLuint, GLenum, GLint),
    glProgramUniform1d: extern "system" fn(GLuint, GLint, GLdouble),
    glProgramUniform1dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
    glProgramUniform1f: extern "system" fn(GLuint, GLint, GLfloat),
    glProgramUniform1fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
    glProgramUniform1i: extern "system" fn(GLuint, GLint, GLint),
    glProgramUniform1iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
    glProgramUniform1ui: extern "system" fn(GLuint, GLint, GLuint),
    glProgramUniform1uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
    glProgramUniform2d: extern "system" fn(GLuint, GLint, GLdouble, GLdouble),
    glProgramUniform2dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
    glProgramUniform2f: extern "system" fn(GLuint, GLint, GLfloat, GLfloat),
    glProgramUniform2fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
    glProgramUniform2i: extern "system" fn(GLuint, GLint, GLint, GLint),
    glProgramUniform2iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
    glProgramUniform2ui: extern "system" fn(GLuint, GLint, GLuint, GLuint),
    glProgramUniform2uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
    glProgramUniform3d: extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble),
    glProgramUniform3dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
    glProgramUniform3f: extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat),
    glProgramUniform3fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
    glProgramUniform3i: extern "system" fn(GLuint, GLint, GLint, GLint, GLint),
    glProgramUniform3iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
    glProgramUniform3ui: extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint),
    glProgramUniform3uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
    glProgramUniform4d: extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble),
    glProgramUniform4dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
    glProgramUniform4f: extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat),
    glProgramUniform4fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
    glProgramUniform4i: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint),
    glProgramUniform4iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
    glProgramUniform4ui: extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint),
    glProgramUniform4uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
    glProgramUniformMatrix2dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix2fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix2x3dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix2x3fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix2x4dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix2x4fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix3dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix3fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix3x2dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix3x2fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix3x4dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix3x4fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix4dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix4fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix4x2dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix4x2fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProgramUniformMatrix4x3dv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
    glProgramUniformMatrix4x3fv:
        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
    glProvokingVertex: extern "system" fn(GLenum),
    glPushDebugGroup: extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar),
    glQueryCounter: extern "system" fn(GLuint, GLenum),
    glReadBuffer: extern "system" fn(GLenum),
    glReadPixels: extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void),
    glReadnPixels:
        extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void),
    glReleaseShaderCompiler: extern "system" fn(),
    glRenderbufferStorage: extern "system" fn(GLenum, GLenum, GLsizei, GLsizei),
    glRenderbufferStorageMultisample: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei),
    glResumeTransformFeedback: extern "system" fn(),
    glSampleCoverage: extern "system" fn(GLfloat, GLboolean),
    glSampleMaski: extern "system" fn(GLuint, GLbitfield),
    glSamplerParameterIiv: extern "system" fn(GLuint, GLenum, *const GLint),
    glSamplerParameterIuiv: extern "system" fn(GLuint, GLenum, *const GLuint),
    glSamplerParameterf: extern "system" fn(GLuint, GLenum, GLfloat),
    glSamplerParameterfv: extern "system" fn(GLuint, GLenum, *const GLfloat),
    glSamplerParameteri: extern "system" fn(GLuint, GLenum, GLint),
    glSamplerParameteriv: extern "system" fn(GLuint, GLenum, *const GLint),
    glScissor: extern "system" fn(GLint, GLint, GLsizei, GLsizei),
    glScissorArrayv: extern "system" fn(GLuint, GLsizei, *const GLint),
    glScissorIndexed: extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei),
    glScissorIndexedv: extern "system" fn(GLuint, *const GLint),
    glShaderBinary: extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei),
    glShaderSource: extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint),
    glShaderStorageBlockBinding: extern "system" fn(GLuint, GLuint, GLuint),
    glSpecializeShader:
        extern "system" fn(GLuint, *const GLchar, GLuint, *const GLuint, *const GLuint),
    glStencilFunc: extern "system" fn(GLenum, GLint, GLuint),
    glStencilFuncSeparate: extern "system" fn(GLenum, GLenum, GLint, GLuint),
    glStencilMask: extern "system" fn(GLuint),
    glStencilMaskSeparate: extern "system" fn(GLenum, GLuint),
    glStencilOp: extern "system" fn(GLenum, GLenum, GLenum),
    glStencilOpSeparate: extern "system" fn(GLenum, GLenum, GLenum, GLenum),
    glTexBuffer: extern "system" fn(GLenum, GLenum, GLuint),
    glTexBufferRange: extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr),
    glTexImage1D:
        extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const c_void),
    glTexImage2D: extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLint,
        GLenum,
        GLenum,
        *const c_void,
    ),
    glTexImage2DMultisample:
        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean),
    glTexImage3D: extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLint,
        GLenum,
        GLenum,
        *const c_void,
    ),
    glTexImage3DMultisample:
        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean),
    glTexParameterIiv: extern "system" fn(GLenum, GLenum, *const GLint),
    glTexParameterIuiv: extern "system" fn(GLenum, GLenum, *const GLuint),
    glTexParameterf: extern "system" fn(GLenum, GLenum, GLfloat),
    glTexParameterfv: extern "system" fn(GLenum, GLenum, *const GLfloat),
    glTexParameteri: extern "system" fn(GLenum, GLenum, GLint),
    glTexParameteriv: extern "system" fn(GLenum, GLenum, *const GLint),
    glTexStorage1D: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei),
    glTexStorage2D: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei),
    glTexStorage2DMultisample:
        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean),
    glTexStorage3D: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei),
    glTexStorage3DMultisample:
        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean),
    glTexSubImage1D:
        extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void),
    glTexSubImage2D: extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *const c_void,
    ),
    glTexSubImage3D: extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *const c_void,
    ),
    glTextureBarrier: extern "system" fn(),
    glTextureBuffer: extern "system" fn(GLuint, GLenum, GLuint),
    glTextureBufferRange: extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr),
    glTextureParameterIiv: extern "system" fn(GLuint, GLenum, *const GLint),
    glTextureParameterIuiv: extern "system" fn(GLuint, GLenum, *const GLuint),
    glTextureParameterf: extern "system" fn(GLuint, GLenum, GLfloat),
    glTextureParameterfv: extern "system" fn(GLuint, GLenum, *const GLfloat),
    glTextureParameteri: extern "system" fn(GLuint, GLenum, GLint),
    glTextureParameteriv: extern "system" fn(GLuint, GLenum, *const GLint),
    glTextureStorage1D: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei),
    glTextureStorage2D: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei),
    glTextureStorage2DMultisample:
        extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean),
    glTextureStorage3D: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei),
    glTextureStorage3DMultisample:
        extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean),
    glTextureSubImage1D:
        extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void),
    glTextureSubImage2D: extern "system" fn(
        GLuint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *const c_void,
    ),
    glTextureSubImage3D: extern "system" fn(
        GLuint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *const c_void,
    ),
    glTextureView:
        extern "system" fn(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint),
    glTransformFeedbackBufferBase: extern "system" fn(GLuint, GLuint, GLuint),
    glTransformFeedbackBufferRange:
        extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr),
    glTransformFeedbackVaryings: extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum),
    glUniform1d: extern "system" fn(GLint, GLdouble),
    glUniform1dv: extern "system" fn(GLint, GLsizei, *const GLdouble),
    glUniform1f: extern "system" fn(GLint, GLfloat),
    glUniform1fv: extern "system" fn(GLint, GLsizei, *const GLfloat),
    glUniform1i: extern "system" fn(GLint, GLint),
    glUniform1iv: extern "system" fn(GLint, GLsizei, *const GLint),
    glUniform1ui: extern "system" fn(GLint, GLuint),
    glUniform1uiv: extern "system" fn(GLint, GLsizei, *const GLuint),
    glUniform2d: extern "system" fn(GLint, GLdouble, GLdouble),
    glUniform2dv: extern "system" fn(GLint, GLsizei, *const GLdouble),
    glUniform2f: extern "system" fn(GLint, GLfloat, GLfloat),
    glUniform2fv: extern "system" fn(GLint, GLsizei, *const GLfloat),
    glUniform2i: extern "system" fn(GLint, GLint, GLint),
    glUniform2iv: extern "system" fn(GLint, GLsizei, *const GLint),
    glUniform2ui: extern "system" fn(GLint, GLuint, GLuint),
    glUniform2uiv: extern "system" fn(GLint, GLsizei, *const GLuint),
    glUniform3d: extern "system" fn(GLint, GLdouble, GLdouble, GLdouble),
    glUniform3dv: extern "system" fn(GLint, GLsizei, *const GLdouble),
    glUniform3f: extern "system" fn(GLint, GLfloat, GLfloat, GLfloat),
    glUniform3fv: extern "system" fn(GLint, GLsizei, *const GLfloat),
    glUniform3i: extern "system" fn(GLint, GLint, GLint, GLint),
    glUniform3iv: extern "system" fn(GLint, GLsizei, *const GLint),
    glUniform3ui: extern "system" fn(GLint, GLuint, GLuint, GLuint),
    glUniform3uiv: extern "system" fn(GLint, GLsizei, *const GLuint),
    glUniform4d: extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble),
    glUniform4dv: extern "system" fn(GLint, GLsizei, *const GLdouble),
    glUniform4f: extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat),
    glUniform4fv: extern "system" fn(GLint, GLsizei, *const GLfloat),
    glUniform4i: extern "system" fn(GLint, GLint, GLint, GLint, GLint),
    glUniform4iv: extern "system" fn(GLint, GLsizei, *const GLint),
    glUniform4ui: extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint),
    glUniform4uiv: extern "system" fn(GLint, GLsizei, *const GLuint),
    glUniformBlockBinding: extern "system" fn(GLuint, GLuint, GLuint),
    glUniformMatrix2dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix2fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix2x3dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix2x3fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix2x4dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix2x4fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix3dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix3fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix3x2dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix3x2fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix3x4dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix3x4fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix4dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix4fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix4x2dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix4x2fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformMatrix4x3dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
    glUniformMatrix4x3fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    glUniformSubroutinesuiv: extern "system" fn(GLenum, GLsizei, *const GLuint),
    glUnmapBuffer: extern "system" fn(GLenum) -> GLboolean,
    glUnmapNamedBuffer: extern "system" fn(GLuint) -> GLboolean,
    glUseProgram: extern "system" fn(GLuint),
    glUseProgramStages: extern "system" fn(GLuint, GLbitfield, GLuint),
    glValidateProgram: extern "system" fn(GLuint),
    glValidateProgramPipeline: extern "system" fn(GLuint),
    glVertexArrayAttribBinding: extern "system" fn(GLuint, GLuint, GLuint),
    glVertexArrayAttribFormat: extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint),
    glVertexArrayAttribIFormat: extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint),
    glVertexArrayAttribLFormat: extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint),
    glVertexArrayBindingDivisor: extern "system" fn(GLuint, GLuint, GLuint),
    glVertexArrayElementBuffer: extern "system" fn(GLuint, GLuint),
    glVertexArrayVertexBuffer: extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei),
    glVertexArrayVertexBuffers:
        extern "system" fn(GLuint, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei),
    glVertexAttrib1d: extern "system" fn(GLuint, GLdouble),
    glVertexAttrib1dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttrib1f: extern "system" fn(GLuint, GLfloat),
    glVertexAttrib1fv: extern "system" fn(GLuint, *const GLfloat),
    glVertexAttrib1s: extern "system" fn(GLuint, GLshort),
    glVertexAttrib1sv: extern "system" fn(GLuint, *const GLshort),
    glVertexAttrib2d: extern "system" fn(GLuint, GLdouble, GLdouble),
    glVertexAttrib2dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttrib2f: extern "system" fn(GLuint, GLfloat, GLfloat),
    glVertexAttrib2fv: extern "system" fn(GLuint, *const GLfloat),
    glVertexAttrib2s: extern "system" fn(GLuint, GLshort, GLshort),
    glVertexAttrib2sv: extern "system" fn(GLuint, *const GLshort),
    glVertexAttrib3d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble),
    glVertexAttrib3dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttrib3f: extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat),
    glVertexAttrib3fv: extern "system" fn(GLuint, *const GLfloat),
    glVertexAttrib3s: extern "system" fn(GLuint, GLshort, GLshort, GLshort),
    glVertexAttrib3sv: extern "system" fn(GLuint, *const GLshort),
    glVertexAttrib4Nbv: extern "system" fn(GLuint, *const GLbyte),
    glVertexAttrib4Niv: extern "system" fn(GLuint, *const GLint),
    glVertexAttrib4Nsv: extern "system" fn(GLuint, *const GLshort),
    glVertexAttrib4Nub: extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte),
    glVertexAttrib4Nubv: extern "system" fn(GLuint, *const GLubyte),
    glVertexAttrib4Nuiv: extern "system" fn(GLuint, *const GLuint),
    glVertexAttrib4Nusv: extern "system" fn(GLuint, *const GLushort),
    glVertexAttrib4bv: extern "system" fn(GLuint, *const GLbyte),
    glVertexAttrib4d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble),
    glVertexAttrib4dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttrib4f: extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat),
    glVertexAttrib4fv: extern "system" fn(GLuint, *const GLfloat),
    glVertexAttrib4iv: extern "system" fn(GLuint, *const GLint),
    glVertexAttrib4s: extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort),
    glVertexAttrib4sv: extern "system" fn(GLuint, *const GLshort),
    glVertexAttrib4ubv: extern "system" fn(GLuint, *const GLubyte),
    glVertexAttrib4uiv: extern "system" fn(GLuint, *const GLuint),
    glVertexAttrib4usv: extern "system" fn(GLuint, *const GLushort),
    glVertexAttribBinding: extern "system" fn(GLuint, GLuint),
    glVertexAttribDivisor: extern "system" fn(GLuint, GLuint),
    glVertexAttribFormat: extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint),
    glVertexAttribI1i: extern "system" fn(GLuint, GLint),
    glVertexAttribI1iv: extern "system" fn(GLuint, *const GLint),
    glVertexAttribI1ui: extern "system" fn(GLuint, GLuint),
    glVertexAttribI1uiv: extern "system" fn(GLuint, *const GLuint),
    glVertexAttribI2i: extern "system" fn(GLuint, GLint, GLint),
    glVertexAttribI2iv: extern "system" fn(GLuint, *const GLint),
    glVertexAttribI2ui: extern "system" fn(GLuint, GLuint, GLuint),
    glVertexAttribI2uiv: extern "system" fn(GLuint, *const GLuint),
    glVertexAttribI3i: extern "system" fn(GLuint, GLint, GLint, GLint),
    glVertexAttribI3iv: extern "system" fn(GLuint, *const GLint),
    glVertexAttribI3ui: extern "system" fn(GLuint, GLuint, GLuint, GLuint),
    glVertexAttribI3uiv: extern "system" fn(GLuint, *const GLuint),
    glVertexAttribI4bv: extern "system" fn(GLuint, *const GLbyte),
    glVertexAttribI4i: extern "system" fn(GLuint, GLint, GLint, GLint, GLint),
    glVertexAttribI4iv: extern "system" fn(GLuint, *const GLint),
    glVertexAttribI4sv: extern "system" fn(GLuint, *const GLshort),
    glVertexAttribI4ubv: extern "system" fn(GLuint, *const GLubyte),
    glVertexAttribI4ui: extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint),
    glVertexAttribI4uiv: extern "system" fn(GLuint, *const GLuint),
    glVertexAttribI4usv: extern "system" fn(GLuint, *const GLushort),
    glVertexAttribIFormat: extern "system" fn(GLuint, GLint, GLenum, GLuint),
    glVertexAttribIPointer: extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void),
    glVertexAttribL1d: extern "system" fn(GLuint, GLdouble),
    glVertexAttribL1dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttribL2d: extern "system" fn(GLuint, GLdouble, GLdouble),
    glVertexAttribL2dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttribL3d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble),
    glVertexAttribL3dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttribL4d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble),
    glVertexAttribL4dv: extern "system" fn(GLuint, *const GLdouble),
    glVertexAttribLFormat: extern "system" fn(GLuint, GLint, GLenum, GLuint),
    glVertexAttribLPointer: extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void),
    glVertexAttribP1ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
    glVertexAttribP1uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
    glVertexAttribP2ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
    glVertexAttribP2uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
    glVertexAttribP3ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
    glVertexAttribP3uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
    glVertexAttribP4ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
    glVertexAttribP4uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
    glVertexAttribPointer:
        extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void),
    glVertexBindingDivisor: extern "system" fn(GLuint, GLuint),
    glViewport: extern "system" fn(GLint, GLint, GLsizei, GLsizei),
    glViewportArrayv: extern "system" fn(GLuint, GLsizei, *const GLfloat),
    glViewportIndexedf: extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat),
    glViewportIndexedfv: extern "system" fn(GLuint, *const GLfloat),
    glWaitSync: extern "system" fn(GLsync, GLbitfield, GLuint64),
}

impl Gl {
    pub unsafe fn load<F>(mut loader_function: F) -> Result<Self>
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        let mut load_pointer = |name: &CStr| -> Result<*const c_void> {
            let pointer = loader_function(name);
            let pointer_usize = pointer as usize;

            if pointer_usize == core::usize::MAX || pointer_usize < 8 {
                Err(LoadError {
                    name: name.to_string_lossy().to_string(),
                    pointer: pointer_usize,
                })
            } else {
                Ok(pointer)
            }
        };

        Ok(Self {
            glActiveShaderProgram: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glActiveShaderProgram\0",
                ))?,
            ),
            glActiveTexture: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glActiveTexture\0"),
            )?),
            glAttachShader: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glAttachShader\0"))?,
            ),
            glBeginConditionalRender: transmute::<*const c_void, extern "system" fn(GLuint, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glBeginConditionalRender\0",
                ))?,
            ),
            glBeginQuery: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBeginQuery\0"))?,
            ),
            glBeginQueryIndexed: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBeginQueryIndexed\0",
            ))?),
            glBeginTransformFeedback: transmute::<*const c_void, extern "system" fn(GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glBeginTransformFeedback\0",
                ))?,
            ),
            glBindAttribLocation: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, *const GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glBindAttribLocation\0"),
            )?),
            glBindBuffer: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindBuffer\0"))?,
            ),
            glBindBufferBase: transmute::<*const c_void, extern "system" fn(GLenum, GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindBufferBase\0"))?,
            ),
            glBindBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindBufferRange\0",
            ))?),
            glBindBuffersBase: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindBuffersBase\0",
            ))?),
            glBindBuffersRange: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLuint,
                    GLsizei,
                    *const GLuint,
                    *const GLintptr,
                    *const GLsizeiptr,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindBuffersRange\0",
            ))?),
            glBindFragDataLocation: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, *const GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glBindFragDataLocation\0"),
            )?),
            glBindFragDataLocationIndexed: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint, *const GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glBindFragDataLocationIndexed\0"),
            )?),
            glBindFramebuffer: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindFramebuffer\0"))?,
            ),
            glBindImageTexture: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindImageTexture\0",
            ))?),
            glBindImageTextures: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindImageTextures\0",
            ))?),
            glBindProgramPipeline: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glBindProgramPipeline\0",
                ))?,
            ),
            glBindRenderbuffer: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindRenderbuffer\0"))?,
            ),
            glBindSampler: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindSampler\0"))?,
            ),
            glBindSamplers: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindSamplers\0",
            ))?),
            glBindTexture: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindTexture\0"))?,
            ),
            glBindTextureUnit: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindTextureUnit\0"))?,
            ),
            glBindTextures: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindTextures\0",
            ))?),
            glBindTransformFeedback: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glBindTransformFeedback\0",
                ))?,
            ),
            glBindVertexArray: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBindVertexArray\0"))?,
            ),
            glBindVertexBuffer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLintptr, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindVertexBuffer\0",
            ))?),
            glBindVertexBuffers: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBindVertexBuffers\0",
            ))?),
            glBlendColor: transmute::<
                *const c_void,
                extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBlendColor\0",
            ))?),
            glBlendEquation: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glBlendEquation\0"),
            )?),
            glBlendEquationSeparate: transmute::<*const c_void, extern "system" fn(GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glBlendEquationSeparate\0",
                ))?,
            ),
            glBlendEquationSeparatei: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glBlendEquationSeparatei\0"),
            )?),
            glBlendEquationi: transmute::<*const c_void, extern "system" fn(GLuint, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBlendEquationi\0"))?,
            ),
            glBlendFunc: transmute::<*const c_void, extern "system" fn(GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBlendFunc\0"))?,
            ),
            glBlendFuncSeparate: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLenum),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBlendFuncSeparate\0",
            ))?),
            glBlendFuncSeparatei: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glBlendFuncSeparatei\0"),
            )?),
            glBlendFunci: transmute::<*const c_void, extern "system" fn(GLuint, GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glBlendFunci\0"))?,
            ),
            glBlitFramebuffer: transmute::<
                *const c_void,
                extern "system" fn(
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLbitfield,
                    GLenum,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBlitFramebuffer\0",
            ))?),
            glBlitNamedFramebuffer: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLbitfield,
                    GLenum,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glBlitNamedFramebuffer\0"),
            )?),
            glBufferData: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBufferData\0",
            ))?),
            glBufferStorage: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBufferStorage\0",
            ))?),
            glBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glBufferSubData\0",
            ))?),
            glCheckFramebufferStatus: transmute::<
                *const c_void,
                extern "system" fn(GLenum) -> GLenum,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCheckFramebufferStatus\0"),
            )?),
            glCheckNamedFramebufferStatus: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum) -> GLenum,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCheckNamedFramebufferStatus\0"),
            )?),
            glClampColor: transmute::<*const c_void, extern "system" fn(GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glClampColor\0"))?,
            ),
            glClear: transmute::<*const c_void, extern "system" fn(GLbitfield)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClear\0"),
            )?),
            glClearBufferData: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearBufferData\0",
            ))?),
            glClearBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLenum,
                    GLintptr,
                    GLsizeiptr,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearBufferSubData\0"),
            )?),
            glClearBufferfi: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLfloat, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearBufferfi\0",
            ))?),
            glClearBufferfv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearBufferfv\0",
            ))?),
            glClearBufferiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearBufferiv\0",
            ))?),
            glClearBufferuiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearBufferuiv\0",
            ))?),
            glClearColor: transmute::<
                *const c_void,
                extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearColor\0",
            ))?),
            glClearDepth: transmute::<*const c_void, extern "system" fn(GLdouble)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearDepth\0"),
            )?),
            glClearDepthf: transmute::<*const c_void, extern "system" fn(GLfloat)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearDepthf\0"),
            )?),
            glClearNamedBufferData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearNamedBufferData\0"),
            )?),
            glClearNamedBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLenum,
                    GLintptr,
                    GLsizeiptr,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearNamedBufferSubData\0"),
            )?),
            glClearNamedFramebufferfi: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint, GLfloat, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearNamedFramebufferfi\0"),
            )?),
            glClearNamedFramebufferfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearNamedFramebufferfv\0"),
            )?),
            glClearNamedFramebufferiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint, *const GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearNamedFramebufferiv\0"),
            )?),
            glClearNamedFramebufferuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearNamedFramebufferuiv\0"),
            )?),
            glClearStencil: transmute::<*const c_void, extern "system" fn(GLint)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glClearStencil\0"),
            )?),
            glClearTexImage: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLenum, *const c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearTexImage\0",
            ))?),
            glClearTexSubImage: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClearTexSubImage\0",
            ))?),
            glClientWaitSync: transmute::<
                *const c_void,
                extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum,
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glClientWaitSync\0",
            ))?),
            glClipControl: transmute::<*const c_void, extern "system" fn(GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glClipControl\0"))?,
            ),
            glColorMask: transmute::<
                *const c_void,
                extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glColorMask\0",
            ))?),
            glColorMaski: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glColorMaski\0",
            ))?),
            glCompileShader: transmute::<*const c_void, extern "system" fn(GLuint)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompileShader\0"),
            )?),
            glCompressedTexImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage1D\0"),
            )?),
            glCompressedTexImage2D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLenum,
                    GLsizei,
                    GLsizei,
                    GLint,
                    GLsizei,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage2D\0"),
            )?),
            glCompressedTexImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLenum,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLint,
                    GLsizei,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTexImage3D\0"),
            )?),
            glCompressedTexSubImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage1D\0"),
            )?),
            glCompressedTexSubImage2D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLsizei,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage2D\0"),
            )?),
            glCompressedTexSubImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLsizei,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTexSubImage3D\0"),
            )?),
            glCompressedTextureSubImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTextureSubImage1D\0"),
            )?),
            glCompressedTextureSubImage2D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLsizei,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTextureSubImage2D\0"),
            )?),
            glCompressedTextureSubImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLsizei,
                    *const c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCompressedTextureSubImage3D\0"),
            )?),
            glCopyBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCopyBufferSubData\0",
            ))?),
            glCopyImageSubData: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLenum,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLuint,
                    GLenum,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCopyImageSubData\0",
            ))?),
            glCopyNamedBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCopyNamedBufferSubData\0"),
            )?),
            glCopyTexImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCopyTexImage1D\0",
            ))?),
            glCopyTexImage2D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCopyTexImage2D\0",
            ))?),
            glCopyTexSubImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCopyTexSubImage1D\0",
            ))?),
            glCopyTexSubImage2D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCopyTexSubImage2D\0",
            ))?),
            glCopyTexSubImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCopyTexSubImage3D\0",
            ))?),
            glCopyTextureSubImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCopyTextureSubImage1D\0"),
            )?),
            glCopyTextureSubImage2D: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCopyTextureSubImage2D\0"),
            )?),
            glCopyTextureSubImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCopyTextureSubImage3D\0"),
            )?),
            glCreateBuffers: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glCreateBuffers\0"))?,
            ),
            glCreateFramebuffers: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCreateFramebuffers\0"),
            )?),
            glCreateProgram: transmute::<*const c_void, extern "system" fn() -> GLuint>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glCreateProgram\0"))?,
            ),
            glCreateProgramPipelines: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCreateProgramPipelines\0"),
            )?),
            glCreateQueries: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, *mut GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCreateQueries\0",
            ))?),
            glCreateRenderbuffers: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCreateRenderbuffers\0"),
            )?),
            glCreateSamplers: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glCreateSamplers\0"))?,
            ),
            glCreateShader: transmute::<*const c_void, extern "system" fn(GLenum) -> GLuint>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glCreateShader\0"))?,
            ),
            glCreateShaderProgramv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCreateShaderProgramv\0"),
            )?),
            glCreateTextures: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, *mut GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glCreateTextures\0",
            ))?),
            glCreateTransformFeedbacks: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCreateTransformFeedbacks\0"),
            )?),
            glCreateVertexArrays: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCreateVertexArrays\0"),
            )?),
            glCullFace: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glCullFace\0"),
            )?),
            glDebugMessageCallback: transmute::<
                *const c_void,
                extern "system" fn(GLDEBUGPROC, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDebugMessageCallback\0"),
            )?),
            glDebugMessageControl: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDebugMessageControl\0"),
            )?),
            glDebugMessageInsert: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDebugMessageInsert\0"),
            )?),
            glDeleteBuffers: transmute::<*const c_void, extern "system" fn(GLsizei, *const GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDeleteBuffers\0"))?,
            ),
            glDeleteFramebuffers: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteFramebuffers\0"),
            )?),
            glDeleteProgram: transmute::<*const c_void, extern "system" fn(GLuint)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteProgram\0"),
            )?),
            glDeleteProgramPipelines: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteProgramPipelines\0"),
            )?),
            glDeleteQueries: transmute::<*const c_void, extern "system" fn(GLsizei, *const GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDeleteQueries\0"))?,
            ),
            glDeleteRenderbuffers: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteRenderbuffers\0"),
            )?),
            glDeleteSamplers: transmute::<*const c_void, extern "system" fn(GLsizei, *const GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDeleteSamplers\0"))?,
            ),
            glDeleteShader: transmute::<*const c_void, extern "system" fn(GLuint)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteShader\0"),
            )?),
            glDeleteSync: transmute::<*const c_void, extern "system" fn(GLsync)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteSync\0"),
            )?),
            glDeleteTextures: transmute::<*const c_void, extern "system" fn(GLsizei, *const GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDeleteTextures\0"))?,
            ),
            glDeleteTransformFeedbacks: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteTransformFeedbacks\0"),
            )?),
            glDeleteVertexArrays: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDeleteVertexArrays\0"),
            )?),
            glDepthFunc: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDepthFunc\0"),
            )?),
            glDepthMask: transmute::<*const c_void, extern "system" fn(GLboolean)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDepthMask\0"),
            )?),
            glDepthRange: transmute::<*const c_void, extern "system" fn(GLdouble, GLdouble)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDepthRange\0"))?,
            ),
            glDepthRangeArrayv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glDepthRangeArrayv\0",
            ))?),
            glDepthRangeIndexed: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glDepthRangeIndexed\0",
            ))?),
            glDepthRangef: transmute::<*const c_void, extern "system" fn(GLfloat, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDepthRangef\0"))?,
            ),
            glDetachShader: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDetachShader\0"))?,
            ),
            glDisable: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDisable\0"),
            )?),
            glDisableVertexArrayAttrib: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDisableVertexArrayAttrib\0"),
            )?),
            glDisableVertexAttribArray: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glDisableVertexAttribArray\0",
                ))?,
            ),
            glDisablei: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDisablei\0"))?,
            ),
            glDispatchCompute: transmute::<*const c_void, extern "system" fn(GLuint, GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDispatchCompute\0"))?,
            ),
            glDispatchComputeIndirect: transmute::<*const c_void, extern "system" fn(GLintptr)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glDispatchComputeIndirect\0",
                ))?,
            ),
            glDrawArrays: transmute::<*const c_void, extern "system" fn(GLenum, GLint, GLsizei)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDrawArrays\0"))?,
            ),
            glDrawArraysIndirect: transmute::<
                *const c_void,
                extern "system" fn(GLenum, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawArraysIndirect\0"),
            )?),
            glDrawArraysInstanced: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawArraysInstanced\0"),
            )?),
            glDrawArraysInstancedBaseInstance: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawArraysInstancedBaseInstance\0"),
            )?),
            glDrawBuffer: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawBuffer\0"),
            )?),
            glDrawBuffers: transmute::<*const c_void, extern "system" fn(GLsizei, *const GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glDrawBuffers\0"))?,
            ),
            glDrawElements: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, *const c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glDrawElements\0",
            ))?),
            glDrawElementsBaseVertex: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawElementsBaseVertex\0"),
            )?),
            glDrawElementsIndirect: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawElementsIndirect\0"),
            )?),
            glDrawElementsInstanced: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawElementsInstanced\0"),
            )?),
            glDrawElementsInstancedBaseInstance: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawElementsInstancedBaseInstance\0"),
            )?),
            glDrawElementsInstancedBaseVertex: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawElementsInstancedBaseVertex\0"),
            )?),
            glDrawElementsInstancedBaseVertexBaseInstance: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(
                    b"glDrawElementsInstancedBaseVertexBaseInstance\0",
                ),
            )?),
            glDrawRangeElements: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glDrawRangeElements\0",
            ))?),
            glDrawRangeElementsBaseVertex: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawRangeElementsBaseVertex\0"),
            )?),
            glDrawTransformFeedback: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glDrawTransformFeedback\0",
                ))?,
            ),
            glDrawTransformFeedbackInstanced: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawTransformFeedbackInstanced\0"),
            )?),
            glDrawTransformFeedbackStream: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawTransformFeedbackStream\0"),
            )?),
            glDrawTransformFeedbackStreamInstanced: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLuint, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glDrawTransformFeedbackStreamInstanced\0"),
            )?),
            glEnable: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glEnable\0"),
            )?),
            glEnableVertexArrayAttrib: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glEnableVertexArrayAttrib\0",
                ))?,
            ),
            glEnableVertexAttribArray: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glEnableVertexAttribArray\0",
                ))?,
            ),
            glEnablei: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glEnablei\0"))?,
            ),
            glEndConditionalRender: transmute::<*const c_void, extern "system" fn()>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glEndConditionalRender\0"),
            )?),
            glEndQuery: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glEndQuery\0"),
            )?),
            glEndQueryIndexed: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glEndQueryIndexed\0"))?,
            ),
            glEndTransformFeedback: transmute::<*const c_void, extern "system" fn()>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glEndTransformFeedback\0"),
            )?),
            glFenceSync: transmute::<*const c_void, extern "system" fn(GLenum, GLbitfield) -> GLsync>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glFenceSync\0"))?,
            ),
            glFinish: transmute::<*const c_void, extern "system" fn()>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFinish\0"),
            )?),
            glFlush: transmute::<*const c_void, extern "system" fn()>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFlush\0"),
            )?),
            glFlushMappedBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLintptr, GLsizeiptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFlushMappedBufferRange\0"),
            )?),
            glFlushMappedNamedBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLintptr, GLsizeiptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFlushMappedNamedBufferRange\0"),
            )?),
            glFramebufferParameteri: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFramebufferParameteri\0"),
            )?),
            glFramebufferRenderbuffer: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFramebufferRenderbuffer\0"),
            )?),
            glFramebufferTexture: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLuint, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture\0"),
            )?),
            glFramebufferTexture1D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture1D\0"),
            )?),
            glFramebufferTexture2D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture2D\0"),
            )?),
            glFramebufferTexture3D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFramebufferTexture3D\0"),
            )?),
            glFramebufferTextureLayer: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFramebufferTextureLayer\0"),
            )?),
            glFrontFace: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glFrontFace\0"),
            )?),
            glGenBuffers: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGenBuffers\0"))?,
            ),
            glGenFramebuffers: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGenFramebuffers\0"))?,
            ),
            glGenProgramPipelines: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGenProgramPipelines\0"),
            )?),
            glGenQueries: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGenQueries\0"))?,
            ),
            glGenRenderbuffers: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGenRenderbuffers\0"))?,
            ),
            glGenSamplers: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGenSamplers\0"))?,
            ),
            glGenTextures: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGenTextures\0"))?,
            ),
            glGenTransformFeedbacks: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGenTransformFeedbacks\0"),
            )?),
            glGenVertexArrays: transmute::<*const c_void, extern "system" fn(GLsizei, *mut GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGenVertexArrays\0"))?,
            ),
            glGenerateMipmap: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGenerateMipmap\0"),
            )?),
            glGenerateTextureMipmap: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glGenerateTextureMipmap\0",
                ))?,
            ),
            glGetActiveAtomicCounterBufferiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveAtomicCounterBufferiv\0"),
            )?),
            glGetActiveAttrib: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLuint,
                    GLsizei,
                    *mut GLsizei,
                    *mut GLint,
                    *mut GLenum,
                    *mut GLchar,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetActiveAttrib\0",
            ))?),
            glGetActiveSubroutineName: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveSubroutineName\0"),
            )?),
            glGetActiveSubroutineUniformName: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveSubroutineUniformName\0"),
            )?),
            glGetActiveSubroutineUniformiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveSubroutineUniformiv\0"),
            )?),
            glGetActiveUniform: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLuint,
                    GLsizei,
                    *mut GLsizei,
                    *mut GLint,
                    *mut GLenum,
                    *mut GLchar,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetActiveUniform\0",
            ))?),
            glGetActiveUniformBlockName: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformBlockName\0"),
            )?),
            glGetActiveUniformBlockiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformBlockiv\0"),
            )?),
            glGetActiveUniformName: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformName\0"),
            )?),
            glGetActiveUniformsiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetActiveUniformsiv\0"),
            )?),
            glGetAttachedShaders: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetAttachedShaders\0"),
            )?),
            glGetAttribLocation: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLchar) -> GLint,
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetAttribLocation\0",
            ))?),
            glGetBooleani_v: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, *mut GLboolean),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetBooleani_v\0",
            ))?),
            glGetBooleanv: transmute::<*const c_void, extern "system" fn(GLenum, *mut GLboolean)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetBooleanv\0"))?,
            ),
            glGetBufferParameteri64v: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLint64),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetBufferParameteri64v\0"),
            )?),
            glGetBufferParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetBufferParameteriv\0"),
            )?),
            glGetBufferPointerv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut *mut c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetBufferPointerv\0",
            ))?),
            glGetBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetBufferSubData\0",
            ))?),
            glGetCompressedTexImage: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, *mut c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetCompressedTexImage\0"),
            )?),
            glGetCompressedTextureImage: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *mut c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetCompressedTextureImage\0"),
            )?),
            glGetCompressedTextureSubImage: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    *mut c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetCompressedTextureSubImage\0"),
            )?),
            glGetDebugMessageLog: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLsizei,
                    *mut GLenum,
                    *mut GLenum,
                    *mut GLuint,
                    *mut GLenum,
                    *mut GLsizei,
                    *mut GLchar,
                ) -> GLuint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetDebugMessageLog\0"),
            )?),
            glGetDoublei_v: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, *mut GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetDoublei_v\0",
            ))?),
            glGetDoublev: transmute::<*const c_void, extern "system" fn(GLenum, *mut GLdouble)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetDoublev\0"))?,
            ),
            glGetError: transmute::<*const c_void, extern "system" fn() -> GLenum>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetError\0"),
            )?),
            glGetFloati_v: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, *mut GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetFloati_v\0",
            ))?),
            glGetFloatv: transmute::<*const c_void, extern "system" fn(GLenum, *mut GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetFloatv\0"))?,
            ),
            glGetFragDataIndex: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLchar) -> GLint,
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetFragDataIndex\0",
            ))?),
            glGetFragDataLocation: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLchar) -> GLint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetFragDataLocation\0"),
            )?),
            glGetFramebufferAttachmentParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetFramebufferAttachmentParameteriv\0"),
            )?),
            glGetFramebufferParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetFramebufferParameteriv\0"),
            )?),
            glGetGraphicsResetStatus: transmute::<*const c_void, extern "system" fn() -> GLenum>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glGetGraphicsResetStatus\0",
                ))?,
            ),
            glGetInteger64i_v: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, *mut GLint64),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetInteger64i_v\0",
            ))?),
            glGetInteger64v: transmute::<*const c_void, extern "system" fn(GLenum, *mut GLint64)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetInteger64v\0"))?,
            ),
            glGetIntegeri_v: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetIntegeri_v\0",
            ))?),
            glGetIntegerv: transmute::<*const c_void, extern "system" fn(GLenum, *mut GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetIntegerv\0"))?,
            ),
            glGetInternalformati64v: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetInternalformati64v\0"),
            )?),
            glGetInternalformativ: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetInternalformativ\0"),
            )?),
            glGetMultisamplefv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, *mut GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetMultisamplefv\0",
            ))?),
            glGetNamedBufferParameteri64v: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint64),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetNamedBufferParameteri64v\0"),
            )?),
            glGetNamedBufferParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetNamedBufferParameteriv\0"),
            )?),
            glGetNamedBufferPointerv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut *mut c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetNamedBufferPointerv\0"),
            )?),
            glGetNamedBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetNamedBufferSubData\0"),
            )?),
            glGetNamedFramebufferAttachmentParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(
                    b"glGetNamedFramebufferAttachmentParameteriv\0",
                ),
            )?),
            glGetNamedFramebufferParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetNamedFramebufferParameteriv\0"),
            )?),
            glGetNamedRenderbufferParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetNamedRenderbufferParameteriv\0"),
            )?),
            glGetObjectLabel: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetObjectLabel\0",
            ))?),
            glGetObjectPtrLabel: transmute::<
                *const c_void,
                extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetObjectPtrLabel\0",
            ))?),
            glGetPointerv: transmute::<*const c_void, extern "system" fn(GLenum, *mut *mut c_void)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetPointerv\0"))?,
            ),
            glGetProgramBinary: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetProgramBinary\0",
            ))?),
            glGetProgramInfoLog: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetProgramInfoLog\0",
            ))?),
            glGetProgramInterfaceiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramInterfaceiv\0"),
            )?),
            glGetProgramPipelineInfoLog: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramPipelineInfoLog\0"),
            )?),
            glGetProgramPipelineiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramPipelineiv\0"),
            )?),
            glGetProgramResourceIndex: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceIndex\0"),
            )?),
            glGetProgramResourceLocation: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceLocation\0"),
            )?),
            glGetProgramResourceLocationIndex: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceLocationIndex\0"),
            )?),
            glGetProgramResourceName: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceName\0"),
            )?),
            glGetProgramResourceiv: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLenum,
                    GLuint,
                    GLsizei,
                    *const GLenum,
                    GLsizei,
                    *mut GLsizei,
                    *mut GLint,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetProgramResourceiv\0"),
            )?),
            glGetProgramStageiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLenum, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetProgramStageiv\0",
            ))?),
            glGetProgramiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetProgramiv\0",
            ))?),
            glGetQueryBufferObjecti64v: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetQueryBufferObjecti64v\0"),
            )?),
            glGetQueryBufferObjectiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetQueryBufferObjectiv\0"),
            )?),
            glGetQueryBufferObjectui64v: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetQueryBufferObjectui64v\0"),
            )?),
            glGetQueryBufferObjectuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, GLintptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetQueryBufferObjectuiv\0"),
            )?),
            glGetQueryIndexediv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLenum, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetQueryIndexediv\0",
            ))?),
            glGetQueryObjecti64v: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint64),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetQueryObjecti64v\0"),
            )?),
            glGetQueryObjectiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetQueryObjectiv\0",
            ))?),
            glGetQueryObjectui64v: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLuint64),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetQueryObjectui64v\0"),
            )?),
            glGetQueryObjectuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetQueryObjectuiv\0",
            ))?),
            glGetQueryiv: transmute::<*const c_void, extern "system" fn(GLenum, GLenum, *mut GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetQueryiv\0"))?,
            ),
            glGetRenderbufferParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetRenderbufferParameteriv\0"),
            )?),
            glGetSamplerParameterIiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterIiv\0"),
            )?),
            glGetSamplerParameterIuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterIuiv\0"),
            )?),
            glGetSamplerParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameterfv\0"),
            )?),
            glGetSamplerParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetSamplerParameteriv\0"),
            )?),
            glGetShaderInfoLog: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetShaderInfoLog\0",
            ))?),
            glGetShaderPrecisionFormat: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetShaderPrecisionFormat\0"),
            )?),
            glGetShaderSource: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetShaderSource\0",
            ))?),
            glGetShaderiv: transmute::<*const c_void, extern "system" fn(GLuint, GLenum, *mut GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetShaderiv\0"))?,
            ),
            glGetString: transmute::<*const c_void, extern "system" fn(GLenum) -> *const GLubyte>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetString\0"))?,
            ),
            glGetStringi: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint) -> *const GLubyte,
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetStringi\0",
            ))?),
            glGetSubroutineIndex: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetSubroutineIndex\0"),
            )?),
            glGetSubroutineUniformLocation: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetSubroutineUniformLocation\0"),
            )?),
            glGetSynciv: transmute::<
                *const c_void,
                extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetSynciv\0",
            ))?),
            glGetTexImage: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetTexImage\0",
            ))?),
            glGetTexLevelParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTexLevelParameterfv\0"),
            )?),
            glGetTexLevelParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTexLevelParameteriv\0"),
            )?),
            glGetTexParameterIiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterIiv\0"),
            )?),
            glGetTexParameterIuiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTexParameterIuiv\0"),
            )?),
            glGetTexParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetTexParameterfv\0",
            ))?),
            glGetTexParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetTexParameteriv\0",
            ))?),
            glGetTextureImage: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetTextureImage\0",
            ))?),
            glGetTextureLevelParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTextureLevelParameterfv\0"),
            )?),
            glGetTextureLevelParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTextureLevelParameteriv\0"),
            )?),
            glGetTextureParameterIiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameterIiv\0"),
            )?),
            glGetTextureParameterIuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameterIuiv\0"),
            )?),
            glGetTextureParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameterfv\0"),
            )?),
            glGetTextureParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTextureParameteriv\0"),
            )?),
            glGetTextureSubImage: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLenum,
                    GLsizei,
                    *mut c_void,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTextureSubImage\0"),
            )?),
            glGetTransformFeedbackVarying: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLuint,
                    GLsizei,
                    *mut GLsizei,
                    *mut GLsizei,
                    *mut GLenum,
                    *mut GLchar,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTransformFeedbackVarying\0"),
            )?),
            glGetTransformFeedbacki64_v: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTransformFeedbacki64_v\0"),
            )?),
            glGetTransformFeedbacki_v: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTransformFeedbacki_v\0"),
            )?),
            glGetTransformFeedbackiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetTransformFeedbackiv\0"),
            )?),
            glGetUniformBlockIndex: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLchar) -> GLuint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetUniformBlockIndex\0"),
            )?),
            glGetUniformIndices: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetUniformIndices\0",
            ))?),
            glGetUniformLocation: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLchar) -> GLint,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetUniformLocation\0"),
            )?),
            glGetUniformSubroutineuiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetUniformSubroutineuiv\0"),
            )?),
            glGetUniformdv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, *mut GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetUniformdv\0",
            ))?),
            glGetUniformfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, *mut GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetUniformfv\0",
            ))?),
            glGetUniformiv: transmute::<*const c_void, extern "system" fn(GLuint, GLint, *mut GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glGetUniformiv\0"))?,
            ),
            glGetUniformuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, *mut GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetUniformuiv\0",
            ))?),
            glGetVertexArrayIndexed64iv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetVertexArrayIndexed64iv\0"),
            )?),
            glGetVertexArrayIndexediv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetVertexArrayIndexediv\0"),
            )?),
            glGetVertexArrayiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetVertexArrayiv\0",
            ))?),
            glGetVertexAttribIiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribIiv\0"),
            )?),
            glGetVertexAttribIuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribIuiv\0"),
            )?),
            glGetVertexAttribLdv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribLdv\0"),
            )?),
            glGetVertexAttribPointerv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut *mut c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetVertexAttribPointerv\0"),
            )?),
            glGetVertexAttribdv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetVertexAttribdv\0",
            ))?),
            glGetVertexAttribfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetVertexAttribfv\0",
            ))?),
            glGetVertexAttribiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetVertexAttribiv\0",
            ))?),
            glGetnCompressedTexImage: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLsizei, *mut c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glGetnCompressedTexImage\0"),
            )?),
            glGetnTexImage: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetnTexImage\0",
            ))?),
            glGetnUniformdv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetnUniformdv\0",
            ))?),
            glGetnUniformfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetnUniformfv\0",
            ))?),
            glGetnUniformiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *mut GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetnUniformiv\0",
            ))?),
            glGetnUniformuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glGetnUniformuiv\0",
            ))?),
            glHint: transmute::<*const c_void, extern "system" fn(GLenum, GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glHint\0"),
            )?),
            glInvalidateBufferData: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glInvalidateBufferData\0",
                ))?,
            ),
            glInvalidateBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLintptr, GLsizeiptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glInvalidateBufferSubData\0"),
            )?),
            glInvalidateFramebuffer: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, *const GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glInvalidateFramebuffer\0"),
            )?),
            glInvalidateNamedFramebufferData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glInvalidateNamedFramebufferData\0"),
            )?),
            glInvalidateNamedFramebufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glInvalidateNamedFramebufferSubData\0"),
            )?),
            glInvalidateSubFramebuffer: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glInvalidateSubFramebuffer\0"),
            )?),
            glInvalidateTexImage: transmute::<*const c_void, extern "system" fn(GLuint, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glInvalidateTexImage\0",
                ))?,
            ),
            glInvalidateTexSubImage: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glInvalidateTexSubImage\0"),
            )?),
            glIsBuffer: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsBuffer\0"))?,
            ),
            glIsEnabled: transmute::<*const c_void, extern "system" fn(GLenum) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsEnabled\0"))?,
            ),
            glIsEnabledi: transmute::<*const c_void, extern "system" fn(GLenum, GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsEnabledi\0"))?,
            ),
            glIsFramebuffer: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsFramebuffer\0"))?,
            ),
            glIsProgram: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsProgram\0"))?,
            ),
            glIsProgramPipeline: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glIsProgramPipeline\0",
                ))?,
            ),
            glIsQuery: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsQuery\0"))?,
            ),
            glIsRenderbuffer: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsRenderbuffer\0"))?,
            ),
            glIsSampler: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsSampler\0"))?,
            ),
            glIsShader: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsShader\0"))?,
            ),
            glIsSync: transmute::<*const c_void, extern "system" fn(GLsync) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsSync\0"))?,
            ),
            glIsTexture: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsTexture\0"))?,
            ),
            glIsTransformFeedback: transmute::<
                *const c_void,
                extern "system" fn(GLuint) -> GLboolean,
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glIsTransformFeedback\0"),
            )?),
            glIsVertexArray: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glIsVertexArray\0"))?,
            ),
            glLineWidth: transmute::<*const c_void, extern "system" fn(GLfloat)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glLineWidth\0"),
            )?),
            glLinkProgram: transmute::<*const c_void, extern "system" fn(GLuint)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glLinkProgram\0"),
            )?),
            glLogicOp: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glLogicOp\0"),
            )?),
            glMapBuffer: transmute::<*const c_void, extern "system" fn(GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glMapBuffer\0"))?,
            ),
            glMapBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glMapBufferRange\0",
            ))?),
            glMapNamedBuffer: transmute::<*const c_void, extern "system" fn(GLuint, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glMapNamedBuffer\0"))?,
            ),
            glMapNamedBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glMapNamedBufferRange\0"),
            )?),
            glMemoryBarrier: transmute::<*const c_void, extern "system" fn(GLbitfield)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glMemoryBarrier\0"))?,
            ),
            glMemoryBarrierByRegion: transmute::<*const c_void, extern "system" fn(GLbitfield)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glMemoryBarrierByRegion\0",
                ))?,
            ),
            glMinSampleShading: transmute::<*const c_void, extern "system" fn(GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glMinSampleShading\0"))?,
            ),
            glMultiDrawArrays: transmute::<
                *const c_void,
                extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glMultiDrawArrays\0",
            ))?),
            glMultiDrawArraysIndirect: transmute::<
                *const c_void,
                extern "system" fn(GLenum, *const c_void, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glMultiDrawArraysIndirect\0"),
            )?),
            glMultiDrawArraysIndirectCount: transmute::<
                *const c_void,
                extern "system" fn(GLenum, *const c_void, GLintptr, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glMultiDrawArraysIndirectCount\0"),
            )?),
            glMultiDrawElements: transmute::<
                *const c_void,
                extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glMultiDrawElements\0",
            ))?),
            glMultiDrawElementsBaseVertex: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    *const GLsizei,
                    GLenum,
                    *const *const c_void,
                    GLsizei,
                    *const GLint,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glMultiDrawElementsBaseVertex\0"),
            )?),
            glMultiDrawElementsIndirect: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *const c_void, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glMultiDrawElementsIndirect\0"),
            )?),
            glMultiDrawElementsIndirectCount: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *const c_void, GLintptr, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glMultiDrawElementsIndirectCount\0"),
            )?),
            glNamedBufferData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLenum),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glNamedBufferData\0",
            ))?),
            glNamedBufferStorage: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLbitfield),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedBufferStorage\0"),
            )?),
            glNamedBufferSubData: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedBufferSubData\0"),
            )?),
            glNamedFramebufferDrawBuffer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferDrawBuffer\0"),
            )?),
            glNamedFramebufferDrawBuffers: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferDrawBuffers\0"),
            )?),
            glNamedFramebufferParameteri: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferParameteri\0"),
            )?),
            glNamedFramebufferReadBuffer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferReadBuffer\0"),
            )?),
            glNamedFramebufferRenderbuffer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLenum, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferRenderbuffer\0"),
            )?),
            glNamedFramebufferTexture: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferTexture\0"),
            )?),
            glNamedFramebufferTextureLayer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedFramebufferTextureLayer\0"),
            )?),
            glNamedRenderbufferStorage: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedRenderbufferStorage\0"),
            )?),
            glNamedRenderbufferStorageMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glNamedRenderbufferStorageMultisample\0"),
            )?),
            glObjectLabel: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glObjectLabel\0",
            ))?),
            glObjectPtrLabel: transmute::<
                *const c_void,
                extern "system" fn(*const c_void, GLsizei, *const GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glObjectPtrLabel\0",
            ))?),
            glPatchParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glPatchParameterfv\0",
            ))?),
            glPatchParameteri: transmute::<*const c_void, extern "system" fn(GLenum, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPatchParameteri\0"))?,
            ),
            glPauseTransformFeedback: transmute::<*const c_void, extern "system" fn()>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glPauseTransformFeedback\0",
                ))?,
            ),
            glPixelStoref: transmute::<*const c_void, extern "system" fn(GLenum, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPixelStoref\0"))?,
            ),
            glPixelStorei: transmute::<*const c_void, extern "system" fn(GLenum, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPixelStorei\0"))?,
            ),
            glPointParameterf: transmute::<*const c_void, extern "system" fn(GLenum, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPointParameterf\0"))?,
            ),
            glPointParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glPointParameterfv\0",
            ))?),
            glPointParameteri: transmute::<*const c_void, extern "system" fn(GLenum, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPointParameteri\0"))?,
            ),
            glPointParameteriv: transmute::<*const c_void, extern "system" fn(GLenum, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPointParameteriv\0"))?,
            ),
            glPointSize: transmute::<*const c_void, extern "system" fn(GLfloat)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glPointSize\0"),
            )?),
            glPolygonMode: transmute::<*const c_void, extern "system" fn(GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPolygonMode\0"))?,
            ),
            glPolygonOffset: transmute::<*const c_void, extern "system" fn(GLfloat, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glPolygonOffset\0"))?,
            ),
            glPolygonOffsetClamp: transmute::<
                *const c_void,
                extern "system" fn(GLfloat, GLfloat, GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glPolygonOffsetClamp\0"),
            )?),
            glPopDebugGroup: transmute::<*const c_void, extern "system" fn()>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glPopDebugGroup\0"),
            )?),
            glPrimitiveRestartIndex: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glPrimitiveRestartIndex\0",
                ))?,
            ),
            glProgramBinary: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const c_void, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramBinary\0",
            ))?),
            glProgramParameteri: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramParameteri\0",
            ))?),
            glProgramUniform1d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform1d\0",
            ))?),
            glProgramUniform1dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform1dv\0",
            ))?),
            glProgramUniform1f: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform1f\0",
            ))?),
            glProgramUniform1fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform1fv\0",
            ))?),
            glProgramUniform1i: transmute::<*const c_void, extern "system" fn(GLuint, GLint, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1i\0"))?,
            ),
            glProgramUniform1iv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform1iv\0",
            ))?),
            glProgramUniform1ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform1ui\0",
            ))?),
            glProgramUniform1uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniform1uiv\0"),
            )?),
            glProgramUniform2d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform2d\0",
            ))?),
            glProgramUniform2dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform2dv\0",
            ))?),
            glProgramUniform2f: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform2f\0",
            ))?),
            glProgramUniform2fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform2fv\0",
            ))?),
            glProgramUniform2i: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform2i\0",
            ))?),
            glProgramUniform2iv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform2iv\0",
            ))?),
            glProgramUniform2ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform2ui\0",
            ))?),
            glProgramUniform2uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniform2uiv\0"),
            )?),
            glProgramUniform3d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform3d\0",
            ))?),
            glProgramUniform3dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform3dv\0",
            ))?),
            glProgramUniform3f: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform3f\0",
            ))?),
            glProgramUniform3fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform3fv\0",
            ))?),
            glProgramUniform3i: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform3i\0",
            ))?),
            glProgramUniform3iv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform3iv\0",
            ))?),
            glProgramUniform3ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform3ui\0",
            ))?),
            glProgramUniform3uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniform3uiv\0"),
            )?),
            glProgramUniform4d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform4d\0",
            ))?),
            glProgramUniform4dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform4dv\0",
            ))?),
            glProgramUniform4f: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform4f\0",
            ))?),
            glProgramUniform4fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform4fv\0",
            ))?),
            glProgramUniform4i: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform4i\0",
            ))?),
            glProgramUniform4iv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform4iv\0",
            ))?),
            glProgramUniform4ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glProgramUniform4ui\0",
            ))?),
            glProgramUniform4uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniform4uiv\0"),
            )?),
            glProgramUniformMatrix2dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2dv\0"),
            )?),
            glProgramUniformMatrix2fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2fv\0"),
            )?),
            glProgramUniformMatrix2x3dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2x3dv\0"),
            )?),
            glProgramUniformMatrix2x3fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2x3fv\0"),
            )?),
            glProgramUniformMatrix2x4dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2x4dv\0"),
            )?),
            glProgramUniformMatrix2x4fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix2x4fv\0"),
            )?),
            glProgramUniformMatrix3dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3dv\0"),
            )?),
            glProgramUniformMatrix3fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3fv\0"),
            )?),
            glProgramUniformMatrix3x2dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3x2dv\0"),
            )?),
            glProgramUniformMatrix3x2fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3x2fv\0"),
            )?),
            glProgramUniformMatrix3x4dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3x4dv\0"),
            )?),
            glProgramUniformMatrix3x4fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix3x4fv\0"),
            )?),
            glProgramUniformMatrix4dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4dv\0"),
            )?),
            glProgramUniformMatrix4fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4fv\0"),
            )?),
            glProgramUniformMatrix4x2dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4x2dv\0"),
            )?),
            glProgramUniformMatrix4x2fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4x2fv\0"),
            )?),
            glProgramUniformMatrix4x3dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4x3dv\0"),
            )?),
            glProgramUniformMatrix4x3fv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glProgramUniformMatrix4x3fv\0"),
            )?),
            glProvokingVertex: transmute::<*const c_void, extern "system" fn(GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glProvokingVertex\0"))?,
            ),
            glPushDebugGroup: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glPushDebugGroup\0",
            ))?),
            glQueryCounter: transmute::<*const c_void, extern "system" fn(GLuint, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glQueryCounter\0"))?,
            ),
            glReadBuffer: transmute::<*const c_void, extern "system" fn(GLenum)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glReadBuffer\0"),
            )?),
            glReadPixels: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glReadPixels\0",
            ))?),
            glReadnPixels: transmute::<
                *const c_void,
                extern "system" fn(
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLenum,
                    GLsizei,
                    *mut c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glReadnPixels\0",
            ))?),
            glReleaseShaderCompiler: transmute::<*const c_void, extern "system" fn()>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glReleaseShaderCompiler\0",
                ))?,
            ),
            glRenderbufferStorage: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glRenderbufferStorage\0"),
            )?),
            glRenderbufferStorageMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glRenderbufferStorageMultisample\0"),
            )?),
            glResumeTransformFeedback: transmute::<*const c_void, extern "system" fn()>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glResumeTransformFeedback\0",
                ))?,
            ),
            glSampleCoverage: transmute::<*const c_void, extern "system" fn(GLfloat, GLboolean)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glSampleCoverage\0"))?,
            ),
            glSampleMaski: transmute::<*const c_void, extern "system" fn(GLuint, GLbitfield)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glSampleMaski\0"))?,
            ),
            glSamplerParameterIiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterIiv\0"),
            )?),
            glSamplerParameterIuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterIuiv\0"),
            )?),
            glSamplerParameterf: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glSamplerParameterf\0",
            ))?),
            glSamplerParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glSamplerParameterfv\0"),
            )?),
            glSamplerParameteri: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glSamplerParameteri\0",
            ))?),
            glSamplerParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glSamplerParameteriv\0"),
            )?),
            glScissor: transmute::<*const c_void, extern "system" fn(GLint, GLint, GLsizei, GLsizei)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glScissor\0"))?,
            ),
            glScissorArrayv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glScissorArrayv\0",
            ))?),
            glScissorIndexed: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glScissorIndexed\0",
            ))?),
            glScissorIndexedv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glScissorIndexedv\0"))?,
            ),
            glShaderBinary: transmute::<
                *const c_void,
                extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glShaderBinary\0",
            ))?),
            glShaderSource: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glShaderSource\0",
            ))?),
            glShaderStorageBlockBinding: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glShaderStorageBlockBinding\0"),
            )?),
            glSpecializeShader: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLchar, GLuint, *const GLuint, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glSpecializeShader\0",
            ))?),
            glStencilFunc: transmute::<*const c_void, extern "system" fn(GLenum, GLint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glStencilFunc\0"))?,
            ),
            glStencilFuncSeparate: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glStencilFuncSeparate\0"),
            )?),
            glStencilMask: transmute::<*const c_void, extern "system" fn(GLuint)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glStencilMask\0"),
            )?),
            glStencilMaskSeparate: transmute::<*const c_void, extern "system" fn(GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glStencilMaskSeparate\0",
                ))?,
            ),
            glStencilOp: transmute::<*const c_void, extern "system" fn(GLenum, GLenum, GLenum)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glStencilOp\0"))?,
            ),
            glStencilOpSeparate: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLenum, GLenum),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glStencilOpSeparate\0",
            ))?),
            glTexBuffer: transmute::<*const c_void, extern "system" fn(GLenum, GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glTexBuffer\0"))?,
            ),
            glTexBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexBufferRange\0",
            ))?),
            glTexImage1D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLsizei,
                    GLint,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexImage1D\0",
            ))?),
            glTexImage2D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLint,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexImage2D\0",
            ))?),
            glTexImage2DMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTexImage2DMultisample\0"),
            )?),
            glTexImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLint,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexImage3D\0",
            ))?),
            glTexImage3DMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTexImage3DMultisample\0"),
            )?),
            glTexParameterIiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexParameterIiv\0",
            ))?),
            glTexParameterIuiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexParameterIuiv\0",
            ))?),
            glTexParameterf: transmute::<*const c_void, extern "system" fn(GLenum, GLenum, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glTexParameterf\0"))?,
            ),
            glTexParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexParameterfv\0",
            ))?),
            glTexParameteri: transmute::<*const c_void, extern "system" fn(GLenum, GLenum, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glTexParameteri\0"))?,
            ),
            glTexParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLenum, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexParameteriv\0",
            ))?),
            glTexStorage1D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexStorage1D\0",
            ))?),
            glTexStorage2D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexStorage2D\0",
            ))?),
            glTexStorage2DMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTexStorage2DMultisample\0"),
            )?),
            glTexStorage3D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexStorage3D\0",
            ))?),
            glTexStorage3DMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTexStorage3DMultisample\0"),
            )?),
            glTexSubImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexSubImage1D\0",
            ))?),
            glTexSubImage2D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexSubImage2D\0",
            ))?),
            glTexSubImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLenum,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTexSubImage3D\0",
            ))?),
            glTextureBarrier: transmute::<*const c_void, extern "system" fn()>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureBarrier\0"),
            )?),
            glTextureBuffer: transmute::<*const c_void, extern "system" fn(GLuint, GLenum, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glTextureBuffer\0"))?,
            ),
            glTextureBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureBufferRange\0"),
            )?),
            glTextureParameterIiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureParameterIiv\0"),
            )?),
            glTextureParameterIuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureParameterIuiv\0"),
            )?),
            glTextureParameterf: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureParameterf\0",
            ))?),
            glTextureParameterfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureParameterfv\0"),
            )?),
            glTextureParameteri: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureParameteri\0",
            ))?),
            glTextureParameteriv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, *const GLint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureParameteriv\0"),
            )?),
            glTextureStorage1D: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, GLenum, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureStorage1D\0",
            ))?),
            glTextureStorage2D: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureStorage2D\0",
            ))?),
            glTextureStorage2DMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureStorage2DMultisample\0"),
            )?),
            glTextureStorage3D: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureStorage3D\0",
            ))?),
            glTextureStorage3DMultisample: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTextureStorage3DMultisample\0"),
            )?),
            glTextureSubImage1D: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureSubImage1D\0",
            ))?),
            glTextureSubImage2D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureSubImage2D\0",
            ))?),
            glTextureSubImage3D: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLint,
                    GLint,
                    GLint,
                    GLint,
                    GLsizei,
                    GLsizei,
                    GLsizei,
                    GLenum,
                    GLenum,
                    *const c_void,
                ),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureSubImage3D\0",
            ))?),
            glTextureView: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glTextureView\0",
            ))?),
            glTransformFeedbackBufferBase: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTransformFeedbackBufferBase\0"),
            )?),
            glTransformFeedbackBufferRange: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTransformFeedbackBufferRange\0"),
            )?),
            glTransformFeedbackVaryings: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glTransformFeedbackVaryings\0"),
            )?),
            glUniform1d: transmute::<*const c_void, extern "system" fn(GLint, GLdouble)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform1d\0"))?,
            ),
            glUniform1dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform1dv\0",
            ))?),
            glUniform1f: transmute::<*const c_void, extern "system" fn(GLint, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform1f\0"))?,
            ),
            glUniform1fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform1fv\0",
            ))?),
            glUniform1i: transmute::<*const c_void, extern "system" fn(GLint, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform1i\0"))?,
            ),
            glUniform1iv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform1iv\0",
            ))?),
            glUniform1ui: transmute::<*const c_void, extern "system" fn(GLint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform1ui\0"))?,
            ),
            glUniform1uiv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform1uiv\0",
            ))?),
            glUniform2d: transmute::<*const c_void, extern "system" fn(GLint, GLdouble, GLdouble)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform2d\0"))?,
            ),
            glUniform2dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform2dv\0",
            ))?),
            glUniform2f: transmute::<*const c_void, extern "system" fn(GLint, GLfloat, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform2f\0"))?,
            ),
            glUniform2fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform2fv\0",
            ))?),
            glUniform2i: transmute::<*const c_void, extern "system" fn(GLint, GLint, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform2i\0"))?,
            ),
            glUniform2iv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform2iv\0",
            ))?),
            glUniform2ui: transmute::<*const c_void, extern "system" fn(GLint, GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform2ui\0"))?,
            ),
            glUniform2uiv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform2uiv\0",
            ))?),
            glUniform3d: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform3d\0",
            ))?),
            glUniform3dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform3dv\0",
            ))?),
            glUniform3f: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform3f\0",
            ))?),
            glUniform3fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform3fv\0",
            ))?),
            glUniform3i: transmute::<*const c_void, extern "system" fn(GLint, GLint, GLint, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUniform3i\0"))?,
            ),
            glUniform3iv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform3iv\0",
            ))?),
            glUniform3ui: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform3ui\0",
            ))?),
            glUniform3uiv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform3uiv\0",
            ))?),
            glUniform4d: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4d\0",
            ))?),
            glUniform4dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4dv\0",
            ))?),
            glUniform4f: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4f\0",
            ))?),
            glUniform4fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4fv\0",
            ))?),
            glUniform4i: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLint, GLint, GLint, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4i\0",
            ))?),
            glUniform4iv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4iv\0",
            ))?),
            glUniform4ui: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4ui\0",
            ))?),
            glUniform4uiv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniform4uiv\0",
            ))?),
            glUniformBlockBinding: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformBlockBinding\0"),
            )?),
            glUniformMatrix2dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniformMatrix2dv\0",
            ))?),
            glUniformMatrix2fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniformMatrix2fv\0",
            ))?),
            glUniformMatrix2x3dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x3dv\0"),
            )?),
            glUniformMatrix2x3fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x3fv\0"),
            )?),
            glUniformMatrix2x4dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x4dv\0"),
            )?),
            glUniformMatrix2x4fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix2x4fv\0"),
            )?),
            glUniformMatrix3dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniformMatrix3dv\0",
            ))?),
            glUniformMatrix3fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniformMatrix3fv\0",
            ))?),
            glUniformMatrix3x2dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x2dv\0"),
            )?),
            glUniformMatrix3x2fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x2fv\0"),
            )?),
            glUniformMatrix3x4dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x4dv\0"),
            )?),
            glUniformMatrix3x4fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix3x4fv\0"),
            )?),
            glUniformMatrix4dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniformMatrix4dv\0",
            ))?),
            glUniformMatrix4fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUniformMatrix4fv\0",
            ))?),
            glUniformMatrix4x2dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x2dv\0"),
            )?),
            glUniformMatrix4x2fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x2fv\0"),
            )?),
            glUniformMatrix4x3dv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x3dv\0"),
            )?),
            glUniformMatrix4x3fv: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformMatrix4x3fv\0"),
            )?),
            glUniformSubroutinesuiv: transmute::<
                *const c_void,
                extern "system" fn(GLenum, GLsizei, *const GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUniformSubroutinesuiv\0"),
            )?),
            glUnmapBuffer: transmute::<*const c_void, extern "system" fn(GLenum) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUnmapBuffer\0"))?,
            ),
            glUnmapNamedBuffer: transmute::<*const c_void, extern "system" fn(GLuint) -> GLboolean>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glUnmapNamedBuffer\0"))?,
            ),
            glUseProgram: transmute::<*const c_void, extern "system" fn(GLuint)>(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glUseProgram\0"),
            )?),
            glUseProgramStages: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLbitfield, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glUseProgramStages\0",
            ))?),
            glValidateProgram: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glValidateProgram\0"))?,
            ),
            glValidateProgramPipeline: transmute::<*const c_void, extern "system" fn(GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glValidateProgramPipeline\0",
                ))?,
            ),
            glVertexArrayAttribBinding: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayAttribBinding\0"),
            )?),
            glVertexArrayAttribFormat: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayAttribFormat\0"),
            )?),
            glVertexArrayAttribIFormat: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayAttribIFormat\0"),
            )?),
            glVertexArrayAttribLFormat: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayAttribLFormat\0"),
            )?),
            glVertexArrayBindingDivisor: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayBindingDivisor\0"),
            )?),
            glVertexArrayElementBuffer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayElementBuffer\0"),
            )?),
            glVertexArrayVertexBuffer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayVertexBuffer\0"),
            )?),
            glVertexArrayVertexBuffers: transmute::<
                *const c_void,
                extern "system" fn(
                    GLuint,
                    GLuint,
                    GLsizei,
                    *const GLuint,
                    *const GLintptr,
                    *const GLsizei,
                ),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexArrayVertexBuffers\0"),
            )?),
            glVertexAttrib1d: transmute::<*const c_void, extern "system" fn(GLuint, GLdouble)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1d\0"))?,
            ),
            glVertexAttrib1dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib1dv\0",
            ))?),
            glVertexAttrib1f: transmute::<*const c_void, extern "system" fn(GLuint, GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1f\0"))?,
            ),
            glVertexAttrib1fv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1fv\0"))?,
            ),
            glVertexAttrib1s: transmute::<*const c_void, extern "system" fn(GLuint, GLshort)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1s\0"))?,
            ),
            glVertexAttrib1sv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLshort)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib1sv\0"))?,
            ),
            glVertexAttrib2d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib2d\0",
            ))?),
            glVertexAttrib2dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib2dv\0",
            ))?),
            glVertexAttrib2f: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib2f\0",
            ))?),
            glVertexAttrib2fv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2fv\0"))?,
            ),
            glVertexAttrib2s: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLshort, GLshort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib2s\0",
            ))?),
            glVertexAttrib2sv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLshort)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib2sv\0"))?,
            ),
            glVertexAttrib3d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib3d\0",
            ))?),
            glVertexAttrib3dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib3dv\0",
            ))?),
            glVertexAttrib3f: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib3f\0",
            ))?),
            glVertexAttrib3fv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3fv\0"))?,
            ),
            glVertexAttrib3s: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLshort, GLshort, GLshort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib3s\0",
            ))?),
            glVertexAttrib3sv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLshort)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib3sv\0"))?,
            ),
            glVertexAttrib4Nbv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLbyte)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Nbv\0"))?,
            ),
            glVertexAttrib4Niv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4Niv\0"))?,
            ),
            glVertexAttrib4Nsv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLshort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4Nsv\0",
            ))?),
            glVertexAttrib4Nub: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4Nub\0",
            ))?),
            glVertexAttrib4Nubv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLubyte),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4Nubv\0",
            ))?),
            glVertexAttrib4Nuiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4Nuiv\0",
            ))?),
            glVertexAttrib4Nusv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLushort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4Nusv\0",
            ))?),
            glVertexAttrib4bv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLbyte)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4bv\0"))?,
            ),
            glVertexAttrib4d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4d\0",
            ))?),
            glVertexAttrib4dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4dv\0",
            ))?),
            glVertexAttrib4f: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4f\0",
            ))?),
            glVertexAttrib4fv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLfloat)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4fv\0"))?,
            ),
            glVertexAttrib4iv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4iv\0"))?,
            ),
            glVertexAttrib4s: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4s\0",
            ))?),
            glVertexAttrib4sv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLshort)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4sv\0"))?,
            ),
            glVertexAttrib4ubv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLubyte),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4ubv\0",
            ))?),
            glVertexAttrib4uiv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttrib4uiv\0"))?,
            ),
            glVertexAttrib4usv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLushort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttrib4usv\0",
            ))?),
            glVertexAttribBinding: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glVertexAttribBinding\0",
                ))?,
            ),
            glVertexAttribDivisor: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glVertexAttribDivisor\0",
                ))?,
            ),
            glVertexAttribFormat: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexAttribFormat\0"),
            )?),
            glVertexAttribI1i: transmute::<*const c_void, extern "system" fn(GLuint, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI1i\0"))?,
            ),
            glVertexAttribI1iv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI1iv\0"))?,
            ),
            glVertexAttribI1ui: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI1ui\0"))?,
            ),
            glVertexAttribI1uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI1uiv\0",
            ))?),
            glVertexAttribI2i: transmute::<*const c_void, extern "system" fn(GLuint, GLint, GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI2i\0"))?,
            ),
            glVertexAttribI2iv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI2iv\0"))?,
            ),
            glVertexAttribI2ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI2ui\0",
            ))?),
            glVertexAttribI2uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI2uiv\0",
            ))?),
            glVertexAttribI3i: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI3i\0",
            ))?),
            glVertexAttribI3iv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI3iv\0"))?,
            ),
            glVertexAttribI3ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI3ui\0",
            ))?),
            glVertexAttribI3uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI3uiv\0",
            ))?),
            glVertexAttribI4bv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLbyte)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4bv\0"))?,
            ),
            glVertexAttribI4i: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLint, GLint, GLint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI4i\0",
            ))?),
            glVertexAttribI4iv: transmute::<*const c_void, extern "system" fn(GLuint, *const GLint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribI4iv\0"))?,
            ),
            glVertexAttribI4sv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLshort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI4sv\0",
            ))?),
            glVertexAttribI4ubv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLubyte),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI4ubv\0",
            ))?),
            glVertexAttribI4ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI4ui\0",
            ))?),
            glVertexAttribI4uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI4uiv\0",
            ))?),
            glVertexAttribI4usv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLushort),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribI4usv\0",
            ))?),
            glVertexAttribIFormat: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexAttribIFormat\0"),
            )?),
            glVertexAttribIPointer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexAttribIPointer\0"),
            )?),
            glVertexAttribL1d: transmute::<*const c_void, extern "system" fn(GLuint, GLdouble)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glVertexAttribL1d\0"))?,
            ),
            glVertexAttribL1dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribL1dv\0",
            ))?),
            glVertexAttribL2d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribL2d\0",
            ))?),
            glVertexAttribL2dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribL2dv\0",
            ))?),
            glVertexAttribL3d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribL3d\0",
            ))?),
            glVertexAttribL3dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribL3dv\0",
            ))?),
            glVertexAttribL4d: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribL4d\0",
            ))?),
            glVertexAttribL4dv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLdouble),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribL4dv\0",
            ))?),
            glVertexAttribLFormat: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLuint),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexAttribLFormat\0"),
            )?),
            glVertexAttribLPointer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexAttribLPointer\0"),
            )?),
            glVertexAttribP1ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP1ui\0",
            ))?),
            glVertexAttribP1uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP1uiv\0",
            ))?),
            glVertexAttribP2ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP2ui\0",
            ))?),
            glVertexAttribP2uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP2uiv\0",
            ))?),
            glVertexAttribP3ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP3ui\0",
            ))?),
            glVertexAttribP3uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP3uiv\0",
            ))?),
            glVertexAttribP4ui: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP4ui\0",
            ))?),
            glVertexAttribP4uiv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glVertexAttribP4uiv\0",
            ))?),
            glVertexAttribPointer: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void),
            >(load_pointer(
                CStr::from_bytes_with_nul_unchecked(b"glVertexAttribPointer\0"),
            )?),
            glVertexBindingDivisor: transmute::<*const c_void, extern "system" fn(GLuint, GLuint)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(
                    b"glVertexBindingDivisor\0",
                ))?,
            ),
            glViewport: transmute::<
                *const c_void,
                extern "system" fn(GLint, GLint, GLsizei, GLsizei),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glViewport\0",
            ))?),
            glViewportArrayv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLsizei, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glViewportArrayv\0",
            ))?),
            glViewportIndexedf: transmute::<
                *const c_void,
                extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glViewportIndexedf\0",
            ))?),
            glViewportIndexedfv: transmute::<
                *const c_void,
                extern "system" fn(GLuint, *const GLfloat),
            >(load_pointer(CStr::from_bytes_with_nul_unchecked(
                b"glViewportIndexedfv\0",
            ))?),
            glWaitSync: transmute::<*const c_void, extern "system" fn(GLsync, GLbitfield, GLuint64)>(
                load_pointer(CStr::from_bytes_with_nul_unchecked(b"glWaitSync\0"))?,
            ),
        })
    }

    pub unsafe fn ActiveShaderProgram(&self, pipeline: GLuint, program: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ActiveShaderProgram()");
        (self.glActiveShaderProgram)(pipeline, program)
    }
    pub unsafe fn ActiveTexture(&self, texture: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ActiveTexture()");
        (self.glActiveTexture)(texture)
    }
    pub unsafe fn AttachShader(&self, program: GLuint, shader: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling AttachShader()");
        (self.glAttachShader)(program, shader)
    }
    pub unsafe fn BeginConditionalRender(&self, id: GLuint, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BeginConditionalRender()");
        (self.glBeginConditionalRender)(id, mode)
    }
    pub unsafe fn BeginQuery(&self, target: GLenum, id: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BeginQuery()");
        (self.glBeginQuery)(target, id)
    }
    pub unsafe fn BeginQueryIndexed(&self, target: GLenum, index: GLuint, id: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BeginQueryIndexed()");
        (self.glBeginQueryIndexed)(target, index, id)
    }
    pub unsafe fn BeginTransformFeedback(&self, primitiveMode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BeginTransformFeedback()");
        (self.glBeginTransformFeedback)(primitiveMode)
    }
    pub unsafe fn BindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindAttribLocation()");
        (self.glBindAttribLocation)(program, index, name)
    }
    pub unsafe fn BindBuffer(&self, target: GLenum, buffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindBuffer()");
        (self.glBindBuffer)(target, buffer)
    }
    pub unsafe fn BindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindBufferBase()");
        (self.glBindBufferBase)(target, index, buffer)
    }
    pub unsafe fn BindBufferRange(
        &self,
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindBufferRange()");
        (self.glBindBufferRange)(target, index, buffer, offset, size)
    }
    pub unsafe fn BindBuffersBase(
        &self,
        target: GLenum,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindBuffersBase()");
        (self.glBindBuffersBase)(target, first, count, buffers)
    }
    pub unsafe fn BindBuffersRange(
        &self,
        target: GLenum,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        sizes: *const GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindBuffersRange()");
        (self.glBindBuffersRange)(target, first, count, buffers, offsets, sizes)
    }
    pub unsafe fn BindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindFragDataLocation()");
        (self.glBindFragDataLocation)(program, color, name)
    }
    pub unsafe fn BindFragDataLocationIndexed(
        &self,
        program: GLuint,
        colorNumber: GLuint,
        index: GLuint,
        name: *const GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindFragDataLocationIndexed()");
        (self.glBindFragDataLocationIndexed)(program, colorNumber, index, name)
    }
    pub unsafe fn BindFramebuffer(&self, target: GLenum, framebuffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindFramebuffer()");
        (self.glBindFramebuffer)(target, framebuffer)
    }
    pub unsafe fn BindImageTexture(
        &self,
        unit: GLuint,
        texture: GLuint,
        level: GLint,
        layered: GLboolean,
        layer: GLint,
        access: GLenum,
        format: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindImageTexture()");
        (self.glBindImageTexture)(unit, texture, level, layered, layer, access, format)
    }
    pub unsafe fn BindImageTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindImageTextures()");
        (self.glBindImageTextures)(first, count, textures)
    }
    pub unsafe fn BindProgramPipeline(&self, pipeline: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindProgramPipeline()");
        (self.glBindProgramPipeline)(pipeline)
    }
    pub unsafe fn BindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindRenderbuffer()");
        (self.glBindRenderbuffer)(target, renderbuffer)
    }
    pub unsafe fn BindSampler(&self, unit: GLuint, sampler: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindSampler()");
        (self.glBindSampler)(unit, sampler)
    }
    pub unsafe fn BindSamplers(&self, first: GLuint, count: GLsizei, samplers: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindSamplers()");
        (self.glBindSamplers)(first, count, samplers)
    }
    pub unsafe fn BindTexture(&self, target: GLenum, texture: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindTexture()");
        (self.glBindTexture)(target, texture)
    }
    pub unsafe fn BindTextureUnit(&self, unit: GLuint, texture: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindTextureUnit()");
        (self.glBindTextureUnit)(unit, texture)
    }
    pub unsafe fn BindTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindTextures()");
        (self.glBindTextures)(first, count, textures)
    }
    pub unsafe fn BindTransformFeedback(&self, target: GLenum, id: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindTransformFeedback()");
        (self.glBindTransformFeedback)(target, id)
    }
    pub unsafe fn BindVertexArray(&self, array: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindVertexArray()");
        (self.glBindVertexArray)(array)
    }
    pub unsafe fn BindVertexBuffer(
        &self,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindVertexBuffer()");
        (self.glBindVertexBuffer)(bindingindex, buffer, offset, stride)
    }
    pub unsafe fn BindVertexBuffers(
        &self,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BindVertexBuffers()");
        (self.glBindVertexBuffers)(first, count, buffers, offsets, strides)
    }
    pub unsafe fn BlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendColor()");
        (self.glBlendColor)(red, green, blue, alpha)
    }
    pub unsafe fn BlendEquation(&self, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendEquation()");
        (self.glBlendEquation)(mode)
    }
    pub unsafe fn BlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendEquationSeparate()");
        (self.glBlendEquationSeparate)(modeRGB, modeAlpha)
    }
    pub unsafe fn BlendEquationSeparatei(&self, buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendEquationSeparatei()");
        (self.glBlendEquationSeparatei)(buf, modeRGB, modeAlpha)
    }
    pub unsafe fn BlendEquationi(&self, buf: GLuint, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendEquationi()");
        (self.glBlendEquationi)(buf, mode)
    }
    pub unsafe fn BlendFunc(&self, sfactor: GLenum, dfactor: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendFunc()");
        (self.glBlendFunc)(sfactor, dfactor)
    }
    pub unsafe fn BlendFuncSeparate(
        &self,
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendFuncSeparate()");
        (self.glBlendFuncSeparate)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
    }
    pub unsafe fn BlendFuncSeparatei(
        &self,
        buf: GLuint,
        srcRGB: GLenum,
        dstRGB: GLenum,
        srcAlpha: GLenum,
        dstAlpha: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendFuncSeparatei()");
        (self.glBlendFuncSeparatei)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha)
    }
    pub unsafe fn BlendFunci(&self, buf: GLuint, src: GLenum, dst: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlendFunci()");
        (self.glBlendFunci)(buf, src, dst)
    }
    pub unsafe fn BlitFramebuffer(
        &self,
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlitFramebuffer()");
        (self.glBlitFramebuffer)(
            srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter,
        )
    }
    pub unsafe fn BlitNamedFramebuffer(
        &self,
        readFramebuffer: GLuint,
        drawFramebuffer: GLuint,
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BlitNamedFramebuffer()");
        (self.glBlitNamedFramebuffer)(
            readFramebuffer,
            drawFramebuffer,
            srcX0,
            srcY0,
            srcX1,
            srcY1,
            dstX0,
            dstY0,
            dstX1,
            dstY1,
            mask,
            filter,
        )
    }
    pub unsafe fn BufferData(
        &self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BufferData()");
        (self.glBufferData)(target, size, data, usage)
    }
    pub unsafe fn BufferStorage(
        &self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const c_void,
        flags: GLbitfield,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BufferStorage()");
        (self.glBufferStorage)(target, size, data, flags)
    }
    pub unsafe fn BufferSubData(
        &self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling BufferSubData()");
        (self.glBufferSubData)(target, offset, size, data)
    }
    pub unsafe fn CheckFramebufferStatus(&self, target: GLenum) -> GLenum {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CheckFramebufferStatus()");
        (self.glCheckFramebufferStatus)(target)
    }
    pub unsafe fn CheckNamedFramebufferStatus(
        &self,
        framebuffer: GLuint,
        target: GLenum,
    ) -> GLenum {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CheckNamedFramebufferStatus()");
        (self.glCheckNamedFramebufferStatus)(framebuffer, target)
    }
    pub unsafe fn ClampColor(&self, target: GLenum, clamp: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClampColor()");
        (self.glClampColor)(target, clamp)
    }
    pub unsafe fn Clear(&self, mask: GLbitfield) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Clear()");
        (self.glClear)(mask)
    }
    pub unsafe fn ClearBufferData(
        &self,
        target: GLenum,
        internalformat: GLenum,
        format: GLenum,
        r#type: GLenum,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearBufferData()");
        (self.glClearBufferData)(target, internalformat, format, r#type, data)
    }
    pub unsafe fn ClearBufferSubData(
        &self,
        target: GLenum,
        internalformat: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        format: GLenum,
        r#type: GLenum,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearBufferSubData()");
        (self.glClearBufferSubData)(target, internalformat, offset, size, format, r#type, data)
    }
    pub unsafe fn ClearBufferfi(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearBufferfi()");
        (self.glClearBufferfi)(buffer, drawbuffer, depth, stencil)
    }
    pub unsafe fn ClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearBufferfv()");
        (self.glClearBufferfv)(buffer, drawbuffer, value)
    }
    pub unsafe fn ClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearBufferiv()");
        (self.glClearBufferiv)(buffer, drawbuffer, value)
    }
    pub unsafe fn ClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearBufferuiv()");
        (self.glClearBufferuiv)(buffer, drawbuffer, value)
    }
    pub unsafe fn ClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearColor()");
        (self.glClearColor)(red, green, blue, alpha)
    }
    pub unsafe fn ClearDepth(&self, depth: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearDepth()");
        (self.glClearDepth)(depth)
    }
    pub unsafe fn ClearDepthf(&self, d: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearDepthf()");
        (self.glClearDepthf)(d)
    }
    pub unsafe fn ClearNamedBufferData(
        &self,
        buffer: GLuint,
        internalformat: GLenum,
        format: GLenum,
        r#type: GLenum,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearNamedBufferData()");
        (self.glClearNamedBufferData)(buffer, internalformat, format, r#type, data)
    }
    pub unsafe fn ClearNamedBufferSubData(
        &self,
        buffer: GLuint,
        internalformat: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        format: GLenum,
        r#type: GLenum,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearNamedBufferSubData()");
        (self.glClearNamedBufferSubData)(buffer, internalformat, offset, size, format, r#type, data)
    }
    pub unsafe fn ClearNamedFramebufferfi(
        &self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearNamedFramebufferfi()");
        (self.glClearNamedFramebufferfi)(framebuffer, buffer, drawbuffer, depth, stencil)
    }
    pub unsafe fn ClearNamedFramebufferfv(
        &self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearNamedFramebufferfv()");
        (self.glClearNamedFramebufferfv)(framebuffer, buffer, drawbuffer, value)
    }
    pub unsafe fn ClearNamedFramebufferiv(
        &self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearNamedFramebufferiv()");
        (self.glClearNamedFramebufferiv)(framebuffer, buffer, drawbuffer, value)
    }
    pub unsafe fn ClearNamedFramebufferuiv(
        &self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearNamedFramebufferuiv()");
        (self.glClearNamedFramebufferuiv)(framebuffer, buffer, drawbuffer, value)
    }
    pub unsafe fn ClearStencil(&self, s: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearStencil()");
        (self.glClearStencil)(s)
    }
    pub unsafe fn ClearTexImage(
        &self,
        texture: GLuint,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearTexImage()");
        (self.glClearTexImage)(texture, level, format, r#type, data)
    }
    pub unsafe fn ClearTexSubImage(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClearTexSubImage()");
        (self.glClearTexSubImage)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, data,
        )
    }
    pub unsafe fn ClientWaitSync(
        &self,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> GLenum {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClientWaitSync()");
        (self.glClientWaitSync)(sync, flags, timeout)
    }
    pub unsafe fn ClipControl(&self, origin: GLenum, depth: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ClipControl()");
        (self.glClipControl)(origin, depth)
    }
    pub unsafe fn ColorMask(
        &self,
        red: GLboolean,
        green: GLboolean,
        blue: GLboolean,
        alpha: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ColorMask()");
        (self.glColorMask)(red, green, blue, alpha)
    }
    pub unsafe fn ColorMaski(
        &self,
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ColorMaski()");
        (self.glColorMaski)(index, r, g, b, a)
    }
    pub unsafe fn CompileShader(&self, shader: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompileShader()");
        (self.glCompileShader)(shader)
    }
    pub unsafe fn CompressedTexImage1D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTexImage1D()");
        (self.glCompressedTexImage1D)(
            target,
            level,
            internalformat,
            width,
            border,
            imageSize,
            data,
        )
    }
    pub unsafe fn CompressedTexImage2D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTexImage2D()");
        (self.glCompressedTexImage2D)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        )
    }
    pub unsafe fn CompressedTexImage3D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTexImage3D()");
        (self.glCompressedTexImage3D)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            imageSize,
            data,
        )
    }
    pub unsafe fn CompressedTexSubImage1D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTexSubImage1D()");
        (self.glCompressedTexSubImage1D)(target, level, xoffset, width, format, imageSize, data)
    }
    pub unsafe fn CompressedTexSubImage2D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTexSubImage2D()");
        (self.glCompressedTexSubImage2D)(
            target, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    }
    pub unsafe fn CompressedTexSubImage3D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTexSubImage3D()");
        (self.glCompressedTexSubImage3D)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data,
        )
    }
    pub unsafe fn CompressedTextureSubImage1D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTextureSubImage1D()");
        (self.glCompressedTextureSubImage1D)(
            texture, level, xoffset, width, format, imageSize, data,
        )
    }
    pub unsafe fn CompressedTextureSubImage2D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTextureSubImage2D()");
        (self.glCompressedTextureSubImage2D)(
            texture, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    }
    pub unsafe fn CompressedTextureSubImage3D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CompressedTextureSubImage3D()");
        (self.glCompressedTextureSubImage3D)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize,
            data,
        )
    }
    pub unsafe fn CopyBufferSubData(
        &self,
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyBufferSubData()");
        (self.glCopyBufferSubData)(readTarget, writeTarget, readOffset, writeOffset, size)
    }
    pub unsafe fn CopyImageSubData(
        &self,
        srcName: GLuint,
        srcTarget: GLenum,
        srcLevel: GLint,
        srcX: GLint,
        srcY: GLint,
        srcZ: GLint,
        dstName: GLuint,
        dstTarget: GLenum,
        dstLevel: GLint,
        dstX: GLint,
        dstY: GLint,
        dstZ: GLint,
        srcWidth: GLsizei,
        srcHeight: GLsizei,
        srcDepth: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyImageSubData()");
        (self.glCopyImageSubData)(
            srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX,
            dstY, dstZ, srcWidth, srcHeight, srcDepth,
        )
    }
    pub unsafe fn CopyNamedBufferSubData(
        &self,
        readBuffer: GLuint,
        writeBuffer: GLuint,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyNamedBufferSubData()");
        (self.glCopyNamedBufferSubData)(readBuffer, writeBuffer, readOffset, writeOffset, size)
    }
    pub unsafe fn CopyTexImage1D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        border: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTexImage1D()");
        (self.glCopyTexImage1D)(target, level, internalformat, x, y, width, border)
    }
    pub unsafe fn CopyTexImage2D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTexImage2D()");
        (self.glCopyTexImage2D)(target, level, internalformat, x, y, width, height, border)
    }
    pub unsafe fn CopyTexSubImage1D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTexSubImage1D()");
        (self.glCopyTexSubImage1D)(target, level, xoffset, x, y, width)
    }
    pub unsafe fn CopyTexSubImage2D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTexSubImage2D()");
        (self.glCopyTexSubImage2D)(target, level, xoffset, yoffset, x, y, width, height)
    }
    pub unsafe fn CopyTexSubImage3D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTexSubImage3D()");
        (self.glCopyTexSubImage3D)(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }
    pub unsafe fn CopyTextureSubImage1D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTextureSubImage1D()");
        (self.glCopyTextureSubImage1D)(texture, level, xoffset, x, y, width)
    }
    pub unsafe fn CopyTextureSubImage2D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTextureSubImage2D()");
        (self.glCopyTextureSubImage2D)(texture, level, xoffset, yoffset, x, y, width, height)
    }
    pub unsafe fn CopyTextureSubImage3D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CopyTextureSubImage3D()");
        (self.glCopyTextureSubImage3D)(
            texture, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }
    pub unsafe fn CreateBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateBuffers()");
        (self.glCreateBuffers)(n, buffers)
    }
    pub unsafe fn CreateFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateFramebuffers()");
        (self.glCreateFramebuffers)(n, framebuffers)
    }
    pub unsafe fn CreateProgram(&self) -> GLuint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateProgram()");
        (self.glCreateProgram)()
    }
    pub unsafe fn CreateProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateProgramPipelines()");
        (self.glCreateProgramPipelines)(n, pipelines)
    }
    pub unsafe fn CreateQueries(&self, target: GLenum, n: GLsizei, ids: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateQueries()");
        (self.glCreateQueries)(target, n, ids)
    }
    pub unsafe fn CreateRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateRenderbuffers()");
        (self.glCreateRenderbuffers)(n, renderbuffers)
    }
    pub unsafe fn CreateSamplers(&self, n: GLsizei, samplers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateSamplers()");
        (self.glCreateSamplers)(n, samplers)
    }
    pub unsafe fn CreateShader(&self, r#type: GLenum) -> GLuint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateShader()");
        (self.glCreateShader)(r#type)
    }
    pub unsafe fn CreateShaderProgramv(
        &self,
        r#type: GLenum,
        count: GLsizei,
        strings: *const *const GLchar,
    ) -> GLuint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateShaderProgramv()");
        (self.glCreateShaderProgramv)(r#type, count, strings)
    }
    pub unsafe fn CreateTextures(&self, target: GLenum, n: GLsizei, textures: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateTextures()");
        (self.glCreateTextures)(target, n, textures)
    }
    pub unsafe fn CreateTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateTransformFeedbacks()");
        (self.glCreateTransformFeedbacks)(n, ids)
    }
    pub unsafe fn CreateVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CreateVertexArrays()");
        (self.glCreateVertexArrays)(n, arrays)
    }
    pub unsafe fn CullFace(&self, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling CullFace()");
        (self.glCullFace)(mode)
    }
    pub unsafe fn DebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const c_void) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DebugMessageCallback()");
        (self.glDebugMessageCallback)(callback, userParam)
    }
    pub unsafe fn DebugMessageControl(
        &self,
        source: GLenum,
        r#type: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DebugMessageControl()");
        (self.glDebugMessageControl)(source, r#type, severity, count, ids, enabled)
    }
    pub unsafe fn DebugMessageInsert(
        &self,
        source: GLenum,
        r#type: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DebugMessageInsert()");
        (self.glDebugMessageInsert)(source, r#type, id, severity, length, buf)
    }
    pub unsafe fn DeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteBuffers()");
        (self.glDeleteBuffers)(n, buffers)
    }
    pub unsafe fn DeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteFramebuffers()");
        (self.glDeleteFramebuffers)(n, framebuffers)
    }
    pub unsafe fn DeleteProgram(&self, program: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteProgram()");
        (self.glDeleteProgram)(program)
    }
    pub unsafe fn DeleteProgramPipelines(&self, n: GLsizei, pipelines: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteProgramPipelines()");
        (self.glDeleteProgramPipelines)(n, pipelines)
    }
    pub unsafe fn DeleteQueries(&self, n: GLsizei, ids: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteQueries()");
        (self.glDeleteQueries)(n, ids)
    }
    pub unsafe fn DeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteRenderbuffers()");
        (self.glDeleteRenderbuffers)(n, renderbuffers)
    }
    pub unsafe fn DeleteSamplers(&self, count: GLsizei, samplers: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteSamplers()");
        (self.glDeleteSamplers)(count, samplers)
    }
    pub unsafe fn DeleteShader(&self, shader: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteShader()");
        (self.glDeleteShader)(shader)
    }
    pub unsafe fn DeleteSync(&self, sync: GLsync) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteSync()");
        (self.glDeleteSync)(sync)
    }
    pub unsafe fn DeleteTextures(&self, n: GLsizei, textures: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteTextures()");
        (self.glDeleteTextures)(n, textures)
    }
    pub unsafe fn DeleteTransformFeedbacks(&self, n: GLsizei, ids: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteTransformFeedbacks()");
        (self.glDeleteTransformFeedbacks)(n, ids)
    }
    pub unsafe fn DeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DeleteVertexArrays()");
        (self.glDeleteVertexArrays)(n, arrays)
    }
    pub unsafe fn DepthFunc(&self, func: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DepthFunc()");
        (self.glDepthFunc)(func)
    }
    pub unsafe fn DepthMask(&self, flag: GLboolean) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DepthMask()");
        (self.glDepthMask)(flag)
    }
    pub unsafe fn DepthRange(&self, n: GLdouble, f: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DepthRange()");
        (self.glDepthRange)(n, f)
    }
    pub unsafe fn DepthRangeArrayv(&self, first: GLuint, count: GLsizei, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DepthRangeArrayv()");
        (self.glDepthRangeArrayv)(first, count, v)
    }
    pub unsafe fn DepthRangeIndexed(&self, index: GLuint, n: GLdouble, f: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DepthRangeIndexed()");
        (self.glDepthRangeIndexed)(index, n, f)
    }
    pub unsafe fn DepthRangef(&self, n: GLfloat, f: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DepthRangef()");
        (self.glDepthRangef)(n, f)
    }
    pub unsafe fn DetachShader(&self, program: GLuint, shader: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DetachShader()");
        (self.glDetachShader)(program, shader)
    }
    pub unsafe fn Disable(&self, cap: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Disable()");
        (self.glDisable)(cap)
    }
    pub unsafe fn DisableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DisableVertexArrayAttrib()");
        (self.glDisableVertexArrayAttrib)(vaobj, index)
    }
    pub unsafe fn DisableVertexAttribArray(&self, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DisableVertexAttribArray()");
        (self.glDisableVertexAttribArray)(index)
    }
    pub unsafe fn Disablei(&self, target: GLenum, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Disablei()");
        (self.glDisablei)(target, index)
    }
    pub unsafe fn DispatchCompute(
        &self,
        num_groups_x: GLuint,
        num_groups_y: GLuint,
        num_groups_z: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DispatchCompute()");
        (self.glDispatchCompute)(num_groups_x, num_groups_y, num_groups_z)
    }
    pub unsafe fn DispatchComputeIndirect(&self, indirect: GLintptr) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DispatchComputeIndirect()");
        (self.glDispatchComputeIndirect)(indirect)
    }
    pub unsafe fn DrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawArrays()");
        (self.glDrawArrays)(mode, first, count)
    }
    pub unsafe fn DrawArraysIndirect(&self, mode: GLenum, indirect: *const c_void) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawArraysIndirect()");
        (self.glDrawArraysIndirect)(mode, indirect)
    }
    pub unsafe fn DrawArraysInstanced(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawArraysInstanced()");
        (self.glDrawArraysInstanced)(mode, first, count, instancecount)
    }
    pub unsafe fn DrawArraysInstancedBaseInstance(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
        baseinstance: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawArraysInstancedBaseInstance()");
        (self.glDrawArraysInstancedBaseInstance)(mode, first, count, instancecount, baseinstance)
    }
    pub unsafe fn DrawBuffer(&self, buf: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawBuffer()");
        (self.glDrawBuffer)(buf)
    }
    pub unsafe fn DrawBuffers(&self, n: GLsizei, bufs: *const GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawBuffers()");
        (self.glDrawBuffers)(n, bufs)
    }
    pub unsafe fn DrawElements(
        &self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawElements()");
        (self.glDrawElements)(mode, count, r#type, indices)
    }
    pub unsafe fn DrawElementsBaseVertex(
        &self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
        basevertex: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawElementsBaseVertex()");
        (self.glDrawElementsBaseVertex)(mode, count, r#type, indices, basevertex)
    }
    pub unsafe fn DrawElementsIndirect(
        &self,
        mode: GLenum,
        r#type: GLenum,
        indirect: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawElementsIndirect()");
        (self.glDrawElementsIndirect)(mode, r#type, indirect)
    }
    pub unsafe fn DrawElementsInstanced(
        &self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawElementsInstanced()");
        (self.glDrawElementsInstanced)(mode, count, r#type, indices, instancecount)
    }
    pub unsafe fn DrawElementsInstancedBaseInstance(
        &self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        baseinstance: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawElementsInstancedBaseInstance()");
        (self.glDrawElementsInstancedBaseInstance)(
            mode,
            count,
            r#type,
            indices,
            instancecount,
            baseinstance,
        )
    }
    pub unsafe fn DrawElementsInstancedBaseVertex(
        &self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        basevertex: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawElementsInstancedBaseVertex()");
        (self.glDrawElementsInstancedBaseVertex)(
            mode,
            count,
            r#type,
            indices,
            instancecount,
            basevertex,
        )
    }
    pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(
        &self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        basevertex: GLint,
        baseinstance: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawElementsInstancedBaseVertexBaseInstance()");
        (self.glDrawElementsInstancedBaseVertexBaseInstance)(
            mode,
            count,
            r#type,
            indices,
            instancecount,
            basevertex,
            baseinstance,
        )
    }
    pub unsafe fn DrawRangeElements(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawRangeElements()");
        (self.glDrawRangeElements)(mode, start, end, count, r#type, indices)
    }
    pub unsafe fn DrawRangeElementsBaseVertex(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        r#type: GLenum,
        indices: *const c_void,
        basevertex: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawRangeElementsBaseVertex()");
        (self.glDrawRangeElementsBaseVertex)(mode, start, end, count, r#type, indices, basevertex)
    }
    pub unsafe fn DrawTransformFeedback(&self, mode: GLenum, id: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawTransformFeedback()");
        (self.glDrawTransformFeedback)(mode, id)
    }
    pub unsafe fn DrawTransformFeedbackInstanced(
        &self,
        mode: GLenum,
        id: GLuint,
        instancecount: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawTransformFeedbackInstanced()");
        (self.glDrawTransformFeedbackInstanced)(mode, id, instancecount)
    }
    pub unsafe fn DrawTransformFeedbackStream(&self, mode: GLenum, id: GLuint, stream: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawTransformFeedbackStream()");
        (self.glDrawTransformFeedbackStream)(mode, id, stream)
    }
    pub unsafe fn DrawTransformFeedbackStreamInstanced(
        &self,
        mode: GLenum,
        id: GLuint,
        stream: GLuint,
        instancecount: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling DrawTransformFeedbackStreamInstanced()");
        (self.glDrawTransformFeedbackStreamInstanced)(mode, id, stream, instancecount)
    }
    pub unsafe fn Enable(&self, cap: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Enable()");
        (self.glEnable)(cap)
    }
    pub unsafe fn EnableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling EnableVertexArrayAttrib()");
        (self.glEnableVertexArrayAttrib)(vaobj, index)
    }
    pub unsafe fn EnableVertexAttribArray(&self, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling EnableVertexAttribArray()");
        (self.glEnableVertexAttribArray)(index)
    }
    pub unsafe fn Enablei(&self, target: GLenum, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Enablei()");
        (self.glEnablei)(target, index)
    }
    pub unsafe fn EndConditionalRender(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling EndConditionalRender()");
        (self.glEndConditionalRender)()
    }
    pub unsafe fn EndQuery(&self, target: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling EndQuery()");
        (self.glEndQuery)(target)
    }
    pub unsafe fn EndQueryIndexed(&self, target: GLenum, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling EndQueryIndexed()");
        (self.glEndQueryIndexed)(target, index)
    }
    pub unsafe fn EndTransformFeedback(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling EndTransformFeedback()");
        (self.glEndTransformFeedback)()
    }
    pub unsafe fn FenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FenceSync()");
        (self.glFenceSync)(condition, flags)
    }
    pub unsafe fn Finish(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Finish()");
        (self.glFinish)()
    }
    pub unsafe fn Flush(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Flush()");
        (self.glFlush)()
    }
    pub unsafe fn FlushMappedBufferRange(
        &self,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FlushMappedBufferRange()");
        (self.glFlushMappedBufferRange)(target, offset, length)
    }
    pub unsafe fn FlushMappedNamedBufferRange(
        &self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FlushMappedNamedBufferRange()");
        (self.glFlushMappedNamedBufferRange)(buffer, offset, length)
    }
    pub unsafe fn FramebufferParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FramebufferParameteri()");
        (self.glFramebufferParameteri)(target, pname, param)
    }
    pub unsafe fn FramebufferRenderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FramebufferRenderbuffer()");
        (self.glFramebufferRenderbuffer)(target, attachment, renderbuffertarget, renderbuffer)
    }
    pub unsafe fn FramebufferTexture(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FramebufferTexture()");
        (self.glFramebufferTexture)(target, attachment, texture, level)
    }
    pub unsafe fn FramebufferTexture1D(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FramebufferTexture1D()");
        (self.glFramebufferTexture1D)(target, attachment, textarget, texture, level)
    }
    pub unsafe fn FramebufferTexture2D(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FramebufferTexture2D()");
        (self.glFramebufferTexture2D)(target, attachment, textarget, texture, level)
    }
    pub unsafe fn FramebufferTexture3D(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
        zoffset: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FramebufferTexture3D()");
        (self.glFramebufferTexture3D)(target, attachment, textarget, texture, level, zoffset)
    }
    pub unsafe fn FramebufferTextureLayer(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FramebufferTextureLayer()");
        (self.glFramebufferTextureLayer)(target, attachment, texture, level, layer)
    }
    pub unsafe fn FrontFace(&self, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling FrontFace()");
        (self.glFrontFace)(mode)
    }
    pub unsafe fn GenBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenBuffers()");
        (self.glGenBuffers)(n, buffers)
    }
    pub unsafe fn GenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenFramebuffers()");
        (self.glGenFramebuffers)(n, framebuffers)
    }
    pub unsafe fn GenProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenProgramPipelines()");
        (self.glGenProgramPipelines)(n, pipelines)
    }
    pub unsafe fn GenQueries(&self, n: GLsizei, ids: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenQueries()");
        (self.glGenQueries)(n, ids)
    }
    pub unsafe fn GenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenRenderbuffers()");
        (self.glGenRenderbuffers)(n, renderbuffers)
    }
    pub unsafe fn GenSamplers(&self, count: GLsizei, samplers: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenSamplers()");
        (self.glGenSamplers)(count, samplers)
    }
    pub unsafe fn GenTextures(&self, n: GLsizei, textures: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenTextures()");
        (self.glGenTextures)(n, textures)
    }
    pub unsafe fn GenTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenTransformFeedbacks()");
        (self.glGenTransformFeedbacks)(n, ids)
    }
    pub unsafe fn GenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenVertexArrays()");
        (self.glGenVertexArrays)(n, arrays)
    }
    pub unsafe fn GenerateMipmap(&self, target: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenerateMipmap()");
        (self.glGenerateMipmap)(target)
    }
    pub unsafe fn GenerateTextureMipmap(&self, texture: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GenerateTextureMipmap()");
        (self.glGenerateTextureMipmap)(texture)
    }
    pub unsafe fn GetActiveAtomicCounterBufferiv(
        &self,
        program: GLuint,
        bufferIndex: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveAtomicCounterBufferiv()");
        (self.glGetActiveAtomicCounterBufferiv)(program, bufferIndex, pname, params)
    }
    pub unsafe fn GetActiveAttrib(
        &self,
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        r#type: *mut GLenum,
        name: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveAttrib()");
        (self.glGetActiveAttrib)(program, index, bufSize, length, size, r#type, name)
    }
    pub unsafe fn GetActiveSubroutineName(
        &self,
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveSubroutineName()");
        (self.glGetActiveSubroutineName)(program, shadertype, index, bufSize, length, name)
    }
    pub unsafe fn GetActiveSubroutineUniformName(
        &self,
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveSubroutineUniformName()");
        (self.glGetActiveSubroutineUniformName)(program, shadertype, index, bufSize, length, name)
    }
    pub unsafe fn GetActiveSubroutineUniformiv(
        &self,
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        pname: GLenum,
        values: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveSubroutineUniformiv()");
        (self.glGetActiveSubroutineUniformiv)(program, shadertype, index, pname, values)
    }
    pub unsafe fn GetActiveUniform(
        &self,
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        r#type: *mut GLenum,
        name: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveUniform()");
        (self.glGetActiveUniform)(program, index, bufSize, length, size, r#type, name)
    }
    pub unsafe fn GetActiveUniformBlockName(
        &self,
        program: GLuint,
        uniformBlockIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformBlockName: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveUniformBlockName()");
        (self.glGetActiveUniformBlockName)(
            program,
            uniformBlockIndex,
            bufSize,
            length,
            uniformBlockName,
        )
    }
    pub unsafe fn GetActiveUniformBlockiv(
        &self,
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveUniformBlockiv()");
        (self.glGetActiveUniformBlockiv)(program, uniformBlockIndex, pname, params)
    }
    pub unsafe fn GetActiveUniformName(
        &self,
        program: GLuint,
        uniformIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformName: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveUniformName()");
        (self.glGetActiveUniformName)(program, uniformIndex, bufSize, length, uniformName)
    }
    pub unsafe fn GetActiveUniformsiv(
        &self,
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetActiveUniformsiv()");
        (self.glGetActiveUniformsiv)(program, uniformCount, uniformIndices, pname, params)
    }
    pub unsafe fn GetAttachedShaders(
        &self,
        program: GLuint,
        maxCount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetAttachedShaders()");
        (self.glGetAttachedShaders)(program, maxCount, count, shaders)
    }
    pub unsafe fn GetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetAttribLocation()");
        (self.glGetAttribLocation)(program, name)
    }
    pub unsafe fn GetBooleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetBooleani_v()");
        (self.glGetBooleani_v)(target, index, data)
    }
    pub unsafe fn GetBooleanv(&self, pname: GLenum, data: *mut GLboolean) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetBooleanv()");
        (self.glGetBooleanv)(pname, data)
    }
    pub unsafe fn GetBufferParameteri64v(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint64,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetBufferParameteri64v()");
        (self.glGetBufferParameteri64v)(target, pname, params)
    }
    pub unsafe fn GetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetBufferParameteriv()");
        (self.glGetBufferParameteriv)(target, pname, params)
    }
    pub unsafe fn GetBufferPointerv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetBufferPointerv()");
        (self.glGetBufferPointerv)(target, pname, params)
    }
    pub unsafe fn GetBufferSubData(
        &self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetBufferSubData()");
        (self.glGetBufferSubData)(target, offset, size, data)
    }
    pub unsafe fn GetCompressedTexImage(&self, target: GLenum, level: GLint, img: *mut c_void) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetCompressedTexImage()");
        (self.glGetCompressedTexImage)(target, level, img)
    }
    pub unsafe fn GetCompressedTextureImage(
        &self,
        texture: GLuint,
        level: GLint,
        bufSize: GLsizei,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetCompressedTextureImage()");
        (self.glGetCompressedTextureImage)(texture, level, bufSize, pixels)
    }
    pub unsafe fn GetCompressedTextureSubImage(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        bufSize: GLsizei,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetCompressedTextureSubImage()");
        (self.glGetCompressedTextureSubImage)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels,
        )
    }
    pub unsafe fn GetDebugMessageLog(
        &self,
        count: GLuint,
        bufSize: GLsizei,
        sources: *mut GLenum,
        types: *mut GLenum,
        ids: *mut GLuint,
        severities: *mut GLenum,
        lengths: *mut GLsizei,
        messageLog: *mut GLchar,
    ) -> GLuint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetDebugMessageLog()");
        (self.glGetDebugMessageLog)(
            count, bufSize, sources, types, ids, severities, lengths, messageLog,
        )
    }
    pub unsafe fn GetDoublei_v(&self, target: GLenum, index: GLuint, data: *mut GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetDoublei_v()");
        (self.glGetDoublei_v)(target, index, data)
    }
    pub unsafe fn GetDoublev(&self, pname: GLenum, data: *mut GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetDoublev()");
        (self.glGetDoublev)(pname, data)
    }
    pub unsafe fn GetError(&self) -> GLenum {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetError()");
        (self.glGetError)()
    }
    pub unsafe fn GetFloati_v(&self, target: GLenum, index: GLuint, data: *mut GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetFloati_v()");
        (self.glGetFloati_v)(target, index, data)
    }
    pub unsafe fn GetFloatv(&self, pname: GLenum, data: *mut GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetFloatv()");
        (self.glGetFloatv)(pname, data)
    }
    pub unsafe fn GetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetFragDataIndex()");
        (self.glGetFragDataIndex)(program, name)
    }
    pub unsafe fn GetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetFragDataLocation()");
        (self.glGetFragDataLocation)(program, name)
    }
    pub unsafe fn GetFramebufferAttachmentParameteriv(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetFramebufferAttachmentParameteriv()");
        (self.glGetFramebufferAttachmentParameteriv)(target, attachment, pname, params)
    }
    pub unsafe fn GetFramebufferParameteriv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetFramebufferParameteriv()");
        (self.glGetFramebufferParameteriv)(target, pname, params)
    }
    pub unsafe fn GetGraphicsResetStatus(&self) -> GLenum {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetGraphicsResetStatus()");
        (self.glGetGraphicsResetStatus)()
    }
    pub unsafe fn GetInteger64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetInteger64i_v()");
        (self.glGetInteger64i_v)(target, index, data)
    }
    pub unsafe fn GetInteger64v(&self, pname: GLenum, data: *mut GLint64) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetInteger64v()");
        (self.glGetInteger64v)(pname, data)
    }
    pub unsafe fn GetIntegeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetIntegeri_v()");
        (self.glGetIntegeri_v)(target, index, data)
    }
    pub unsafe fn GetIntegerv(&self, pname: GLenum, data: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetIntegerv()");
        (self.glGetIntegerv)(pname, data)
    }
    pub unsafe fn GetInternalformati64v(
        &self,
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        count: GLsizei,
        params: *mut GLint64,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetInternalformati64v()");
        (self.glGetInternalformati64v)(target, internalformat, pname, count, params)
    }
    pub unsafe fn GetInternalformativ(
        &self,
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        count: GLsizei,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetInternalformativ()");
        (self.glGetInternalformativ)(target, internalformat, pname, count, params)
    }
    pub unsafe fn GetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetMultisamplefv()");
        (self.glGetMultisamplefv)(pname, index, val)
    }
    pub unsafe fn GetNamedBufferParameteri64v(
        &self,
        buffer: GLuint,
        pname: GLenum,
        params: *mut GLint64,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetNamedBufferParameteri64v()");
        (self.glGetNamedBufferParameteri64v)(buffer, pname, params)
    }
    pub unsafe fn GetNamedBufferParameteriv(
        &self,
        buffer: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetNamedBufferParameteriv()");
        (self.glGetNamedBufferParameteriv)(buffer, pname, params)
    }
    pub unsafe fn GetNamedBufferPointerv(
        &self,
        buffer: GLuint,
        pname: GLenum,
        params: *mut *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetNamedBufferPointerv()");
        (self.glGetNamedBufferPointerv)(buffer, pname, params)
    }
    pub unsafe fn GetNamedBufferSubData(
        &self,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetNamedBufferSubData()");
        (self.glGetNamedBufferSubData)(buffer, offset, size, data)
    }
    pub unsafe fn GetNamedFramebufferAttachmentParameteriv(
        &self,
        framebuffer: GLuint,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetNamedFramebufferAttachmentParameteriv()");
        (self.glGetNamedFramebufferAttachmentParameteriv)(framebuffer, attachment, pname, params)
    }
    pub unsafe fn GetNamedFramebufferParameteriv(
        &self,
        framebuffer: GLuint,
        pname: GLenum,
        param: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetNamedFramebufferParameteriv()");
        (self.glGetNamedFramebufferParameteriv)(framebuffer, pname, param)
    }
    pub unsafe fn GetNamedRenderbufferParameteriv(
        &self,
        renderbuffer: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetNamedRenderbufferParameteriv()");
        (self.glGetNamedRenderbufferParameteriv)(renderbuffer, pname, params)
    }
    pub unsafe fn GetObjectLabel(
        &self,
        identifier: GLenum,
        name: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetObjectLabel()");
        (self.glGetObjectLabel)(identifier, name, bufSize, length, label)
    }
    pub unsafe fn GetObjectPtrLabel(
        &self,
        ptr: *const c_void,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetObjectPtrLabel()");
        (self.glGetObjectPtrLabel)(ptr, bufSize, length, label)
    }
    pub unsafe fn GetPointerv(&self, pname: GLenum, params: *mut *mut c_void) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetPointerv()");
        (self.glGetPointerv)(pname, params)
    }
    pub unsafe fn GetProgramBinary(
        &self,
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        binaryFormat: *mut GLenum,
        binary: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramBinary()");
        (self.glGetProgramBinary)(program, bufSize, length, binaryFormat, binary)
    }
    pub unsafe fn GetProgramInfoLog(
        &self,
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramInfoLog()");
        (self.glGetProgramInfoLog)(program, bufSize, length, infoLog)
    }
    pub unsafe fn GetProgramInterfaceiv(
        &self,
        program: GLuint,
        programInterface: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramInterfaceiv()");
        (self.glGetProgramInterfaceiv)(program, programInterface, pname, params)
    }
    pub unsafe fn GetProgramPipelineInfoLog(
        &self,
        pipeline: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramPipelineInfoLog()");
        (self.glGetProgramPipelineInfoLog)(pipeline, bufSize, length, infoLog)
    }
    pub unsafe fn GetProgramPipelineiv(&self, pipeline: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramPipelineiv()");
        (self.glGetProgramPipelineiv)(pipeline, pname, params)
    }
    pub unsafe fn GetProgramResourceIndex(
        &self,
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar,
    ) -> GLuint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramResourceIndex()");
        (self.glGetProgramResourceIndex)(program, programInterface, name)
    }
    pub unsafe fn GetProgramResourceLocation(
        &self,
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar,
    ) -> GLint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramResourceLocation()");
        (self.glGetProgramResourceLocation)(program, programInterface, name)
    }
    pub unsafe fn GetProgramResourceLocationIndex(
        &self,
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar,
    ) -> GLint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramResourceLocationIndex()");
        (self.glGetProgramResourceLocationIndex)(program, programInterface, name)
    }
    pub unsafe fn GetProgramResourceName(
        &self,
        program: GLuint,
        programInterface: GLenum,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramResourceName()");
        (self.glGetProgramResourceName)(program, programInterface, index, bufSize, length, name)
    }
    pub unsafe fn GetProgramResourceiv(
        &self,
        program: GLuint,
        programInterface: GLenum,
        index: GLuint,
        propCount: GLsizei,
        props: *const GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramResourceiv()");
        (self.glGetProgramResourceiv)(
            program,
            programInterface,
            index,
            propCount,
            props,
            count,
            length,
            params,
        )
    }
    pub unsafe fn GetProgramStageiv(
        &self,
        program: GLuint,
        shadertype: GLenum,
        pname: GLenum,
        values: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramStageiv()");
        (self.glGetProgramStageiv)(program, shadertype, pname, values)
    }
    pub unsafe fn GetProgramiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetProgramiv()");
        (self.glGetProgramiv)(program, pname, params)
    }
    pub unsafe fn GetQueryBufferObjecti64v(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryBufferObjecti64v()");
        (self.glGetQueryBufferObjecti64v)(id, buffer, pname, offset)
    }
    pub unsafe fn GetQueryBufferObjectiv(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryBufferObjectiv()");
        (self.glGetQueryBufferObjectiv)(id, buffer, pname, offset)
    }
    pub unsafe fn GetQueryBufferObjectui64v(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryBufferObjectui64v()");
        (self.glGetQueryBufferObjectui64v)(id, buffer, pname, offset)
    }
    pub unsafe fn GetQueryBufferObjectuiv(
        &self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryBufferObjectuiv()");
        (self.glGetQueryBufferObjectuiv)(id, buffer, pname, offset)
    }
    pub unsafe fn GetQueryIndexediv(
        &self,
        target: GLenum,
        index: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryIndexediv()");
        (self.glGetQueryIndexediv)(target, index, pname, params)
    }
    pub unsafe fn GetQueryObjecti64v(&self, id: GLuint, pname: GLenum, params: *mut GLint64) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryObjecti64v()");
        (self.glGetQueryObjecti64v)(id, pname, params)
    }
    pub unsafe fn GetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryObjectiv()");
        (self.glGetQueryObjectiv)(id, pname, params)
    }
    pub unsafe fn GetQueryObjectui64v(&self, id: GLuint, pname: GLenum, params: *mut GLuint64) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryObjectui64v()");
        (self.glGetQueryObjectui64v)(id, pname, params)
    }
    pub unsafe fn GetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryObjectuiv()");
        (self.glGetQueryObjectuiv)(id, pname, params)
    }
    pub unsafe fn GetQueryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetQueryiv()");
        (self.glGetQueryiv)(target, pname, params)
    }
    pub unsafe fn GetRenderbufferParameteriv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetRenderbufferParameteriv()");
        (self.glGetRenderbufferParameteriv)(target, pname, params)
    }
    pub unsafe fn GetSamplerParameterIiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetSamplerParameterIiv()");
        (self.glGetSamplerParameterIiv)(sampler, pname, params)
    }
    pub unsafe fn GetSamplerParameterIuiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetSamplerParameterIuiv()");
        (self.glGetSamplerParameterIuiv)(sampler, pname, params)
    }
    pub unsafe fn GetSamplerParameterfv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetSamplerParameterfv()");
        (self.glGetSamplerParameterfv)(sampler, pname, params)
    }
    pub unsafe fn GetSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetSamplerParameteriv()");
        (self.glGetSamplerParameteriv)(sampler, pname, params)
    }
    pub unsafe fn GetShaderInfoLog(
        &self,
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetShaderInfoLog()");
        (self.glGetShaderInfoLog)(shader, bufSize, length, infoLog)
    }
    pub unsafe fn GetShaderPrecisionFormat(
        &self,
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetShaderPrecisionFormat()");
        (self.glGetShaderPrecisionFormat)(shadertype, precisiontype, range, precision)
    }
    pub unsafe fn GetShaderSource(
        &self,
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetShaderSource()");
        (self.glGetShaderSource)(shader, bufSize, length, source)
    }
    pub unsafe fn GetShaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetShaderiv()");
        (self.glGetShaderiv)(shader, pname, params)
    }
    pub unsafe fn GetString(&self, name: GLenum) -> *const GLubyte {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetString()");
        (self.glGetString)(name)
    }
    pub unsafe fn GetStringi(&self, name: GLenum, index: GLuint) -> *const GLubyte {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetStringi()");
        (self.glGetStringi)(name, index)
    }
    pub unsafe fn GetSubroutineIndex(
        &self,
        program: GLuint,
        shadertype: GLenum,
        name: *const GLchar,
    ) -> GLuint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetSubroutineIndex()");
        (self.glGetSubroutineIndex)(program, shadertype, name)
    }
    pub unsafe fn GetSubroutineUniformLocation(
        &self,
        program: GLuint,
        shadertype: GLenum,
        name: *const GLchar,
    ) -> GLint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetSubroutineUniformLocation()");
        (self.glGetSubroutineUniformLocation)(program, shadertype, name)
    }
    pub unsafe fn GetSynciv(
        &self,
        sync: GLsync,
        pname: GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetSynciv()");
        (self.glGetSynciv)(sync, pname, count, length, values)
    }
    pub unsafe fn GetTexImage(
        &self,
        target: GLenum,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTexImage()");
        (self.glGetTexImage)(target, level, format, r#type, pixels)
    }
    pub unsafe fn GetTexLevelParameterfv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTexLevelParameterfv()");
        (self.glGetTexLevelParameterfv)(target, level, pname, params)
    }
    pub unsafe fn GetTexLevelParameteriv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTexLevelParameteriv()");
        (self.glGetTexLevelParameteriv)(target, level, pname, params)
    }
    pub unsafe fn GetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTexParameterIiv()");
        (self.glGetTexParameterIiv)(target, pname, params)
    }
    pub unsafe fn GetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTexParameterIuiv()");
        (self.glGetTexParameterIuiv)(target, pname, params)
    }
    pub unsafe fn GetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTexParameterfv()");
        (self.glGetTexParameterfv)(target, pname, params)
    }
    pub unsafe fn GetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTexParameteriv()");
        (self.glGetTexParameteriv)(target, pname, params)
    }
    pub unsafe fn GetTextureImage(
        &self,
        texture: GLuint,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        bufSize: GLsizei,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureImage()");
        (self.glGetTextureImage)(texture, level, format, r#type, bufSize, pixels)
    }
    pub unsafe fn GetTextureLevelParameterfv(
        &self,
        texture: GLuint,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureLevelParameterfv()");
        (self.glGetTextureLevelParameterfv)(texture, level, pname, params)
    }
    pub unsafe fn GetTextureLevelParameteriv(
        &self,
        texture: GLuint,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureLevelParameteriv()");
        (self.glGetTextureLevelParameteriv)(texture, level, pname, params)
    }
    pub unsafe fn GetTextureParameterIiv(
        &self,
        texture: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureParameterIiv()");
        (self.glGetTextureParameterIiv)(texture, pname, params)
    }
    pub unsafe fn GetTextureParameterIuiv(
        &self,
        texture: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureParameterIuiv()");
        (self.glGetTextureParameterIuiv)(texture, pname, params)
    }
    pub unsafe fn GetTextureParameterfv(
        &self,
        texture: GLuint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureParameterfv()");
        (self.glGetTextureParameterfv)(texture, pname, params)
    }
    pub unsafe fn GetTextureParameteriv(&self, texture: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureParameteriv()");
        (self.glGetTextureParameteriv)(texture, pname, params)
    }
    pub unsafe fn GetTextureSubImage(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        bufSize: GLsizei,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTextureSubImage()");
        (self.glGetTextureSubImage)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type,
            bufSize, pixels,
        )
    }
    pub unsafe fn GetTransformFeedbackVarying(
        &self,
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        r#type: *mut GLenum,
        name: *mut GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTransformFeedbackVarying()");
        (self.glGetTransformFeedbackVarying)(program, index, bufSize, length, size, r#type, name)
    }
    pub unsafe fn GetTransformFeedbacki64_v(
        &self,
        xfb: GLuint,
        pname: GLenum,
        index: GLuint,
        param: *mut GLint64,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTransformFeedbacki64_v()");
        (self.glGetTransformFeedbacki64_v)(xfb, pname, index, param)
    }
    pub unsafe fn GetTransformFeedbacki_v(
        &self,
        xfb: GLuint,
        pname: GLenum,
        index: GLuint,
        param: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTransformFeedbacki_v()");
        (self.glGetTransformFeedbacki_v)(xfb, pname, index, param)
    }
    pub unsafe fn GetTransformFeedbackiv(&self, xfb: GLuint, pname: GLenum, param: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetTransformFeedbackiv()");
        (self.glGetTransformFeedbackiv)(xfb, pname, param)
    }
    pub unsafe fn GetUniformBlockIndex(
        &self,
        program: GLuint,
        uniformBlockName: *const GLchar,
    ) -> GLuint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformBlockIndex()");
        (self.glGetUniformBlockIndex)(program, uniformBlockName)
    }
    pub unsafe fn GetUniformIndices(
        &self,
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformIndices()");
        (self.glGetUniformIndices)(program, uniformCount, uniformNames, uniformIndices)
    }
    pub unsafe fn GetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformLocation()");
        (self.glGetUniformLocation)(program, name)
    }
    pub unsafe fn GetUniformSubroutineuiv(
        &self,
        shadertype: GLenum,
        location: GLint,
        params: *mut GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformSubroutineuiv()");
        (self.glGetUniformSubroutineuiv)(shadertype, location, params)
    }
    pub unsafe fn GetUniformdv(&self, program: GLuint, location: GLint, params: *mut GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformdv()");
        (self.glGetUniformdv)(program, location, params)
    }
    pub unsafe fn GetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformfv()");
        (self.glGetUniformfv)(program, location, params)
    }
    pub unsafe fn GetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformiv()");
        (self.glGetUniformiv)(program, location, params)
    }
    pub unsafe fn GetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetUniformuiv()");
        (self.glGetUniformuiv)(program, location, params)
    }
    pub unsafe fn GetVertexArrayIndexed64iv(
        &self,
        vaobj: GLuint,
        index: GLuint,
        pname: GLenum,
        param: *mut GLint64,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexArrayIndexed64iv()");
        (self.glGetVertexArrayIndexed64iv)(vaobj, index, pname, param)
    }
    pub unsafe fn GetVertexArrayIndexediv(
        &self,
        vaobj: GLuint,
        index: GLuint,
        pname: GLenum,
        param: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexArrayIndexediv()");
        (self.glGetVertexArrayIndexediv)(vaobj, index, pname, param)
    }
    pub unsafe fn GetVertexArrayiv(&self, vaobj: GLuint, pname: GLenum, param: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexArrayiv()");
        (self.glGetVertexArrayiv)(vaobj, pname, param)
    }
    pub unsafe fn GetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexAttribIiv()");
        (self.glGetVertexAttribIiv)(index, pname, params)
    }
    pub unsafe fn GetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexAttribIuiv()");
        (self.glGetVertexAttribIuiv)(index, pname, params)
    }
    pub unsafe fn GetVertexAttribLdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexAttribLdv()");
        (self.glGetVertexAttribLdv)(index, pname, params)
    }
    pub unsafe fn GetVertexAttribPointerv(
        &self,
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexAttribPointerv()");
        (self.glGetVertexAttribPointerv)(index, pname, pointer)
    }
    pub unsafe fn GetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexAttribdv()");
        (self.glGetVertexAttribdv)(index, pname, params)
    }
    pub unsafe fn GetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexAttribfv()");
        (self.glGetVertexAttribfv)(index, pname, params)
    }
    pub unsafe fn GetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetVertexAttribiv()");
        (self.glGetVertexAttribiv)(index, pname, params)
    }
    pub unsafe fn GetnCompressedTexImage(
        &self,
        target: GLenum,
        lod: GLint,
        bufSize: GLsizei,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetnCompressedTexImage()");
        (self.glGetnCompressedTexImage)(target, lod, bufSize, pixels)
    }
    pub unsafe fn GetnTexImage(
        &self,
        target: GLenum,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        bufSize: GLsizei,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetnTexImage()");
        (self.glGetnTexImage)(target, level, format, r#type, bufSize, pixels)
    }
    pub unsafe fn GetnUniformdv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetnUniformdv()");
        (self.glGetnUniformdv)(program, location, bufSize, params)
    }
    pub unsafe fn GetnUniformfv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetnUniformfv()");
        (self.glGetnUniformfv)(program, location, bufSize, params)
    }
    pub unsafe fn GetnUniformiv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetnUniformiv()");
        (self.glGetnUniformiv)(program, location, bufSize, params)
    }
    pub unsafe fn GetnUniformuiv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling GetnUniformuiv()");
        (self.glGetnUniformuiv)(program, location, bufSize, params)
    }
    pub unsafe fn Hint(&self, target: GLenum, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Hint()");
        (self.glHint)(target, mode)
    }
    pub unsafe fn InvalidateBufferData(&self, buffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateBufferData()");
        (self.glInvalidateBufferData)(buffer)
    }
    pub unsafe fn InvalidateBufferSubData(
        &self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateBufferSubData()");
        (self.glInvalidateBufferSubData)(buffer, offset, length)
    }
    pub unsafe fn InvalidateFramebuffer(
        &self,
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateFramebuffer()");
        (self.glInvalidateFramebuffer)(target, numAttachments, attachments)
    }
    pub unsafe fn InvalidateNamedFramebufferData(
        &self,
        framebuffer: GLuint,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateNamedFramebufferData()");
        (self.glInvalidateNamedFramebufferData)(framebuffer, numAttachments, attachments)
    }
    pub unsafe fn InvalidateNamedFramebufferSubData(
        &self,
        framebuffer: GLuint,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateNamedFramebufferSubData()");
        (self.glInvalidateNamedFramebufferSubData)(
            framebuffer,
            numAttachments,
            attachments,
            x,
            y,
            width,
            height,
        )
    }
    pub unsafe fn InvalidateSubFramebuffer(
        &self,
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateSubFramebuffer()");
        (self.glInvalidateSubFramebuffer)(target, numAttachments, attachments, x, y, width, height)
    }
    pub unsafe fn InvalidateTexImage(&self, texture: GLuint, level: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateTexImage()");
        (self.glInvalidateTexImage)(texture, level)
    }
    pub unsafe fn InvalidateTexSubImage(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling InvalidateTexSubImage()");
        (self.glInvalidateTexSubImage)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth,
        )
    }
    pub unsafe fn IsBuffer(&self, buffer: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsBuffer()");
        (self.glIsBuffer)(buffer)
    }
    pub unsafe fn IsEnabled(&self, cap: GLenum) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsEnabled()");
        (self.glIsEnabled)(cap)
    }
    pub unsafe fn IsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsEnabledi()");
        (self.glIsEnabledi)(target, index)
    }
    pub unsafe fn IsFramebuffer(&self, framebuffer: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsFramebuffer()");
        (self.glIsFramebuffer)(framebuffer)
    }
    pub unsafe fn IsProgram(&self, program: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsProgram()");
        (self.glIsProgram)(program)
    }
    pub unsafe fn IsProgramPipeline(&self, pipeline: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsProgramPipeline()");
        (self.glIsProgramPipeline)(pipeline)
    }
    pub unsafe fn IsQuery(&self, id: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsQuery()");
        (self.glIsQuery)(id)
    }
    pub unsafe fn IsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsRenderbuffer()");
        (self.glIsRenderbuffer)(renderbuffer)
    }
    pub unsafe fn IsSampler(&self, sampler: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsSampler()");
        (self.glIsSampler)(sampler)
    }
    pub unsafe fn IsShader(&self, shader: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsShader()");
        (self.glIsShader)(shader)
    }
    pub unsafe fn IsSync(&self, sync: GLsync) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsSync()");
        (self.glIsSync)(sync)
    }
    pub unsafe fn IsTexture(&self, texture: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsTexture()");
        (self.glIsTexture)(texture)
    }
    pub unsafe fn IsTransformFeedback(&self, id: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsTransformFeedback()");
        (self.glIsTransformFeedback)(id)
    }
    pub unsafe fn IsVertexArray(&self, array: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling IsVertexArray()");
        (self.glIsVertexArray)(array)
    }
    pub unsafe fn LineWidth(&self, width: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling LineWidth()");
        (self.glLineWidth)(width)
    }
    pub unsafe fn LinkProgram(&self, program: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling LinkProgram()");
        (self.glLinkProgram)(program)
    }
    pub unsafe fn LogicOp(&self, opcode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling LogicOp()");
        (self.glLogicOp)(opcode)
    }
    pub unsafe fn MapBuffer(&self, target: GLenum, access: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MapBuffer()");
        (self.glMapBuffer)(target, access)
    }
    pub unsafe fn MapBufferRange(
        &self,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MapBufferRange()");
        (self.glMapBufferRange)(target, offset, length, access)
    }
    pub unsafe fn MapNamedBuffer(&self, buffer: GLuint, access: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MapNamedBuffer()");
        (self.glMapNamedBuffer)(buffer, access)
    }
    pub unsafe fn MapNamedBufferRange(
        &self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MapNamedBufferRange()");
        (self.glMapNamedBufferRange)(buffer, offset, length, access)
    }
    pub unsafe fn MemoryBarrier(&self, barriers: GLbitfield) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MemoryBarrier()");
        (self.glMemoryBarrier)(barriers)
    }
    pub unsafe fn MemoryBarrierByRegion(&self, barriers: GLbitfield) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MemoryBarrierByRegion()");
        (self.glMemoryBarrierByRegion)(barriers)
    }
    pub unsafe fn MinSampleShading(&self, value: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MinSampleShading()");
        (self.glMinSampleShading)(value)
    }
    pub unsafe fn MultiDrawArrays(
        &self,
        mode: GLenum,
        first: *const GLint,
        count: *const GLsizei,
        drawcount: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MultiDrawArrays()");
        (self.glMultiDrawArrays)(mode, first, count, drawcount)
    }
    pub unsafe fn MultiDrawArraysIndirect(
        &self,
        mode: GLenum,
        indirect: *const c_void,
        drawcount: GLsizei,
        stride: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MultiDrawArraysIndirect()");
        (self.glMultiDrawArraysIndirect)(mode, indirect, drawcount, stride)
    }
    pub unsafe fn MultiDrawArraysIndirectCount(
        &self,
        mode: GLenum,
        indirect: *const c_void,
        drawcount: GLintptr,
        maxdrawcount: GLsizei,
        stride: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MultiDrawArraysIndirectCount()");
        (self.glMultiDrawArraysIndirectCount)(mode, indirect, drawcount, maxdrawcount, stride)
    }
    pub unsafe fn MultiDrawElements(
        &self,
        mode: GLenum,
        count: *const GLsizei,
        r#type: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MultiDrawElements()");
        (self.glMultiDrawElements)(mode, count, r#type, indices, drawcount)
    }
    pub unsafe fn MultiDrawElementsBaseVertex(
        &self,
        mode: GLenum,
        count: *const GLsizei,
        r#type: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei,
        basevertex: *const GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MultiDrawElementsBaseVertex()");
        (self.glMultiDrawElementsBaseVertex)(mode, count, r#type, indices, drawcount, basevertex)
    }
    pub unsafe fn MultiDrawElementsIndirect(
        &self,
        mode: GLenum,
        r#type: GLenum,
        indirect: *const c_void,
        drawcount: GLsizei,
        stride: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MultiDrawElementsIndirect()");
        (self.glMultiDrawElementsIndirect)(mode, r#type, indirect, drawcount, stride)
    }
    pub unsafe fn MultiDrawElementsIndirectCount(
        &self,
        mode: GLenum,
        r#type: GLenum,
        indirect: *const c_void,
        drawcount: GLintptr,
        maxdrawcount: GLsizei,
        stride: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling MultiDrawElementsIndirectCount()");
        (self.glMultiDrawElementsIndirectCount)(
            mode,
            r#type,
            indirect,
            drawcount,
            maxdrawcount,
            stride,
        )
    }
    pub unsafe fn NamedBufferData(
        &self,
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedBufferData()");
        (self.glNamedBufferData)(buffer, size, data, usage)
    }
    pub unsafe fn NamedBufferStorage(
        &self,
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const c_void,
        flags: GLbitfield,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedBufferStorage()");
        (self.glNamedBufferStorage)(buffer, size, data, flags)
    }
    pub unsafe fn NamedBufferSubData(
        &self,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedBufferSubData()");
        (self.glNamedBufferSubData)(buffer, offset, size, data)
    }
    pub unsafe fn NamedFramebufferDrawBuffer(&self, framebuffer: GLuint, buf: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedFramebufferDrawBuffer()");
        (self.glNamedFramebufferDrawBuffer)(framebuffer, buf)
    }
    pub unsafe fn NamedFramebufferDrawBuffers(
        &self,
        framebuffer: GLuint,
        n: GLsizei,
        bufs: *const GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedFramebufferDrawBuffers()");
        (self.glNamedFramebufferDrawBuffers)(framebuffer, n, bufs)
    }
    pub unsafe fn NamedFramebufferParameteri(
        &self,
        framebuffer: GLuint,
        pname: GLenum,
        param: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedFramebufferParameteri()");
        (self.glNamedFramebufferParameteri)(framebuffer, pname, param)
    }
    pub unsafe fn NamedFramebufferReadBuffer(&self, framebuffer: GLuint, src: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedFramebufferReadBuffer()");
        (self.glNamedFramebufferReadBuffer)(framebuffer, src)
    }
    pub unsafe fn NamedFramebufferRenderbuffer(
        &self,
        framebuffer: GLuint,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedFramebufferRenderbuffer()");
        (self.glNamedFramebufferRenderbuffer)(
            framebuffer,
            attachment,
            renderbuffertarget,
            renderbuffer,
        )
    }
    pub unsafe fn NamedFramebufferTexture(
        &self,
        framebuffer: GLuint,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedFramebufferTexture()");
        (self.glNamedFramebufferTexture)(framebuffer, attachment, texture, level)
    }
    pub unsafe fn NamedFramebufferTextureLayer(
        &self,
        framebuffer: GLuint,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedFramebufferTextureLayer()");
        (self.glNamedFramebufferTextureLayer)(framebuffer, attachment, texture, level, layer)
    }
    pub unsafe fn NamedRenderbufferStorage(
        &self,
        renderbuffer: GLuint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedRenderbufferStorage()");
        (self.glNamedRenderbufferStorage)(renderbuffer, internalformat, width, height)
    }
    pub unsafe fn NamedRenderbufferStorageMultisample(
        &self,
        renderbuffer: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling NamedRenderbufferStorageMultisample()");
        (self.glNamedRenderbufferStorageMultisample)(
            renderbuffer,
            samples,
            internalformat,
            width,
            height,
        )
    }
    pub unsafe fn ObjectLabel(
        &self,
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ObjectLabel()");
        (self.glObjectLabel)(identifier, name, length, label)
    }
    pub unsafe fn ObjectPtrLabel(&self, ptr: *const c_void, length: GLsizei, label: *const GLchar) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ObjectPtrLabel()");
        (self.glObjectPtrLabel)(ptr, length, label)
    }
    pub unsafe fn PatchParameterfv(&self, pname: GLenum, values: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PatchParameterfv()");
        (self.glPatchParameterfv)(pname, values)
    }
    pub unsafe fn PatchParameteri(&self, pname: GLenum, value: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PatchParameteri()");
        (self.glPatchParameteri)(pname, value)
    }
    pub unsafe fn PauseTransformFeedback(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PauseTransformFeedback()");
        (self.glPauseTransformFeedback)()
    }
    pub unsafe fn PixelStoref(&self, pname: GLenum, param: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PixelStoref()");
        (self.glPixelStoref)(pname, param)
    }
    pub unsafe fn PixelStorei(&self, pname: GLenum, param: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PixelStorei()");
        (self.glPixelStorei)(pname, param)
    }
    pub unsafe fn PointParameterf(&self, pname: GLenum, param: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PointParameterf()");
        (self.glPointParameterf)(pname, param)
    }
    pub unsafe fn PointParameterfv(&self, pname: GLenum, params: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PointParameterfv()");
        (self.glPointParameterfv)(pname, params)
    }
    pub unsafe fn PointParameteri(&self, pname: GLenum, param: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PointParameteri()");
        (self.glPointParameteri)(pname, param)
    }
    pub unsafe fn PointParameteriv(&self, pname: GLenum, params: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PointParameteriv()");
        (self.glPointParameteriv)(pname, params)
    }
    pub unsafe fn PointSize(&self, size: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PointSize()");
        (self.glPointSize)(size)
    }
    pub unsafe fn PolygonMode(&self, face: GLenum, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PolygonMode()");
        (self.glPolygonMode)(face, mode)
    }
    pub unsafe fn PolygonOffset(&self, factor: GLfloat, units: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PolygonOffset()");
        (self.glPolygonOffset)(factor, units)
    }
    pub unsafe fn PolygonOffsetClamp(&self, factor: GLfloat, units: GLfloat, clamp: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PolygonOffsetClamp()");
        (self.glPolygonOffsetClamp)(factor, units, clamp)
    }
    pub unsafe fn PopDebugGroup(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PopDebugGroup()");
        (self.glPopDebugGroup)()
    }
    pub unsafe fn PrimitiveRestartIndex(&self, index: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PrimitiveRestartIndex()");
        (self.glPrimitiveRestartIndex)(index)
    }
    pub unsafe fn ProgramBinary(
        &self,
        program: GLuint,
        binaryFormat: GLenum,
        binary: *const c_void,
        length: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramBinary()");
        (self.glProgramBinary)(program, binaryFormat, binary, length)
    }
    pub unsafe fn ProgramParameteri(&self, program: GLuint, pname: GLenum, value: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramParameteri()");
        (self.glProgramParameteri)(program, pname, value)
    }
    pub unsafe fn ProgramUniform1d(&self, program: GLuint, location: GLint, v0: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1d()");
        (self.glProgramUniform1d)(program, location, v0)
    }
    pub unsafe fn ProgramUniform1dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1dv()");
        (self.glProgramUniform1dv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform1f(&self, program: GLuint, location: GLint, v0: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1f()");
        (self.glProgramUniform1f)(program, location, v0)
    }
    pub unsafe fn ProgramUniform1fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1fv()");
        (self.glProgramUniform1fv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform1i(&self, program: GLuint, location: GLint, v0: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1i()");
        (self.glProgramUniform1i)(program, location, v0)
    }
    pub unsafe fn ProgramUniform1iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1iv()");
        (self.glProgramUniform1iv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform1ui(&self, program: GLuint, location: GLint, v0: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1ui()");
        (self.glProgramUniform1ui)(program, location, v0)
    }
    pub unsafe fn ProgramUniform1uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform1uiv()");
        (self.glProgramUniform1uiv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform2d(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2d()");
        (self.glProgramUniform2d)(program, location, v0, v1)
    }
    pub unsafe fn ProgramUniform2dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2dv()");
        (self.glProgramUniform2dv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform2f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2f()");
        (self.glProgramUniform2f)(program, location, v0, v1)
    }
    pub unsafe fn ProgramUniform2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2fv()");
        (self.glProgramUniform2fv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform2i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2i()");
        (self.glProgramUniform2i)(program, location, v0, v1)
    }
    pub unsafe fn ProgramUniform2iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2iv()");
        (self.glProgramUniform2iv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform2ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2ui()");
        (self.glProgramUniform2ui)(program, location, v0, v1)
    }
    pub unsafe fn ProgramUniform2uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform2uiv()");
        (self.glProgramUniform2uiv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform3d(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
        v2: GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3d()");
        (self.glProgramUniform3d)(program, location, v0, v1, v2)
    }
    pub unsafe fn ProgramUniform3dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3dv()");
        (self.glProgramUniform3dv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform3f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3f()");
        (self.glProgramUniform3f)(program, location, v0, v1, v2)
    }
    pub unsafe fn ProgramUniform3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3fv()");
        (self.glProgramUniform3fv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform3i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3i()");
        (self.glProgramUniform3i)(program, location, v0, v1, v2)
    }
    pub unsafe fn ProgramUniform3iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3iv()");
        (self.glProgramUniform3iv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform3ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3ui()");
        (self.glProgramUniform3ui)(program, location, v0, v1, v2)
    }
    pub unsafe fn ProgramUniform3uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform3uiv()");
        (self.glProgramUniform3uiv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform4d(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
        v2: GLdouble,
        v3: GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4d()");
        (self.glProgramUniform4d)(program, location, v0, v1, v2, v3)
    }
    pub unsafe fn ProgramUniform4dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4dv()");
        (self.glProgramUniform4dv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform4f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4f()");
        (self.glProgramUniform4f)(program, location, v0, v1, v2, v3)
    }
    pub unsafe fn ProgramUniform4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4fv()");
        (self.glProgramUniform4fv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform4i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4i()");
        (self.glProgramUniform4i)(program, location, v0, v1, v2, v3)
    }
    pub unsafe fn ProgramUniform4iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4iv()");
        (self.glProgramUniform4iv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniform4ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4ui()");
        (self.glProgramUniform4ui)(program, location, v0, v1, v2, v3)
    }
    pub unsafe fn ProgramUniform4uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniform4uiv()");
        (self.glProgramUniform4uiv)(program, location, count, value)
    }
    pub unsafe fn ProgramUniformMatrix2dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix2dv()");
        (self.glProgramUniformMatrix2dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix2fv()");
        (self.glProgramUniformMatrix2fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix2x3dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix2x3dv()");
        (self.glProgramUniformMatrix2x3dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix2x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix2x3fv()");
        (self.glProgramUniformMatrix2x3fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix2x4dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix2x4dv()");
        (self.glProgramUniformMatrix2x4dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix2x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix2x4fv()");
        (self.glProgramUniformMatrix2x4fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix3dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix3dv()");
        (self.glProgramUniformMatrix3dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix3fv()");
        (self.glProgramUniformMatrix3fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix3x2dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix3x2dv()");
        (self.glProgramUniformMatrix3x2dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix3x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix3x2fv()");
        (self.glProgramUniformMatrix3x2fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix3x4dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix3x4dv()");
        (self.glProgramUniformMatrix3x4dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix3x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix3x4fv()");
        (self.glProgramUniformMatrix3x4fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix4dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix4dv()");
        (self.glProgramUniformMatrix4dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix4fv()");
        (self.glProgramUniformMatrix4fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix4x2dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix4x2dv()");
        (self.glProgramUniformMatrix4x2dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix4x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix4x2fv()");
        (self.glProgramUniformMatrix4x2fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix4x3dv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix4x3dv()");
        (self.glProgramUniformMatrix4x3dv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProgramUniformMatrix4x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProgramUniformMatrix4x3fv()");
        (self.glProgramUniformMatrix4x3fv)(program, location, count, transpose, value)
    }
    pub unsafe fn ProvokingVertex(&self, mode: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ProvokingVertex()");
        (self.glProvokingVertex)(mode)
    }
    pub unsafe fn PushDebugGroup(
        &self,
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling PushDebugGroup()");
        (self.glPushDebugGroup)(source, id, length, message)
    }
    pub unsafe fn QueryCounter(&self, id: GLuint, target: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling QueryCounter()");
        (self.glQueryCounter)(id, target)
    }
    pub unsafe fn ReadBuffer(&self, src: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ReadBuffer()");
        (self.glReadBuffer)(src)
    }
    pub unsafe fn ReadPixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ReadPixels()");
        (self.glReadPixels)(x, y, width, height, format, r#type, pixels)
    }
    pub unsafe fn ReadnPixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        bufSize: GLsizei,
        data: *mut c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ReadnPixels()");
        (self.glReadnPixels)(x, y, width, height, format, r#type, bufSize, data)
    }
    pub unsafe fn ReleaseShaderCompiler(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ReleaseShaderCompiler()");
        (self.glReleaseShaderCompiler)()
    }
    pub unsafe fn RenderbufferStorage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling RenderbufferStorage()");
        (self.glRenderbufferStorage)(target, internalformat, width, height)
    }
    pub unsafe fn RenderbufferStorageMultisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling RenderbufferStorageMultisample()");
        (self.glRenderbufferStorageMultisample)(target, samples, internalformat, width, height)
    }
    pub unsafe fn ResumeTransformFeedback(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ResumeTransformFeedback()");
        (self.glResumeTransformFeedback)()
    }
    pub unsafe fn SampleCoverage(&self, value: GLfloat, invert: GLboolean) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SampleCoverage()");
        (self.glSampleCoverage)(value, invert)
    }
    pub unsafe fn SampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SampleMaski()");
        (self.glSampleMaski)(maskNumber, mask)
    }
    pub unsafe fn SamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SamplerParameterIiv()");
        (self.glSamplerParameterIiv)(sampler, pname, param)
    }
    pub unsafe fn SamplerParameterIuiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SamplerParameterIuiv()");
        (self.glSamplerParameterIuiv)(sampler, pname, param)
    }
    pub unsafe fn SamplerParameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SamplerParameterf()");
        (self.glSamplerParameterf)(sampler, pname, param)
    }
    pub unsafe fn SamplerParameterfv(&self, sampler: GLuint, pname: GLenum, param: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SamplerParameterfv()");
        (self.glSamplerParameterfv)(sampler, pname, param)
    }
    pub unsafe fn SamplerParameteri(&self, sampler: GLuint, pname: GLenum, param: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SamplerParameteri()");
        (self.glSamplerParameteri)(sampler, pname, param)
    }
    pub unsafe fn SamplerParameteriv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SamplerParameteriv()");
        (self.glSamplerParameteriv)(sampler, pname, param)
    }
    pub unsafe fn Scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Scissor()");
        (self.glScissor)(x, y, width, height)
    }
    pub unsafe fn ScissorArrayv(&self, first: GLuint, count: GLsizei, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ScissorArrayv()");
        (self.glScissorArrayv)(first, count, v)
    }
    pub unsafe fn ScissorIndexed(
        &self,
        index: GLuint,
        left: GLint,
        bottom: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ScissorIndexed()");
        (self.glScissorIndexed)(index, left, bottom, width, height)
    }
    pub unsafe fn ScissorIndexedv(&self, index: GLuint, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ScissorIndexedv()");
        (self.glScissorIndexedv)(index, v)
    }
    pub unsafe fn ShaderBinary(
        &self,
        count: GLsizei,
        shaders: *const GLuint,
        binaryFormat: GLenum,
        binary: *const c_void,
        length: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ShaderBinary()");
        (self.glShaderBinary)(count, shaders, binaryFormat, binary, length)
    }
    pub unsafe fn ShaderSource(
        &self,
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ShaderSource()");
        (self.glShaderSource)(shader, count, string, length)
    }
    pub unsafe fn ShaderStorageBlockBinding(
        &self,
        program: GLuint,
        storageBlockIndex: GLuint,
        storageBlockBinding: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ShaderStorageBlockBinding()");
        (self.glShaderStorageBlockBinding)(program, storageBlockIndex, storageBlockBinding)
    }
    pub unsafe fn SpecializeShader(
        &self,
        shader: GLuint,
        pEntryPoint: *const GLchar,
        numSpecializationConstants: GLuint,
        pConstantIndex: *const GLuint,
        pConstantValue: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling SpecializeShader()");
        (self.glSpecializeShader)(
            shader,
            pEntryPoint,
            numSpecializationConstants,
            pConstantIndex,
            pConstantValue,
        )
    }
    pub unsafe fn StencilFunc(&self, func: GLenum, r#ref: GLint, mask: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling StencilFunc()");
        (self.glStencilFunc)(func, r#ref, mask)
    }
    pub unsafe fn StencilFuncSeparate(
        &self,
        face: GLenum,
        func: GLenum,
        r#ref: GLint,
        mask: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling StencilFuncSeparate()");
        (self.glStencilFuncSeparate)(face, func, r#ref, mask)
    }
    pub unsafe fn StencilMask(&self, mask: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling StencilMask()");
        (self.glStencilMask)(mask)
    }
    pub unsafe fn StencilMaskSeparate(&self, face: GLenum, mask: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling StencilMaskSeparate()");
        (self.glStencilMaskSeparate)(face, mask)
    }
    pub unsafe fn StencilOp(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling StencilOp()");
        (self.glStencilOp)(fail, zfail, zpass)
    }
    pub unsafe fn StencilOpSeparate(
        &self,
        face: GLenum,
        sfail: GLenum,
        dpfail: GLenum,
        dppass: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling StencilOpSeparate()");
        (self.glStencilOpSeparate)(face, sfail, dpfail, dppass)
    }
    pub unsafe fn TexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexBuffer()");
        (self.glTexBuffer)(target, internalformat, buffer)
    }
    pub unsafe fn TexBufferRange(
        &self,
        target: GLenum,
        internalformat: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexBufferRange()");
        (self.glTexBufferRange)(target, internalformat, buffer, offset, size)
    }
    pub unsafe fn TexImage1D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        border: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexImage1D()");
        (self.glTexImage1D)(
            target,
            level,
            internalformat,
            width,
            border,
            format,
            r#type,
            pixels,
        )
    }
    pub unsafe fn TexImage2D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexImage2D()");
        (self.glTexImage2D)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            r#type,
            pixels,
        )
    }
    pub unsafe fn TexImage2DMultisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexImage2DMultisample()");
        (self.glTexImage2DMultisample)(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }
    pub unsafe fn TexImage3D(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexImage3D()");
        (self.glTexImage3D)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            r#type,
            pixels,
        )
    }
    pub unsafe fn TexImage3DMultisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexImage3DMultisample()");
        (self.glTexImage3DMultisample)(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }
    pub unsafe fn TexParameterIiv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexParameterIiv()");
        (self.glTexParameterIiv)(target, pname, params)
    }
    pub unsafe fn TexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexParameterIuiv()");
        (self.glTexParameterIuiv)(target, pname, params)
    }
    pub unsafe fn TexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexParameterf()");
        (self.glTexParameterf)(target, pname, param)
    }
    pub unsafe fn TexParameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexParameterfv()");
        (self.glTexParameterfv)(target, pname, params)
    }
    pub unsafe fn TexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexParameteri()");
        (self.glTexParameteri)(target, pname, param)
    }
    pub unsafe fn TexParameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexParameteriv()");
        (self.glTexParameteriv)(target, pname, params)
    }
    pub unsafe fn TexStorage1D(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexStorage1D()");
        (self.glTexStorage1D)(target, levels, internalformat, width)
    }
    pub unsafe fn TexStorage2D(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexStorage2D()");
        (self.glTexStorage2D)(target, levels, internalformat, width, height)
    }
    pub unsafe fn TexStorage2DMultisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexStorage2DMultisample()");
        (self.glTexStorage2DMultisample)(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }
    pub unsafe fn TexStorage3D(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexStorage3D()");
        (self.glTexStorage3D)(target, levels, internalformat, width, height, depth)
    }
    pub unsafe fn TexStorage3DMultisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexStorage3DMultisample()");
        (self.glTexStorage3DMultisample)(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }
    pub unsafe fn TexSubImage1D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexSubImage1D()");
        (self.glTexSubImage1D)(target, level, xoffset, width, format, r#type, pixels)
    }
    pub unsafe fn TexSubImage2D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexSubImage2D()");
        (self.glTexSubImage2D)(
            target, level, xoffset, yoffset, width, height, format, r#type, pixels,
        )
    }
    pub unsafe fn TexSubImage3D(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TexSubImage3D()");
        (self.glTexSubImage3D)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, pixels,
        )
    }
    pub unsafe fn TextureBarrier(&self) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureBarrier()");
        (self.glTextureBarrier)()
    }
    pub unsafe fn TextureBuffer(&self, texture: GLuint, internalformat: GLenum, buffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureBuffer()");
        (self.glTextureBuffer)(texture, internalformat, buffer)
    }
    pub unsafe fn TextureBufferRange(
        &self,
        texture: GLuint,
        internalformat: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureBufferRange()");
        (self.glTextureBufferRange)(texture, internalformat, buffer, offset, size)
    }
    pub unsafe fn TextureParameterIiv(&self, texture: GLuint, pname: GLenum, params: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureParameterIiv()");
        (self.glTextureParameterIiv)(texture, pname, params)
    }
    pub unsafe fn TextureParameterIuiv(
        &self,
        texture: GLuint,
        pname: GLenum,
        params: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureParameterIuiv()");
        (self.glTextureParameterIuiv)(texture, pname, params)
    }
    pub unsafe fn TextureParameterf(&self, texture: GLuint, pname: GLenum, param: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureParameterf()");
        (self.glTextureParameterf)(texture, pname, param)
    }
    pub unsafe fn TextureParameterfv(&self, texture: GLuint, pname: GLenum, param: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureParameterfv()");
        (self.glTextureParameterfv)(texture, pname, param)
    }
    pub unsafe fn TextureParameteri(&self, texture: GLuint, pname: GLenum, param: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureParameteri()");
        (self.glTextureParameteri)(texture, pname, param)
    }
    pub unsafe fn TextureParameteriv(&self, texture: GLuint, pname: GLenum, param: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureParameteriv()");
        (self.glTextureParameteriv)(texture, pname, param)
    }
    pub unsafe fn TextureStorage1D(
        &self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureStorage1D()");
        (self.glTextureStorage1D)(texture, levels, internalformat, width)
    }
    pub unsafe fn TextureStorage2D(
        &self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureStorage2D()");
        (self.glTextureStorage2D)(texture, levels, internalformat, width, height)
    }
    pub unsafe fn TextureStorage2DMultisample(
        &self,
        texture: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureStorage2DMultisample()");
        (self.glTextureStorage2DMultisample)(
            texture,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }
    pub unsafe fn TextureStorage3D(
        &self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureStorage3D()");
        (self.glTextureStorage3D)(texture, levels, internalformat, width, height, depth)
    }
    pub unsafe fn TextureStorage3DMultisample(
        &self,
        texture: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureStorage3DMultisample()");
        (self.glTextureStorage3DMultisample)(
            texture,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }
    pub unsafe fn TextureSubImage1D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureSubImage1D()");
        (self.glTextureSubImage1D)(texture, level, xoffset, width, format, r#type, pixels)
    }
    pub unsafe fn TextureSubImage2D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureSubImage2D()");
        (self.glTextureSubImage2D)(
            texture, level, xoffset, yoffset, width, height, format, r#type, pixels,
        )
    }
    pub unsafe fn TextureSubImage3D(
        &self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureSubImage3D()");
        (self.glTextureSubImage3D)(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, pixels,
        )
    }
    pub unsafe fn TextureView(
        &self,
        texture: GLuint,
        target: GLenum,
        origtexture: GLuint,
        internalformat: GLenum,
        minlevel: GLuint,
        numlevels: GLuint,
        minlayer: GLuint,
        numlayers: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TextureView()");
        (self.glTextureView)(
            texture,
            target,
            origtexture,
            internalformat,
            minlevel,
            numlevels,
            minlayer,
            numlayers,
        )
    }
    pub unsafe fn TransformFeedbackBufferBase(&self, xfb: GLuint, index: GLuint, buffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TransformFeedbackBufferBase()");
        (self.glTransformFeedbackBufferBase)(xfb, index, buffer)
    }
    pub unsafe fn TransformFeedbackBufferRange(
        &self,
        xfb: GLuint,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TransformFeedbackBufferRange()");
        (self.glTransformFeedbackBufferRange)(xfb, index, buffer, offset, size)
    }
    pub unsafe fn TransformFeedbackVaryings(
        &self,
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling TransformFeedbackVaryings()");
        (self.glTransformFeedbackVaryings)(program, count, varyings, bufferMode)
    }
    pub unsafe fn Uniform1d(&self, location: GLint, x: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1d()");
        (self.glUniform1d)(location, x)
    }
    pub unsafe fn Uniform1dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1dv()");
        (self.glUniform1dv)(location, count, value)
    }
    pub unsafe fn Uniform1f(&self, location: GLint, v0: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1f()");
        (self.glUniform1f)(location, v0)
    }
    pub unsafe fn Uniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1fv()");
        (self.glUniform1fv)(location, count, value)
    }
    pub unsafe fn Uniform1i(&self, location: GLint, v0: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1i()");
        (self.glUniform1i)(location, v0)
    }
    pub unsafe fn Uniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1iv()");
        (self.glUniform1iv)(location, count, value)
    }
    pub unsafe fn Uniform1ui(&self, location: GLint, v0: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1ui()");
        (self.glUniform1ui)(location, v0)
    }
    pub unsafe fn Uniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform1uiv()");
        (self.glUniform1uiv)(location, count, value)
    }
    pub unsafe fn Uniform2d(&self, location: GLint, x: GLdouble, y: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2d()");
        (self.glUniform2d)(location, x, y)
    }
    pub unsafe fn Uniform2dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2dv()");
        (self.glUniform2dv)(location, count, value)
    }
    pub unsafe fn Uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2f()");
        (self.glUniform2f)(location, v0, v1)
    }
    pub unsafe fn Uniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2fv()");
        (self.glUniform2fv)(location, count, value)
    }
    pub unsafe fn Uniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2i()");
        (self.glUniform2i)(location, v0, v1)
    }
    pub unsafe fn Uniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2iv()");
        (self.glUniform2iv)(location, count, value)
    }
    pub unsafe fn Uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2ui()");
        (self.glUniform2ui)(location, v0, v1)
    }
    pub unsafe fn Uniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform2uiv()");
        (self.glUniform2uiv)(location, count, value)
    }
    pub unsafe fn Uniform3d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3d()");
        (self.glUniform3d)(location, x, y, z)
    }
    pub unsafe fn Uniform3dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3dv()");
        (self.glUniform3dv)(location, count, value)
    }
    pub unsafe fn Uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3f()");
        (self.glUniform3f)(location, v0, v1, v2)
    }
    pub unsafe fn Uniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3fv()");
        (self.glUniform3fv)(location, count, value)
    }
    pub unsafe fn Uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3i()");
        (self.glUniform3i)(location, v0, v1, v2)
    }
    pub unsafe fn Uniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3iv()");
        (self.glUniform3iv)(location, count, value)
    }
    pub unsafe fn Uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3ui()");
        (self.glUniform3ui)(location, v0, v1, v2)
    }
    pub unsafe fn Uniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform3uiv()");
        (self.glUniform3uiv)(location, count, value)
    }
    pub unsafe fn Uniform4d(
        &self,
        location: GLint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4d()");
        (self.glUniform4d)(location, x, y, z, w)
    }
    pub unsafe fn Uniform4dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4dv()");
        (self.glUniform4dv)(location, count, value)
    }
    pub unsafe fn Uniform4f(
        &self,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4f()");
        (self.glUniform4f)(location, v0, v1, v2, v3)
    }
    pub unsafe fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4fv()");
        (self.glUniform4fv)(location, count, value)
    }
    pub unsafe fn Uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4i()");
        (self.glUniform4i)(location, v0, v1, v2, v3)
    }
    pub unsafe fn Uniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4iv()");
        (self.glUniform4iv)(location, count, value)
    }
    pub unsafe fn Uniform4ui(
        &self,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4ui()");
        (self.glUniform4ui)(location, v0, v1, v2, v3)
    }
    pub unsafe fn Uniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Uniform4uiv()");
        (self.glUniform4uiv)(location, count, value)
    }
    pub unsafe fn UniformBlockBinding(
        &self,
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformBlockBinding()");
        (self.glUniformBlockBinding)(program, uniformBlockIndex, uniformBlockBinding)
    }
    pub unsafe fn UniformMatrix2dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix2dv()");
        (self.glUniformMatrix2dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix2fv()");
        (self.glUniformMatrix2fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix2x3dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix2x3dv()");
        (self.glUniformMatrix2x3dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix2x3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix2x3fv()");
        (self.glUniformMatrix2x3fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix2x4dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix2x4dv()");
        (self.glUniformMatrix2x4dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix2x4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix2x4fv()");
        (self.glUniformMatrix2x4fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix3dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix3dv()");
        (self.glUniformMatrix3dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix3fv()");
        (self.glUniformMatrix3fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix3x2dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix3x2dv()");
        (self.glUniformMatrix3x2dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix3x2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix3x2fv()");
        (self.glUniformMatrix3x2fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix3x4dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix3x4dv()");
        (self.glUniformMatrix3x4dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix3x4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix3x4fv()");
        (self.glUniformMatrix3x4fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix4dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix4dv()");
        (self.glUniformMatrix4dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix4fv()");
        (self.glUniformMatrix4fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix4x2dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix4x2dv()");
        (self.glUniformMatrix4x2dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix4x2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix4x2fv()");
        (self.glUniformMatrix4x2fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix4x3dv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix4x3dv()");
        (self.glUniformMatrix4x3dv)(location, count, transpose, value)
    }
    pub unsafe fn UniformMatrix4x3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformMatrix4x3fv()");
        (self.glUniformMatrix4x3fv)(location, count, transpose, value)
    }
    pub unsafe fn UniformSubroutinesuiv(
        &self,
        shadertype: GLenum,
        count: GLsizei,
        indices: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UniformSubroutinesuiv()");
        (self.glUniformSubroutinesuiv)(shadertype, count, indices)
    }
    pub unsafe fn UnmapBuffer(&self, target: GLenum) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UnmapBuffer()");
        (self.glUnmapBuffer)(target)
    }
    pub unsafe fn UnmapNamedBuffer(&self, buffer: GLuint) -> GLboolean {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UnmapNamedBuffer()");
        (self.glUnmapNamedBuffer)(buffer)
    }
    pub unsafe fn UseProgram(&self, program: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UseProgram()");
        (self.glUseProgram)(program)
    }
    pub unsafe fn UseProgramStages(&self, pipeline: GLuint, stages: GLbitfield, program: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling UseProgramStages()");
        (self.glUseProgramStages)(pipeline, stages, program)
    }
    pub unsafe fn ValidateProgram(&self, program: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ValidateProgram()");
        (self.glValidateProgram)(program)
    }
    pub unsafe fn ValidateProgramPipeline(&self, pipeline: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ValidateProgramPipeline()");
        (self.glValidateProgramPipeline)(pipeline)
    }
    pub unsafe fn VertexArrayAttribBinding(
        &self,
        vaobj: GLuint,
        attribindex: GLuint,
        bindingindex: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayAttribBinding()");
        (self.glVertexArrayAttribBinding)(vaobj, attribindex, bindingindex)
    }
    pub unsafe fn VertexArrayAttribFormat(
        &self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayAttribFormat()");
        (self.glVertexArrayAttribFormat)(
            vaobj,
            attribindex,
            size,
            r#type,
            normalized,
            relativeoffset,
        )
    }
    pub unsafe fn VertexArrayAttribIFormat(
        &self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayAttribIFormat()");
        (self.glVertexArrayAttribIFormat)(vaobj, attribindex, size, r#type, relativeoffset)
    }
    pub unsafe fn VertexArrayAttribLFormat(
        &self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayAttribLFormat()");
        (self.glVertexArrayAttribLFormat)(vaobj, attribindex, size, r#type, relativeoffset)
    }
    pub unsafe fn VertexArrayBindingDivisor(
        &self,
        vaobj: GLuint,
        bindingindex: GLuint,
        divisor: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayBindingDivisor()");
        (self.glVertexArrayBindingDivisor)(vaobj, bindingindex, divisor)
    }
    pub unsafe fn VertexArrayElementBuffer(&self, vaobj: GLuint, buffer: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayElementBuffer()");
        (self.glVertexArrayElementBuffer)(vaobj, buffer)
    }
    pub unsafe fn VertexArrayVertexBuffer(
        &self,
        vaobj: GLuint,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayVertexBuffer()");
        (self.glVertexArrayVertexBuffer)(vaobj, bindingindex, buffer, offset, stride)
    }
    pub unsafe fn VertexArrayVertexBuffers(
        &self,
        vaobj: GLuint,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexArrayVertexBuffers()");
        (self.glVertexArrayVertexBuffers)(vaobj, first, count, buffers, offsets, strides)
    }
    pub unsafe fn VertexAttrib1d(&self, index: GLuint, x: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib1d()");
        (self.glVertexAttrib1d)(index, x)
    }
    pub unsafe fn VertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib1dv()");
        (self.glVertexAttrib1dv)(index, v)
    }
    pub unsafe fn VertexAttrib1f(&self, index: GLuint, x: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib1f()");
        (self.glVertexAttrib1f)(index, x)
    }
    pub unsafe fn VertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib1fv()");
        (self.glVertexAttrib1fv)(index, v)
    }
    pub unsafe fn VertexAttrib1s(&self, index: GLuint, x: GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib1s()");
        (self.glVertexAttrib1s)(index, x)
    }
    pub unsafe fn VertexAttrib1sv(&self, index: GLuint, v: *const GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib1sv()");
        (self.glVertexAttrib1sv)(index, v)
    }
    pub unsafe fn VertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib2d()");
        (self.glVertexAttrib2d)(index, x, y)
    }
    pub unsafe fn VertexAttrib2dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib2dv()");
        (self.glVertexAttrib2dv)(index, v)
    }
    pub unsafe fn VertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib2f()");
        (self.glVertexAttrib2f)(index, x, y)
    }
    pub unsafe fn VertexAttrib2fv(&self, index: GLuint, v: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib2fv()");
        (self.glVertexAttrib2fv)(index, v)
    }
    pub unsafe fn VertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib2s()");
        (self.glVertexAttrib2s)(index, x, y)
    }
    pub unsafe fn VertexAttrib2sv(&self, index: GLuint, v: *const GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib2sv()");
        (self.glVertexAttrib2sv)(index, v)
    }
    pub unsafe fn VertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib3d()");
        (self.glVertexAttrib3d)(index, x, y, z)
    }
    pub unsafe fn VertexAttrib3dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib3dv()");
        (self.glVertexAttrib3dv)(index, v)
    }
    pub unsafe fn VertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib3f()");
        (self.glVertexAttrib3f)(index, x, y, z)
    }
    pub unsafe fn VertexAttrib3fv(&self, index: GLuint, v: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib3fv()");
        (self.glVertexAttrib3fv)(index, v)
    }
    pub unsafe fn VertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib3s()");
        (self.glVertexAttrib3s)(index, x, y, z)
    }
    pub unsafe fn VertexAttrib3sv(&self, index: GLuint, v: *const GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib3sv()");
        (self.glVertexAttrib3sv)(index, v)
    }
    pub unsafe fn VertexAttrib4Nbv(&self, index: GLuint, v: *const GLbyte) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4Nbv()");
        (self.glVertexAttrib4Nbv)(index, v)
    }
    pub unsafe fn VertexAttrib4Niv(&self, index: GLuint, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4Niv()");
        (self.glVertexAttrib4Niv)(index, v)
    }
    pub unsafe fn VertexAttrib4Nsv(&self, index: GLuint, v: *const GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4Nsv()");
        (self.glVertexAttrib4Nsv)(index, v)
    }
    pub unsafe fn VertexAttrib4Nub(
        &self,
        index: GLuint,
        x: GLubyte,
        y: GLubyte,
        z: GLubyte,
        w: GLubyte,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4Nub()");
        (self.glVertexAttrib4Nub)(index, x, y, z, w)
    }
    pub unsafe fn VertexAttrib4Nubv(&self, index: GLuint, v: *const GLubyte) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4Nubv()");
        (self.glVertexAttrib4Nubv)(index, v)
    }
    pub unsafe fn VertexAttrib4Nuiv(&self, index: GLuint, v: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4Nuiv()");
        (self.glVertexAttrib4Nuiv)(index, v)
    }
    pub unsafe fn VertexAttrib4Nusv(&self, index: GLuint, v: *const GLushort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4Nusv()");
        (self.glVertexAttrib4Nusv)(index, v)
    }
    pub unsafe fn VertexAttrib4bv(&self, index: GLuint, v: *const GLbyte) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4bv()");
        (self.glVertexAttrib4bv)(index, v)
    }
    pub unsafe fn VertexAttrib4d(
        &self,
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4d()");
        (self.glVertexAttrib4d)(index, x, y, z, w)
    }
    pub unsafe fn VertexAttrib4dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4dv()");
        (self.glVertexAttrib4dv)(index, v)
    }
    pub unsafe fn VertexAttrib4f(
        &self,
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4f()");
        (self.glVertexAttrib4f)(index, x, y, z, w)
    }
    pub unsafe fn VertexAttrib4fv(&self, index: GLuint, v: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4fv()");
        (self.glVertexAttrib4fv)(index, v)
    }
    pub unsafe fn VertexAttrib4iv(&self, index: GLuint, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4iv()");
        (self.glVertexAttrib4iv)(index, v)
    }
    pub unsafe fn VertexAttrib4s(
        &self,
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort,
        w: GLshort,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4s()");
        (self.glVertexAttrib4s)(index, x, y, z, w)
    }
    pub unsafe fn VertexAttrib4sv(&self, index: GLuint, v: *const GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4sv()");
        (self.glVertexAttrib4sv)(index, v)
    }
    pub unsafe fn VertexAttrib4ubv(&self, index: GLuint, v: *const GLubyte) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4ubv()");
        (self.glVertexAttrib4ubv)(index, v)
    }
    pub unsafe fn VertexAttrib4uiv(&self, index: GLuint, v: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4uiv()");
        (self.glVertexAttrib4uiv)(index, v)
    }
    pub unsafe fn VertexAttrib4usv(&self, index: GLuint, v: *const GLushort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttrib4usv()");
        (self.glVertexAttrib4usv)(index, v)
    }
    pub unsafe fn VertexAttribBinding(&self, attribindex: GLuint, bindingindex: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribBinding()");
        (self.glVertexAttribBinding)(attribindex, bindingindex)
    }
    pub unsafe fn VertexAttribDivisor(&self, index: GLuint, divisor: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribDivisor()");
        (self.glVertexAttribDivisor)(index, divisor)
    }
    pub unsafe fn VertexAttribFormat(
        &self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribFormat()");
        (self.glVertexAttribFormat)(attribindex, size, r#type, normalized, relativeoffset)
    }
    pub unsafe fn VertexAttribI1i(&self, index: GLuint, x: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI1i()");
        (self.glVertexAttribI1i)(index, x)
    }
    pub unsafe fn VertexAttribI1iv(&self, index: GLuint, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI1iv()");
        (self.glVertexAttribI1iv)(index, v)
    }
    pub unsafe fn VertexAttribI1ui(&self, index: GLuint, x: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI1ui()");
        (self.glVertexAttribI1ui)(index, x)
    }
    pub unsafe fn VertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI1uiv()");
        (self.glVertexAttribI1uiv)(index, v)
    }
    pub unsafe fn VertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI2i()");
        (self.glVertexAttribI2i)(index, x, y)
    }
    pub unsafe fn VertexAttribI2iv(&self, index: GLuint, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI2iv()");
        (self.glVertexAttribI2iv)(index, v)
    }
    pub unsafe fn VertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI2ui()");
        (self.glVertexAttribI2ui)(index, x, y)
    }
    pub unsafe fn VertexAttribI2uiv(&self, index: GLuint, v: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI2uiv()");
        (self.glVertexAttribI2uiv)(index, v)
    }
    pub unsafe fn VertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI3i()");
        (self.glVertexAttribI3i)(index, x, y, z)
    }
    pub unsafe fn VertexAttribI3iv(&self, index: GLuint, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI3iv()");
        (self.glVertexAttribI3iv)(index, v)
    }
    pub unsafe fn VertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI3ui()");
        (self.glVertexAttribI3ui)(index, x, y, z)
    }
    pub unsafe fn VertexAttribI3uiv(&self, index: GLuint, v: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI3uiv()");
        (self.glVertexAttribI3uiv)(index, v)
    }
    pub unsafe fn VertexAttribI4bv(&self, index: GLuint, v: *const GLbyte) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4bv()");
        (self.glVertexAttribI4bv)(index, v)
    }
    pub unsafe fn VertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4i()");
        (self.glVertexAttribI4i)(index, x, y, z, w)
    }
    pub unsafe fn VertexAttribI4iv(&self, index: GLuint, v: *const GLint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4iv()");
        (self.glVertexAttribI4iv)(index, v)
    }
    pub unsafe fn VertexAttribI4sv(&self, index: GLuint, v: *const GLshort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4sv()");
        (self.glVertexAttribI4sv)(index, v)
    }
    pub unsafe fn VertexAttribI4ubv(&self, index: GLuint, v: *const GLubyte) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4ubv()");
        (self.glVertexAttribI4ubv)(index, v)
    }
    pub unsafe fn VertexAttribI4ui(
        &self,
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4ui()");
        (self.glVertexAttribI4ui)(index, x, y, z, w)
    }
    pub unsafe fn VertexAttribI4uiv(&self, index: GLuint, v: *const GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4uiv()");
        (self.glVertexAttribI4uiv)(index, v)
    }
    pub unsafe fn VertexAttribI4usv(&self, index: GLuint, v: *const GLushort) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribI4usv()");
        (self.glVertexAttribI4usv)(index, v)
    }
    pub unsafe fn VertexAttribIFormat(
        &self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribIFormat()");
        (self.glVertexAttribIFormat)(attribindex, size, r#type, relativeoffset)
    }
    pub unsafe fn VertexAttribIPointer(
        &self,
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        stride: GLsizei,
        pointer: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribIPointer()");
        (self.glVertexAttribIPointer)(index, size, r#type, stride, pointer)
    }
    pub unsafe fn VertexAttribL1d(&self, index: GLuint, x: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL1d()");
        (self.glVertexAttribL1d)(index, x)
    }
    pub unsafe fn VertexAttribL1dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL1dv()");
        (self.glVertexAttribL1dv)(index, v)
    }
    pub unsafe fn VertexAttribL2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL2d()");
        (self.glVertexAttribL2d)(index, x, y)
    }
    pub unsafe fn VertexAttribL2dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL2dv()");
        (self.glVertexAttribL2dv)(index, v)
    }
    pub unsafe fn VertexAttribL3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL3d()");
        (self.glVertexAttribL3d)(index, x, y, z)
    }
    pub unsafe fn VertexAttribL3dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL3dv()");
        (self.glVertexAttribL3dv)(index, v)
    }
    pub unsafe fn VertexAttribL4d(
        &self,
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL4d()");
        (self.glVertexAttribL4d)(index, x, y, z, w)
    }
    pub unsafe fn VertexAttribL4dv(&self, index: GLuint, v: *const GLdouble) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribL4dv()");
        (self.glVertexAttribL4dv)(index, v)
    }
    pub unsafe fn VertexAttribLFormat(
        &self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribLFormat()");
        (self.glVertexAttribLFormat)(attribindex, size, r#type, relativeoffset)
    }
    pub unsafe fn VertexAttribLPointer(
        &self,
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        stride: GLsizei,
        pointer: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribLPointer()");
        (self.glVertexAttribLPointer)(index, size, r#type, stride, pointer)
    }
    pub unsafe fn VertexAttribP1ui(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP1ui()");
        (self.glVertexAttribP1ui)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribP1uiv(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP1uiv()");
        (self.glVertexAttribP1uiv)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribP2ui(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP2ui()");
        (self.glVertexAttribP2ui)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribP2uiv(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP2uiv()");
        (self.glVertexAttribP2uiv)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribP3ui(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP3ui()");
        (self.glVertexAttribP3ui)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribP3uiv(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP3uiv()");
        (self.glVertexAttribP3uiv)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribP4ui(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP4ui()");
        (self.glVertexAttribP4ui)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribP4uiv(
        &self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribP4uiv()");
        (self.glVertexAttribP4uiv)(index, r#type, normalized, value)
    }
    pub unsafe fn VertexAttribPointer(
        &self,
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexAttribPointer()");
        (self.glVertexAttribPointer)(index, size, r#type, normalized, stride, pointer)
    }
    pub unsafe fn VertexBindingDivisor(&self, bindingindex: GLuint, divisor: GLuint) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling VertexBindingDivisor()");
        (self.glVertexBindingDivisor)(bindingindex, divisor)
    }
    pub unsafe fn Viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling Viewport()");
        (self.glViewport)(x, y, width, height)
    }
    pub unsafe fn ViewportArrayv(&self, first: GLuint, count: GLsizei, v: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ViewportArrayv()");
        (self.glViewportArrayv)(first, count, v)
    }
    pub unsafe fn ViewportIndexedf(
        &self,
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        w: GLfloat,
        h: GLfloat,
    ) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ViewportIndexedf()");
        (self.glViewportIndexedf)(index, x, y, w, h)
    }
    pub unsafe fn ViewportIndexedfv(&self, index: GLuint, v: *const GLfloat) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling ViewportIndexedfv()");
        (self.glViewportIndexedfv)(index, v)
    }
    pub unsafe fn WaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
        #[cfg(all(debug_assertions, feature = "tracing", feature = "trace-calls"))]
        trace!("Calling WaitSync()");
        (self.glWaitSync)(sync, flags, timeout)
    }
}

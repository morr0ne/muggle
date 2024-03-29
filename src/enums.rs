use muggle_macros::gl_enum;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::ops::{Add, BitOr, Sub};

#[doc(hidden)]
pub use angel::enums::*;

gl_enum! { ShaderType "SHADER" GL_COMPUTE_SHADER GL_VERTEX_SHADER GL_TESS_CONTROL_SHADER GL_TESS_EVALUATION_SHADER GL_GEOMETRY_SHADER GL_FRAGMENT_SHADER }
gl_enum! { ShaderParameter GL_SHADER_TYPE GL_DELETE_STATUS GL_COMPILE_STATUS GL_INFO_LOG_LENGTH GL_SHADER_SOURCE_LENGTH }
gl_enum! {
    ProgramParameter GL_DELETE_STATUS GL_LINK_STATUS GL_VALIDATE_STATUS GL_INFO_LOG_LENGTH GL_ATTACHED_SHADERS GL_ACTIVE_ATOMIC_COUNTER_BUFFERS
    GL_ACTIVE_ATTRIBUTES GL_ACTIVE_ATTRIBUTE_MAX_LENGTH GL_ACTIVE_UNIFORMS GL_ACTIVE_UNIFORM_MAX_LENGTH GL_PROGRAM_BINARY_LENGTH GL_COMPUTE_WORK_GROUP_SIZE
    GL_TRANSFORM_FEEDBACK_BUFFER_MODE GL_TRANSFORM_FEEDBACK_VARYINGS GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH GL_GEOMETRY_VERTICES_OUT GL_GEOMETRY_INPUT_TYPE
    GL_GEOMETRY_OUTPUT_TYPE
}
gl_enum! {
    BufferTarget "BUFFER" GL_ARRAY_BUFFER GL_ATOMIC_COUNTER_BUFFER GL_COPY_READ_BUFFER GL_COPY_WRITE_BUFFER GL_DISPATCH_INDIRECT_BUFFER
    GL_DRAW_INDIRECT_BUFFER GL_ELEMENT_ARRAY_BUFFER GL_PIXEL_PACK_BUFFER GL_QUERY_BUFFER GL_SHADER_STORAGE_BUFFER GL_TEXTURE_BUFFER
    GL_TRANSFORM_FEEDBACK_BUFFER GL_UNIFORM_BUFFER
}
gl_enum! {
    DrawMode GL_POINTS GL_LINE_STRIP GL_LINE_LOOP GL_LINES GL_LINE_STRIP_ADJACENCY GL_LINES_ADJACENCY GL_TRIANGLE_STRIP GL_TRIANGLE_FAN
    GL_TRIANGLES GL_TRIANGLE_STRIP_ADJACENCY GL_TRIANGLES_ADJACENCY GL_PATCHES,
}
gl_enum! { BufferUsage GL_STREAM_READ GL_STREAM_DRAW GL_STREAM_COPY GL_STATIC_DRAW GL_STATIC_READ GL_STATIC_COPY GL_DYNAMIC_DRAW GL_DYNAMIC_READ GL_DYNAMIC_COPY }
gl_enum! {
    Parameter GL_ACTIVE_TEXTURE GL_ALIASED_LINE_WIDTH_RANGE GL_ARRAY_BUFFER_BINDING GL_BLEND GL_BLEND_COLOR GL_BLEND_DST_ALPHA GL_BLEND_DST_RGB
    GL_BLEND_EQUATION_RGB GL_BLEND_EQUATION_ALPHA GL_BLEND_SRC_ALPHA GL_BLEND_SRC_RGB GL_COLOR_CLEAR_VALUE GL_COLOR_LOGIC_OP GL_COLOR_WRITEMASK
    GL_COMPRESSED_TEXTURE_FORMATS GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS GL_MAX_COMPUTE_UNIFORM_BLOCKS
    GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS GL_MAX_COMPUTE_UNIFORM_COMPONENTS GL_MAX_COMPUTE_ATOMIC_COUNTERS GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS
    GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS GL_MAX_COMPUTE_WORK_GROUP_COUNT GL_MAX_COMPUTE_WORK_GROUP_SIZE
    GL_DISPATCH_INDIRECT_BUFFER_BINDING GL_MAX_DEBUG_GROUP_STACK_DEPTH GL_DEBUG_GROUP_STACK_DEPTH GL_CONTEXT_FLAGS GL_CULL_FACE GL_CULL_FACE_MODE
    GL_CURRENT_PROGRAM GL_DEPTH_CLEAR_VALUE GL_DEPTH_FUNC GL_DEPTH_RANGE GL_DEPTH_TEST GL_DEPTH_WRITEMASK GL_DITHER GL_DOUBLEBUFFER GL_DRAW_BUFFER
    GL_DRAW_BUFFER0 GL_DRAW_BUFFER1 GL_DRAW_BUFFER2 GL_DRAW_BUFFER3 GL_DRAW_BUFFER4 GL_DRAW_BUFFER5 GL_DRAW_BUFFER6 GL_DRAW_BUFFER7 GL_DRAW_BUFFER8
    GL_DRAW_BUFFER9 GL_DRAW_BUFFER10 GL_DRAW_BUFFER11 GL_DRAW_BUFFER12 GL_DRAW_BUFFER13 GL_DRAW_BUFFER14 GL_DRAW_BUFFER15 GL_DRAW_FRAMEBUFFER_BINDING
    GL_FRAGMENT_SHADER_DERIVATIVE_HINT GL_IMPLEMENTATION_COLOR_READ_FORMAT GL_IMPLEMENTATION_COLOR_READ_TYPE GL_LINE_SMOOTH GL_LINE_SMOOTH_HINT GL_LINE_WIDTH
    GL_LAYER_PROVOKING_VERTEX GL_LOGIC_OP_MODE GL_MAJOR_VERSION GL_MAX_3D_TEXTURE_SIZE GL_MAX_ARRAY_TEXTURE_LAYERS GL_MAX_CLIP_DISTANCES
    GL_MAX_COLOR_TEXTURE_SAMPLES GL_MAX_COMBINED_ATOMIC_COUNTERS GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS
    GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS GL_MAX_COMBINED_UNIFORM_BLOCKS GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS GL_MAX_CUBE_MAP_TEXTURE_SIZE
    GL_MAX_DEPTH_TEXTURE_SAMPLES GL_MAX_DRAW_BUFFERS GL_MAX_DUAL_SOURCE_DRAW_BUFFERS GL_MAX_ELEMENTS_INDICES GL_MAX_ELEMENTS_VERTICES
    GL_MAX_FRAGMENT_ATOMIC_COUNTERS GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS GL_MAX_FRAGMENT_INPUT_COMPONENTS GL_MAX_FRAGMENT_UNIFORM_COMPONENTS
    GL_MAX_FRAGMENT_UNIFORM_VECTORS GL_MAX_FRAGMENT_UNIFORM_BLOCKS GL_MAX_FRAMEBUFFER_WIDTH GL_MAX_FRAMEBUFFER_HEIGHT GL_MAX_FRAMEBUFFER_LAYERS
    GL_MAX_FRAMEBUFFER_SAMPLES GL_MAX_GEOMETRY_ATOMIC_COUNTERS GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS GL_MAX_GEOMETRY_INPUT_COMPONENTS
    GL_MAX_GEOMETRY_OUTPUT_COMPONENTS GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS GL_MAX_GEOMETRY_UNIFORM_BLOCKS GL_MAX_GEOMETRY_UNIFORM_COMPONENTS
    GL_MAX_INTEGER_SAMPLES GL_MIN_MAP_BUFFER_ALIGNMENT GL_MAX_LABEL_LENGTH GL_MAX_PROGRAM_TEXEL_OFFSET GL_MIN_PROGRAM_TEXEL_OFFSET
    GL_MAX_RECTANGLE_TEXTURE_SIZE GL_MAX_RENDERBUFFER_SIZE GL_MAX_SAMPLE_MASK_WORDS GL_MAX_SERVER_WAIT_TIMEOUT GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS
    GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS
    GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS GL_MAX_TEXTURE_BUFFER_SIZE GL_MAX_TEXTURE_IMAGE_UNITS GL_MAX_TEXTURE_LOD_BIAS GL_MAX_TEXTURE_SIZE
    GL_MAX_UNIFORM_BUFFER_BINDINGS GL_MAX_UNIFORM_BLOCK_SIZE GL_MAX_UNIFORM_LOCATIONS GL_MAX_VARYING_COMPONENTS GL_MAX_VARYING_VECTORS
    GL_MAX_VERTEX_ATOMIC_COUNTERS GL_MAX_VERTEX_ATTRIBS GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS GL_MAX_VERTEX_UNIFORM_COMPONENTS
    GL_MAX_VERTEX_UNIFORM_VECTORS GL_MAX_VERTEX_OUTPUT_COMPONENTS GL_MAX_VERTEX_UNIFORM_BLOCKS GL_MAX_VIEWPORT_DIMS GL_MAX_VIEWPORTS GL_MINOR_VERSION
    GL_NUM_COMPRESSED_TEXTURE_FORMATS GL_NUM_EXTENSIONS GL_NUM_PROGRAM_BINARY_FORMATS GL_NUM_SHADER_BINARY_FORMATS GL_PACK_ALIGNMENT GL_PACK_IMAGE_HEIGHT
    GL_PACK_LSB_FIRST GL_PACK_ROW_LENGTH GL_PACK_SKIP_IMAGES GL_PACK_SKIP_PIXELS GL_PACK_SKIP_ROWS GL_PACK_SWAP_BYTES GL_PIXEL_PACK_BUFFER_BINDING
    GL_PIXEL_UNPACK_BUFFER_BINDING GL_POINT_FADE_THRESHOLD_SIZE GL_PRIMITIVE_RESTART_INDEX GL_PROGRAM_BINARY_FORMATS GL_PROGRAM_PIPELINE_BINDING
    GL_PROGRAM_POINT_SIZE GL_PROVOKING_VERTEX GL_POINT_SIZE GL_POINT_SIZE_GRANULARITY GL_POINT_SIZE_RANGE GL_POLYGON_OFFSET_FACTOR GL_POLYGON_OFFSET_UNITS
    GL_POLYGON_OFFSET_FILL GL_POLYGON_OFFSET_LINE GL_POLYGON_OFFSET_POINT GL_POLYGON_SMOOTH GL_POLYGON_SMOOTH_HINT GL_READ_BUFFER GL_RENDERBUFFER_BINDING
    GL_SAMPLE_BUFFERS GL_SAMPLE_COVERAGE_VALUE GL_SAMPLE_COVERAGE_INVERT GL_SAMPLER_BINDING GL_SAMPLES GL_SCISSOR_BOX GL_SCISSOR_TEST GL_SHADER_COMPILER
    GL_SHADER_STORAGE_BUFFER_BINDING GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT GL_SHADER_STORAGE_BUFFER_START GL_SHADER_STORAGE_BUFFER_SIZE
    GL_SMOOTH_LINE_WIDTH_RANGE GL_SMOOTH_LINE_WIDTH_GRANULARITY GL_STENCIL_BACK_FAIL GL_STENCIL_BACK_FUNC GL_STENCIL_BACK_PASS_DEPTH_FAIL
    GL_STENCIL_BACK_PASS_DEPTH_PASS GL_STENCIL_BACK_REF GL_STENCIL_BACK_VALUE_MASK GL_STENCIL_BACK_WRITEMASK GL_STENCIL_CLEAR_VALUE GL_STENCIL_FAIL
    GL_STENCIL_FUNC GL_STENCIL_PASS_DEPTH_FAIL GL_STENCIL_PASS_DEPTH_PASS GL_STENCIL_REF GL_STENCIL_TEST GL_STENCIL_VALUE_MASK GL_STENCIL_WRITEMASK GL_STEREO
    GL_SUBPIXEL_BITS GL_TEXTURE_BINDING_1D GL_TEXTURE_BINDING_1D_ARRAY GL_TEXTURE_BINDING_2D GL_TEXTURE_BINDING_2D_ARRAY GL_TEXTURE_BINDING_2D_MULTISAMPLE
    GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY GL_TEXTURE_BINDING_3D GL_TEXTURE_BINDING_BUFFER GL_TEXTURE_BINDING_CUBE_MAP GL_TEXTURE_BINDING_RECTANGLE
    GL_TEXTURE_COMPRESSION_HINT GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT GL_TIMESTAMP GL_TRANSFORM_FEEDBACK_BUFFER_BINDING GL_TRANSFORM_FEEDBACK_BUFFER_START
    GL_TRANSFORM_FEEDBACK_BUFFER_SIZE GL_UNIFORM_BUFFER_BINDING GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT GL_UNIFORM_BUFFER_SIZE GL_UNIFORM_BUFFER_START
    GL_UNPACK_ALIGNMENT GL_UNPACK_IMAGE_HEIGHT GL_UNPACK_LSB_FIRST GL_UNPACK_ROW_LENGTH GL_UNPACK_SKIP_IMAGES GL_UNPACK_SKIP_PIXELS GL_UNPACK_SKIP_ROWS
    GL_UNPACK_SWAP_BYTES GL_VERTEX_ARRAY_BINDING GL_VERTEX_BINDING_DIVISOR GL_VERTEX_BINDING_OFFSET GL_VERTEX_BINDING_STRIDE GL_VERTEX_BINDING_BUFFER
    GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET GL_MAX_VERTEX_ATTRIB_BINDINGS GL_VIEWPORT GL_VIEWPORT_BOUNDS_RANGE GL_VIEWPORT_INDEX_PROVOKING_VERTEX
    GL_VIEWPORT_SUBPIXEL_BITS GL_MAX_ELEMENT_INDEX
}

// TODO: Add alias functionality to macro
impl Parameter {
    #[allow(non_upper_case_globals)]
    pub const MaxVaryingFloats: Self = Self::MaxVaryingVectors;
}

gl_enum! { StringParameter GL_VENDOR GL_RENDERER GL_VERSION GL_SHADING_LANGUAGE_VERSION }

// TODO: Allow custom attributes for macro
#[derive(
    Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive, FromPrimitive,
)]
#[repr(u32)]
#[non_exhaustive]
pub enum Error {
    NoError = GL_NO_ERROR,
    InvalidEnum = GL_INVALID_ENUM,
    InvalidValue = GL_INVALID_VALUE,
    InvalidOperation = GL_INVALID_OPERATION,
    InvalidFramebufferOperation = GL_INVALID_FRAMEBUFFER_OPERATION,
    OutOfMemory = GL_OUT_OF_MEMORY,
    StackUnderflow = GL_STACK_UNDERFLOW,
    StackOverflow = GL_STACK_OVERFLOW,
    ContextLost = GL_CONTEXT_LOST,
    #[num_enum(default)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Texture(u32);

impl Texture {
    pub const TEXTURE0: Self = Self(GL_TEXTURE0);
    pub const TEXTURE1: Self = Self(GL_TEXTURE1);
    pub const TEXTURE2: Self = Self(GL_TEXTURE2);
    pub const TEXTURE3: Self = Self(GL_TEXTURE3);
    pub const TEXTURE4: Self = Self(GL_TEXTURE4);
    pub const TEXTURE5: Self = Self(GL_TEXTURE5);
    pub const TEXTURE6: Self = Self(GL_TEXTURE6);
    pub const TEXTURE7: Self = Self(GL_TEXTURE7);
    pub const TEXTURE8: Self = Self(GL_TEXTURE8);
    pub const TEXTURE9: Self = Self(GL_TEXTURE9);
    pub const TEXTURE10: Self = Self(GL_TEXTURE10);
    pub const TEXTURE11: Self = Self(GL_TEXTURE11);
    pub const TEXTURE12: Self = Self(GL_TEXTURE12);
    pub const TEXTURE13: Self = Self(GL_TEXTURE13);
    pub const TEXTURE14: Self = Self(GL_TEXTURE14);
    pub const TEXTURE15: Self = Self(GL_TEXTURE15);
    pub const TEXTURE16: Self = Self(GL_TEXTURE16);
    pub const TEXTURE17: Self = Self(GL_TEXTURE17);
    pub const TEXTURE18: Self = Self(GL_TEXTURE18);
    pub const TEXTURE19: Self = Self(GL_TEXTURE19);
    pub const TEXTURE20: Self = Self(GL_TEXTURE20);
    pub const TEXTURE21: Self = Self(GL_TEXTURE21);
    pub const TEXTURE22: Self = Self(GL_TEXTURE22);
    pub const TEXTURE23: Self = Self(GL_TEXTURE23);
    pub const TEXTURE24: Self = Self(GL_TEXTURE24);
    pub const TEXTURE25: Self = Self(GL_TEXTURE25);
    pub const TEXTURE26: Self = Self(GL_TEXTURE26);
    pub const TEXTURE27: Self = Self(GL_TEXTURE27);
    pub const TEXTURE28: Self = Self(GL_TEXTURE28);
    pub const TEXTURE29: Self = Self(GL_TEXTURE29);
    pub const TEXTURE30: Self = Self(GL_TEXTURE30);
    pub const TEXTURE31: Self = Self(GL_TEXTURE31);

    pub const fn inner(&self) -> u32 {
        self.0
    }
}

impl Add for Texture {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Texture(self.0 + rhs.0)
    }
}

impl Sub for Texture {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Texture(self.0 - rhs.0)
    }
}

impl Add<u32> for Texture {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Texture(self.0 + rhs)
    }
}

impl Sub<u32> for Texture {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Texture(self.0 - rhs)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Mask(u32);

impl Mask {
    pub const COLOR_BUFFER_BIT: Self = Self(GL_COLOR_BUFFER_BIT);
    pub const DEPTH_BUFFER_BIT: Self = Self(GL_DEPTH_BUFFER_BIT);
    pub const STENCIL_BUFFER_BIT: Self = Self(GL_STENCIL_BUFFER_BIT);

    pub const fn inner(&self) -> u32 {
        self.0
    }
}

impl BitOr for Mask {
    type Output = Mask;

    fn bitor(self, rhs: Self) -> Self::Output {
        Mask(self.0 | rhs.0)
    }
}

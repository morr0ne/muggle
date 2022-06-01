use num_enum::{FromPrimitive, IntoPrimitive};
use std::ops::{Add, Sub};

#[doc(hidden)]
pub use crate::gl::enums::*;

// TODO: All this c-style enum can/should be generated files and/or with macros

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum ShaderType {
    Compute = GL_COMPUTE_SHADER,
    Vertex = GL_VERTEX_SHADER,
    TessControl = GL_TESS_CONTROL_SHADER,
    TessEvaluation = GL_TESS_EVALUATION_SHADER,
    Geometry = GL_GEOMETRY_SHADER,
    Fragment = GL_FRAGMENT_SHADER,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum ShaderParameter {
    ShaderType = GL_SHADER_TYPE,
    DeleteStatus = GL_DELETE_STATUS,
    CompileStatus = GL_COMPILE_STATUS,
    InfoLogLength = GL_INFO_LOG_LENGTH,
    ShaderSourceLength = GL_SHADER_SOURCE_LENGTH,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum ProgramParameter {
    DeleteStatus = GL_DELETE_STATUS,
    LinkStatus = GL_LINK_STATUS,
    ValidateStatus = GL_VALIDATE_STATUS,
    InfoLogLength = GL_INFO_LOG_LENGTH,
    AttachedShaders = GL_ATTACHED_SHADERS,
    ActiveAtomicCounterBuffers = GL_ACTIVE_ATOMIC_COUNTER_BUFFERS,
    ActiveAttributes = GL_ACTIVE_ATTRIBUTES,
    ActiveAttributeMaxLength = GL_ACTIVE_ATTRIBUTE_MAX_LENGTH,
    ActiveUniforms = GL_ACTIVE_UNIFORMS,
    ActiveUniformMaxLength = GL_ACTIVE_UNIFORM_MAX_LENGTH,
    ProgramBinaryLength = GL_PROGRAM_BINARY_LENGTH,
    ComputeWorkGroupSize = GL_COMPUTE_WORK_GROUP_SIZE,
    TransformFeedbackBufferMode = GL_TRANSFORM_FEEDBACK_BUFFER_MODE,
    TransformFeedbackVaryings = GL_TRANSFORM_FEEDBACK_VARYINGS,
    TransformFeedbackVaryingMaxLength = GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
    GeometryVerticesOut = GL_GEOMETRY_VERTICES_OUT,
    GeometryInputType = GL_GEOMETRY_INPUT_TYPE,
    GeometryOutputType = GL_GEOMETRY_OUTPUT_TYPE,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum BufferTarget {
    Array = GL_ARRAY_BUFFER,
    AtomicCounter = GL_ATOMIC_COUNTER_BUFFER,
    CopyRead = GL_COPY_READ_BUFFER,
    CopyWrite = GL_COPY_WRITE_BUFFER,
    DispatchIndirect = GL_DISPATCH_INDIRECT_BUFFER,
    DrawIndirect = GL_DRAW_INDIRECT_BUFFER,
    ElementArray = GL_ELEMENT_ARRAY_BUFFER,
    PixelPack = GL_PIXEL_PACK_BUFFER,
    Query = GL_QUERY_BUFFER,
    ShaderStorage = GL_SHADER_STORAGE_BUFFER,
    Texture = GL_TEXTURE_BUFFER,
    TransformFeedback = GL_TRANSFORM_FEEDBACK_BUFFER,
    Uniform = GL_UNIFORM_BUFFER,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum DrawMode {
    Points = GL_POINTS,
    LineStrip = GL_LINE_STRIP,
    LineLoop = GL_LINE_LOOP,
    Lines = GL_LINES,
    LineStripAdjacency = GL_LINE_STRIP_ADJACENCY,
    LinesAdjacency = GL_LINES_ADJACENCY,
    TriangleStrip = GL_TRIANGLE_STRIP,
    TriangleFan = GL_TRIANGLE_FAN,
    Triangles = GL_TRIANGLES,
    TriangleStripAdjacency = GL_TRIANGLE_STRIP_ADJACENCY,
    TrianglesAdjacency = GL_TRIANGLES_ADJACENCY,
    Patches = GL_PATCHES,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum BufferUsage {
    StreamDraw = GL_STREAM_READ,
    StreamRead = GL_STREAM_DRAW,
    StreamCopy = GL_STREAM_COPY,
    StaticDraw = GL_STATIC_DRAW,
    StaticRead = GL_STATIC_READ,
    StaticCopy = GL_STATIC_COPY,
    DynamicDraw = GL_DYNAMIC_DRAW,
    DynamicRead = GL_DYNAMIC_READ,
    DynamicCopy = GL_DYNAMIC_COPY,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum Parameter {
    ActiveTexture = GL_ACTIVE_TEXTURE,
    AliasedLineWidthRange = GL_ALIASED_LINE_WIDTH_RANGE,
    ArrayBufferBinding = GL_ARRAY_BUFFER_BINDING,
    BLEND = GL_BLEND,
    BlendColor = GL_BLEND_COLOR,
    BlendDstAlpha = GL_BLEND_DST_ALPHA,
    BlendDstRgb = GL_BLEND_DST_RGB,
    BlendEquationRgb = GL_BLEND_EQUATION_RGB,
    BlendEquationAlpha = GL_BLEND_EQUATION_ALPHA,
    BlendSrcAlpha = GL_BLEND_SRC_ALPHA,
    BlendSrcRgb = GL_BLEND_SRC_RGB,
    ColorClearValue = GL_COLOR_CLEAR_VALUE,
    ColorLogicOp = GL_COLOR_LOGIC_OP,
    ColorWritemask = GL_COLOR_WRITEMASK,
    CompressedTextureFormats = GL_COMPRESSED_TEXTURE_FORMATS,
    MaxComputeShaderStorageBlocks = GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS,
    MaxCombinedShaderStorageBlocks = GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS,
    MaxComputeUniformBlocks = GL_MAX_COMPUTE_UNIFORM_BLOCKS,
    MaxComputeTextureImageUnits = GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS,
    MaxComputeUniformComponents = GL_MAX_COMPUTE_UNIFORM_COMPONENTS,
    MaxComputeAtomicCounters = GL_MAX_COMPUTE_ATOMIC_COUNTERS,
    MaxComputeAtomicCounterBuffers = GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS,
    MaxCombinedComputeUniformComponents = GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS,
    MaxComputeWorkGroupInvocations = GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS,
    MaxComputeWorkGroupCount = GL_MAX_COMPUTE_WORK_GROUP_COUNT,
    MaxComputeWorkGroupSize = GL_MAX_COMPUTE_WORK_GROUP_SIZE,
    DispatchIndirectBufferBinding = GL_DISPATCH_INDIRECT_BUFFER_BINDING,
    MaxDebugGroupStackDepth = GL_MAX_DEBUG_GROUP_STACK_DEPTH,
    DebugGroupStackDepth = GL_DEBUG_GROUP_STACK_DEPTH,
    ContextFlags = GL_CONTEXT_FLAGS,
    CullFace = GL_CULL_FACE,
    CullFaceMode = GL_CULL_FACE_MODE,
    CurrentProgram = GL_CURRENT_PROGRAM,
    DepthClearValue = GL_DEPTH_CLEAR_VALUE,
    DepthFunc = GL_DEPTH_FUNC,
    DepthRange = GL_DEPTH_RANGE,
    DepthTest = GL_DEPTH_TEST,
    DepthWritemask = GL_DEPTH_WRITEMASK,
    Dither = GL_DITHER,
    DoubleBuffer = GL_DOUBLEBUFFER,
    DrawBuffer = GL_DRAW_BUFFER,
    DrawBuffer0 = GL_DRAW_BUFFER0,
    DrawBuffer1 = GL_DRAW_BUFFER1,
    DrawBuffer2 = GL_DRAW_BUFFER2,
    DrawBuffer3 = GL_DRAW_BUFFER3,
    DrawBuffer4 = GL_DRAW_BUFFER4,
    DrawBuffer5 = GL_DRAW_BUFFER5,
    DrawBuffer6 = GL_DRAW_BUFFER6,
    DrawBuffer7 = GL_DRAW_BUFFER7,
    DrawBuffer8 = GL_DRAW_BUFFER8,
    DrawBuffer9 = GL_DRAW_BUFFER9,
    DrawBuffer10 = GL_DRAW_BUFFER10,
    DrawBuffer11 = GL_DRAW_BUFFER11,
    DrawBuffer12 = GL_DRAW_BUFFER12,
    DrawBuffer13 = GL_DRAW_BUFFER13,
    DrawBuffer14 = GL_DRAW_BUFFER14,
    DrawBuffer15 = GL_DRAW_BUFFER15,
    DrawFramebufferBinding = GL_DRAW_FRAMEBUFFER_BINDING,
    FragmentShaderDerivativeHint = GL_FRAGMENT_SHADER_DERIVATIVE_HINT,
    ImplementationColorReadFormat = GL_IMPLEMENTATION_COLOR_READ_FORMAT,
    ImplementationColorReadType = GL_IMPLEMENTATION_COLOR_READ_TYPE,
    LineSmooth = GL_LINE_SMOOTH,
    LineSmoothHint = GL_LINE_SMOOTH_HINT,
    LineWidth = GL_LINE_WIDTH,
    LayerProvokingVertex = GL_LAYER_PROVOKING_VERTEX,
    LogicOpMode = GL_LOGIC_OP_MODE,
    MajorVersion = GL_MAJOR_VERSION,
    Max3dTextureSize = GL_MAX_3D_TEXTURE_SIZE,
    MaxArrayTextureLayers = GL_MAX_ARRAY_TEXTURE_LAYERS,
    MaxClipDistances = GL_MAX_CLIP_DISTANCES,
    MaxColorTextureSamples = GL_MAX_COLOR_TEXTURE_SAMPLES,
    MaxCombinedAtomicCounters = GL_MAX_COMBINED_ATOMIC_COUNTERS,
    MaxCombinedFragmentUniformComponents = GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS,
    MaxCombinedGeometryUniformComponents = GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS,
    MaxCombinedTextureImageUnits = GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS,
    MaxCombinedUniformBlocks = GL_MAX_COMBINED_UNIFORM_BLOCKS,
    MaxCombinedVertexUniformComponents = GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS,
    MaxCubeMapTextureSize = GL_MAX_CUBE_MAP_TEXTURE_SIZE,
    MaxDepthTextureSamples = GL_MAX_DEPTH_TEXTURE_SAMPLES,
    MaxDrawBuffers = GL_MAX_DRAW_BUFFERS,
    MaxDualSourceDrawBuffers = GL_MAX_DUAL_SOURCE_DRAW_BUFFERS,
    MaxElementsIndices = GL_MAX_ELEMENTS_INDICES,
    MaxElementsVertices = GL_MAX_ELEMENTS_VERTICES,
    MaxFragmentAtomicCounters = GL_MAX_FRAGMENT_ATOMIC_COUNTERS,
    MaxFragmentShaderStorageBlocks = GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS,
    MaxFragmentInputComponents = GL_MAX_FRAGMENT_INPUT_COMPONENTS,
    MaxFragmentUniformComponents = GL_MAX_FRAGMENT_UNIFORM_COMPONENTS,
    MaxFragmentUniformVectors = GL_MAX_FRAGMENT_UNIFORM_VECTORS,
    MaxFragmentUniformBlocks = GL_MAX_FRAGMENT_UNIFORM_BLOCKS,
    MaxFramebufferWidth = GL_MAX_FRAMEBUFFER_WIDTH,
    MaxFramebufferHeight = GL_MAX_FRAMEBUFFER_HEIGHT,
    MaxFramebufferLayers = GL_MAX_FRAMEBUFFER_LAYERS,
    MaxFramebufferSamples = GL_MAX_FRAMEBUFFER_SAMPLES,
    MaxGeometryAtomicCounters = GL_MAX_GEOMETRY_ATOMIC_COUNTERS,
    MaxGeometryShaderStorageBlocks = GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS,
    MaxGeometryInputComponents = GL_MAX_GEOMETRY_INPUT_COMPONENTS,
    MaxGeometryOutputComponents = GL_MAX_GEOMETRY_OUTPUT_COMPONENTS,
    MaxGeometryTextureImageUnits = GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS,
    MaxGeometryUniformBlocks = GL_MAX_GEOMETRY_UNIFORM_BLOCKS,
    MaxGeometryUniformComponents = GL_MAX_GEOMETRY_UNIFORM_COMPONENTS,
    MaxIntegerSamples = GL_MAX_INTEGER_SAMPLES,
    MinMapBufferAlignment = GL_MIN_MAP_BUFFER_ALIGNMENT,
    MaxLabelLength = GL_MAX_LABEL_LENGTH,
    MaxProgramTexelOffset = GL_MAX_PROGRAM_TEXEL_OFFSET,
    MinProgramTexelOffset = GL_MIN_PROGRAM_TEXEL_OFFSET,
    MaxRectangleTextureSize = GL_MAX_RECTANGLE_TEXTURE_SIZE,
    MaxRenderbufferSize = GL_MAX_RENDERBUFFER_SIZE,
    MaxSampleMaskWords = GL_MAX_SAMPLE_MASK_WORDS,
    MaxServerWaitTimeout = GL_MAX_SERVER_WAIT_TIMEOUT,
    MaxShaderStorageBufferBindings = GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS,
    MaxTessControlAtomicCounters = GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS,
    MaxTessEvaluationAtomicCounters = GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS,
    MaxTessControlShaderStorageBlocks = GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS,
    MaxTessEvaluationShaderStorageBlocks = GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS,
    MaxTextureBufferSize = GL_MAX_TEXTURE_BUFFER_SIZE,
    MaxTextureImageUnits = GL_MAX_TEXTURE_IMAGE_UNITS,
    MaxTextureLodBias = GL_MAX_TEXTURE_LOD_BIAS,
    MaxTextureSize = GL_MAX_TEXTURE_SIZE,
    MaxUniformBufferBindings = GL_MAX_UNIFORM_BUFFER_BINDINGS,
    MaxUniformBlockSize = GL_MAX_UNIFORM_BLOCK_SIZE,
    MaxUniformLocations = GL_MAX_UNIFORM_LOCATIONS,
    MaxVaryingComponents = GL_MAX_VARYING_COMPONENTS,
    MaxVaryingVectors = GL_MAX_VARYING_VECTORS,
    MaxVertexAtomicCounters = GL_MAX_VERTEX_ATOMIC_COUNTERS,
    MaxVertexAttribs = GL_MAX_VERTEX_ATTRIBS,
    MaxVertexShaderStorageBlocks = GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS,
    MaxVertexTextureImageUnits = GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS,
    MaxVertexUniformComponents = GL_MAX_VERTEX_UNIFORM_COMPONENTS,
    MaxVertexUniformVectors = GL_MAX_VERTEX_UNIFORM_VECTORS,
    MaxVertexOutputComponents = GL_MAX_VERTEX_OUTPUT_COMPONENTS,
    MaxVertexUniformBlocks = GL_MAX_VERTEX_UNIFORM_BLOCKS,
    MaxViewportDims = GL_MAX_VIEWPORT_DIMS,
    MaxViewports = GL_MAX_VIEWPORTS,
    MinorVersion = GL_MINOR_VERSION,
    NumCompressedTextureFormats = GL_NUM_COMPRESSED_TEXTURE_FORMATS,
    NumExtensions = GL_NUM_EXTENSIONS,
    NumProgramBinaryFormats = GL_NUM_PROGRAM_BINARY_FORMATS,
    NumShaderBinaryFormats = GL_NUM_SHADER_BINARY_FORMATS,
    PackAlignment = GL_PACK_ALIGNMENT,
    PackImageHeight = GL_PACK_IMAGE_HEIGHT,
    PackLsbFirst = GL_PACK_LSB_FIRST,
    PackRowLength = GL_PACK_ROW_LENGTH,
    PackSkipImages = GL_PACK_SKIP_IMAGES,
    PackSkipPixels = GL_PACK_SKIP_PIXELS,
    PackSkipRows = GL_PACK_SKIP_ROWS,
    PackSwapBytes = GL_PACK_SWAP_BYTES,
    PixelPackBufferBinding = GL_PIXEL_PACK_BUFFER_BINDING,
    PixelUnpackBufferBinding = GL_PIXEL_UNPACK_BUFFER_BINDING,
    PointFadeThresholdSize = GL_POINT_FADE_THRESHOLD_SIZE,
    PrimitiveRestartIndex = GL_PRIMITIVE_RESTART_INDEX,
    ProgramBinaryFormats = GL_PROGRAM_BINARY_FORMATS,
    ProgramPipelineBinding = GL_PROGRAM_PIPELINE_BINDING,
    ProgramPointSize = GL_PROGRAM_POINT_SIZE,
    ProvokingVertex = GL_PROVOKING_VERTEX,
    PointSize = GL_POINT_SIZE,
    PointSizeGranularity = GL_POINT_SIZE_GRANULARITY,
    PointSizeRange = GL_POINT_SIZE_RANGE,
    PolygonOffsetFactor = GL_POLYGON_OFFSET_FACTOR,
    PolygonOffsetUnits = GL_POLYGON_OFFSET_UNITS,
    PolygonOffsetFill = GL_POLYGON_OFFSET_FILL,
    PolygonOffsetLine = GL_POLYGON_OFFSET_LINE,
    PolygonOffsetPoint = GL_POLYGON_OFFSET_POINT,
    PolygonSmooth = GL_POLYGON_SMOOTH,
    PolygonSmoothHint = GL_POLYGON_SMOOTH_HINT,
    ReadBuffer = GL_READ_BUFFER,
    RenderbufferBinding = GL_RENDERBUFFER_BINDING,
    SampleBuffers = GL_SAMPLE_BUFFERS,
    SampleCoverageValue = GL_SAMPLE_COVERAGE_VALUE,
    SampleCoverageInvert = GL_SAMPLE_COVERAGE_INVERT,
    SamplerBinding = GL_SAMPLER_BINDING,
    Samples = GL_SAMPLES,
    ScissorBox = GL_SCISSOR_BOX,
    ScissorTest = GL_SCISSOR_TEST,
    ShaderCompiler = GL_SHADER_COMPILER,
    ShaderStorageBufferBinding = GL_SHADER_STORAGE_BUFFER_BINDING,
    ShaderStorageBufferOffsetAlignment = GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT,
    ShaderStorageBufferStart = GL_SHADER_STORAGE_BUFFER_START,
    ShaderStorageBufferSize = GL_SHADER_STORAGE_BUFFER_SIZE,
    SmoothLineWidthRange = GL_SMOOTH_LINE_WIDTH_RANGE,
    SmoothLineWidthGranularity = GL_SMOOTH_LINE_WIDTH_GRANULARITY,
    StencilBackFail = GL_STENCIL_BACK_FAIL,
    StencilBackFunc = GL_STENCIL_BACK_FUNC,
    StencilBackPassDepthFail = GL_STENCIL_BACK_PASS_DEPTH_FAIL,
    StencilBackPassDepthPass = GL_STENCIL_BACK_PASS_DEPTH_PASS,
    StencilBackRef = GL_STENCIL_BACK_REF,
    StencilBackValueMask = GL_STENCIL_BACK_VALUE_MASK,
    StencilBackWritemask = GL_STENCIL_BACK_WRITEMASK,
    StencilClearValue = GL_STENCIL_CLEAR_VALUE,
    StencilFail = GL_STENCIL_FAIL,
    StencilFunc = GL_STENCIL_FUNC,
    StencilPassDepthFail = GL_STENCIL_PASS_DEPTH_FAIL,
    StencilPassDepthPass = GL_STENCIL_PASS_DEPTH_PASS,
    StencilRef = GL_STENCIL_REF,
    StencilTest = GL_STENCIL_TEST,
    StencilValueMask = GL_STENCIL_VALUE_MASK,
    StencilWritemask = GL_STENCIL_WRITEMASK,
    Stereo = GL_STEREO,
    SubpixelBits = GL_SUBPIXEL_BITS,
    TextureBinding1d = GL_TEXTURE_BINDING_1D,
    TextureBinding1dArray = GL_TEXTURE_BINDING_1D_ARRAY,
    TextureBinding2d = GL_TEXTURE_BINDING_2D,
    TextureBinding2dArray = GL_TEXTURE_BINDING_2D_ARRAY,
    TextureBinding2dMultisample = GL_TEXTURE_BINDING_2D_MULTISAMPLE,
    TextureBinding2dMultisampleArray = GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY,
    TextureBinding3d = GL_TEXTURE_BINDING_3D,
    TextureBindingBuffer = GL_TEXTURE_BINDING_BUFFER,
    TextureBindingCubeMap = GL_TEXTURE_BINDING_CUBE_MAP,
    TextureBindingRectangle = GL_TEXTURE_BINDING_RECTANGLE,
    TextureCompressionHint = GL_TEXTURE_COMPRESSION_HINT,
    TextureBufferOffsetAlignment = GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT,
    Timestamp = GL_TIMESTAMP,
    TransformFeedbackBufferBinding = GL_TRANSFORM_FEEDBACK_BUFFER_BINDING,
    TransformFeedbackBufferStart = GL_TRANSFORM_FEEDBACK_BUFFER_START,
    TransformFeedbackBufferSize = GL_TRANSFORM_FEEDBACK_BUFFER_SIZE,
    UniformBufferBinding = GL_UNIFORM_BUFFER_BINDING,
    UniformBufferOffsetAlignment = GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT,
    UniformBufferSize = GL_UNIFORM_BUFFER_SIZE,
    UniformBufferStart = GL_UNIFORM_BUFFER_START,
    UnpackAlignment = GL_UNPACK_ALIGNMENT,
    UnpackImageHeight = GL_UNPACK_IMAGE_HEIGHT,
    UnpackLsbFirst = GL_UNPACK_LSB_FIRST,
    UnpackRowLength = GL_UNPACK_ROW_LENGTH,
    UnpackSkipImages = GL_UNPACK_SKIP_IMAGES,
    UnpackSkipPixels = GL_UNPACK_SKIP_PIXELS,
    UnpackSkipRows = GL_UNPACK_SKIP_ROWS,
    UnpackSwapBytes = GL_UNPACK_SWAP_BYTES,
    VertexArrayBinding = GL_VERTEX_ARRAY_BINDING,
    VertexBindingDivisor = GL_VERTEX_BINDING_DIVISOR,
    VertexBindingOffset = GL_VERTEX_BINDING_OFFSET,
    VertexBindingStride = GL_VERTEX_BINDING_STRIDE,
    VertexBindingBuffer = GL_VERTEX_BINDING_BUFFER,
    MaxVertexAttribRelativeOffset = GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET,
    MaxVertexAttribBindings = GL_MAX_VERTEX_ATTRIB_BINDINGS,
    Viewport = GL_VIEWPORT,
    ViewportBoundsRange = GL_VIEWPORT_BOUNDS_RANGE,
    ViewportIndexProvokingVertex = GL_VIEWPORT_INDEX_PROVOKING_VERTEX,
    ViewportSubpixelBits = GL_VIEWPORT_SUBPIXEL_BITS,
    MaxElementIndex = GL_MAX_ELEMENT_INDEX,
}

impl Parameter {
    #[allow(non_upper_case_globals)]
    pub const MaxVaryingFloats: Self = Self::MaxVaryingVectors;
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum StringParameter {
    Vendor = GL_VENDOR,
    Renderer = GL_RENDERER,
    Version = GL_VERSION,
    ShadingLanguageVersion = GL_SHADING_LANGUAGE_VERSION,
}

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

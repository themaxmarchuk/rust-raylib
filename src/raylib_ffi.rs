pub const MAX_SHADER_LOCATIONS: usize = 32;
pub const MAX_MATERIAL_MAPS: usize = 12;

#[repr(C)]
pub struct rAudioBuffer { _empty: core::marker::PhantomData<()> }
#[repr(C)]
pub struct rAudioProcessor { _empty: core::marker::PhantomData<()> }

pub mod colors {
	pub const LIGHTGRAY: super::Color = super::Color { r: 200, g: 200, b: 200, a: 255 };
	pub const GRAY: super::Color = super::Color { r: 130, g: 130, b: 130, a: 255 };
	pub const DARKGRAY: super::Color = super::Color { r: 80, g: 80, b: 80, a: 255 };
	pub const YELLOW: super::Color = super::Color { r: 253, g: 249, b: 0, a: 255 };
	pub const GOLD: super::Color = super::Color { r: 255, g: 203, b: 0, a: 255 };
	pub const ORANGE: super::Color = super::Color { r: 255, g: 161, b: 0, a: 255 };
	pub const PINK: super::Color = super::Color { r: 255, g: 109, b: 194, a: 255 };
	pub const RED: super::Color = super::Color { r: 230, g: 41, b: 55, a: 255 };
	pub const MAROON: super::Color = super::Color { r: 190, g: 33, b: 55, a: 255 };
	pub const GREEN: super::Color = super::Color { r: 0, g: 228, b: 48, a: 255 };
	pub const LIME: super::Color = super::Color { r: 0, g: 158, b: 47, a: 255 };
	pub const DARKGREEN: super::Color = super::Color { r: 0, g: 117, b: 44, a: 255 };
	pub const SKYBLUE: super::Color = super::Color { r: 102, g: 191, b: 255, a: 255 };
	pub const BLUE: super::Color = super::Color { r: 0, g: 121, b: 241, a: 255 };
	pub const DARKBLUE: super::Color = super::Color { r: 0, g: 82, b: 172, a: 255 };
	pub const PURPLE: super::Color = super::Color { r: 200, g: 122, b: 255, a: 255 };
	pub const VIOLET: super::Color = super::Color { r: 135, g: 60, b: 190, a: 255 };
	pub const DARKPURPLE: super::Color = super::Color { r: 112, g: 31, b: 126, a: 255 };
	pub const BEIGE: super::Color = super::Color { r: 211, g: 176, b: 131, a: 255 };
	pub const BROWN: super::Color = super::Color { r: 127, g: 106, b: 79, a: 255 };
	pub const DARKBROWN: super::Color = super::Color { r: 76, g: 63, b: 47, a: 255 };
	pub const WHITE: super::Color = super::Color { r: 255, g: 255, b: 255, a: 255 };
	pub const BLACK: super::Color = super::Color { r: 0, g: 0, b: 0, a: 255 };
	pub const BLANK: super::Color = super::Color { r: 0, g: 0, b: 0, a: 0 };
	pub const MAGENTA: super::Color = super::Color { r: 255, g: 0, b: 255, a: 255 };
	pub const RAYWHITE: super::Color = super::Color { r: 245, g: 245, b: 245, a: 255 };
}

pub const RAYLIB_VERSION_MAJOR: u32 = 4;
pub const RAYLIB_VERSION_MINOR: u32 = 5;
pub const RAYLIB_VERSION_PATCH: u32 = 0;
pub const RAYLIB_VERSION: &str = "4.5";

/// Vector2, 2 components
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vector2 {
	/// Vector x component
	pub x: core::ffi::c_float,
	/// Vector y component
	pub y: core::ffi::c_float,
}

/// Vector3, 3 components
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vector3 {
	/// Vector x component
	pub x: core::ffi::c_float,
	/// Vector y component
	pub y: core::ffi::c_float,
	/// Vector z component
	pub z: core::ffi::c_float,
}

/// Vector4, 4 components
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vector4 {
	/// Vector x component
	pub x: core::ffi::c_float,
	/// Vector y component
	pub y: core::ffi::c_float,
	/// Vector z component
	pub z: core::ffi::c_float,
	/// Vector w component
	pub w: core::ffi::c_float,
}

/// Matrix, 4x4 components, column major, OpenGL style, right-handed
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Matrix {
	/// Matrix first row (4 components)
	pub m0: core::ffi::c_float,
	/// Matrix first row (4 components)
	pub m4: core::ffi::c_float,
	/// Matrix first row (4 components)
	pub m8: core::ffi::c_float,
	/// Matrix first row (4 components)
	pub m12: core::ffi::c_float,
	/// Matrix second row (4 components)
	pub m1: core::ffi::c_float,
	/// Matrix second row (4 components)
	pub m5: core::ffi::c_float,
	/// Matrix second row (4 components)
	pub m9: core::ffi::c_float,
	/// Matrix second row (4 components)
	pub m13: core::ffi::c_float,
	/// Matrix third row (4 components)
	pub m2: core::ffi::c_float,
	/// Matrix third row (4 components)
	pub m6: core::ffi::c_float,
	/// Matrix third row (4 components)
	pub m10: core::ffi::c_float,
	/// Matrix third row (4 components)
	pub m14: core::ffi::c_float,
	/// Matrix fourth row (4 components)
	pub m3: core::ffi::c_float,
	/// Matrix fourth row (4 components)
	pub m7: core::ffi::c_float,
	/// Matrix fourth row (4 components)
	pub m11: core::ffi::c_float,
	/// Matrix fourth row (4 components)
	pub m15: core::ffi::c_float,
}

/// Color, 4 components, R8G8B8A8 (32bit)
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Color {
	/// Color red value
	pub r: core::ffi::c_uchar,
	/// Color green value
	pub g: core::ffi::c_uchar,
	/// Color blue value
	pub b: core::ffi::c_uchar,
	/// Color alpha value
	pub a: core::ffi::c_uchar,
}

/// Rectangle, 4 components
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Rectangle {
	/// Rectangle top-left corner position x
	pub x: core::ffi::c_float,
	/// Rectangle top-left corner position y
	pub y: core::ffi::c_float,
	/// Rectangle width
	pub width: core::ffi::c_float,
	/// Rectangle height
	pub height: core::ffi::c_float,
}

/// Image, pixel data stored in CPU memory (RAM)
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Image {
	/// Image raw data
	pub data: *mut core::ffi::c_void,
	/// Image base width
	pub width: core::ffi::c_int,
	/// Image base height
	pub height: core::ffi::c_int,
	/// Mipmap levels, 1 by default
	pub mipmaps: core::ffi::c_int,
	/// Data format (PixelFormat type)
	pub format: core::ffi::c_int,
}

/// Texture, tex data stored in GPU memory (VRAM)
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Texture {
	/// OpenGL texture id
	pub id: core::ffi::c_uint,
	/// Texture base width
	pub width: core::ffi::c_int,
	/// Texture base height
	pub height: core::ffi::c_int,
	/// Mipmap levels, 1 by default
	pub mipmaps: core::ffi::c_int,
	/// Data format (PixelFormat type)
	pub format: core::ffi::c_int,
}

/// RenderTexture, fbo for texture rendering
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderTexture {
	/// OpenGL framebuffer object id
	pub id: core::ffi::c_uint,
	/// Color buffer attachment texture
	pub texture: Texture,
	/// Depth buffer attachment texture
	pub depth: Texture,
}

/// NPatchInfo, n-patch layout info
#[repr(C)]
#[derive(Clone, Debug)]
pub struct NPatchInfo {
	/// Texture source rectangle
	pub source: Rectangle,
	/// Left border offset
	pub left: core::ffi::c_int,
	/// Top border offset
	pub top: core::ffi::c_int,
	/// Right border offset
	pub right: core::ffi::c_int,
	/// Bottom border offset
	pub bottom: core::ffi::c_int,
	/// Layout of the n-patch: 3x3, 1x3 or 3x1
	pub layout: core::ffi::c_int,
}

/// GlyphInfo, font characters glyphs info
#[repr(C)]
#[derive(Clone, Debug)]
pub struct GlyphInfo {
	/// Character value (Unicode)
	pub value: core::ffi::c_int,
	/// Character offset X when drawing
	pub offsetX: core::ffi::c_int,
	/// Character offset Y when drawing
	pub offsetY: core::ffi::c_int,
	/// Character advance position X
	pub advanceX: core::ffi::c_int,
	/// Character image data
	pub image: Image,
}

/// Font, font texture and GlyphInfo array data
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Font {
	/// Base size (default chars height)
	pub baseSize: core::ffi::c_int,
	/// Number of glyph characters
	pub glyphCount: core::ffi::c_int,
	/// Padding around the glyph characters
	pub glyphPadding: core::ffi::c_int,
	/// Texture atlas containing the glyphs
	pub texture: Texture2D,
	/// Rectangles in texture for the glyphs
	pub recs: *mut Rectangle,
	/// Glyphs info data
	pub glyphs: *mut GlyphInfo,
}

/// Camera, defines position/orientation in 3d space
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Camera3D {
	/// Camera position
	pub position: Vector3,
	/// Camera target it looks-at
	pub target: Vector3,
	/// Camera up vector (rotation over its axis)
	pub up: Vector3,
	/// Camera field-of-view aperture in Y (degrees) in perspective, used as near plane width in orthographic
	pub fovy: core::ffi::c_float,
	/// Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
	pub projection: core::ffi::c_int,
}

/// Camera2D, defines position/orientation in 2d space
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Camera2D {
	/// Camera offset (displacement from target)
	pub offset: Vector2,
	/// Camera target (rotation and zoom origin)
	pub target: Vector2,
	/// Camera rotation in degrees
	pub rotation: core::ffi::c_float,
	/// Camera zoom (scaling), should be 1.0f by default
	pub zoom: core::ffi::c_float,
}

/// Mesh, vertex data and vao/vbo
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mesh {
	/// Number of vertices stored in arrays
	pub vertexCount: core::ffi::c_int,
	/// Number of triangles stored (indexed or not)
	pub triangleCount: core::ffi::c_int,
	/// Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
	pub vertices: *mut core::ffi::c_float,
	/// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
	pub texcoords: *mut core::ffi::c_float,
	/// Vertex texture second coordinates (UV - 2 components per vertex) (shader-location = 5)
	pub texcoords2: *mut core::ffi::c_float,
	/// Vertex normals (XYZ - 3 components per vertex) (shader-location = 2)
	pub normals: *mut core::ffi::c_float,
	/// Vertex tangents (XYZW - 4 components per vertex) (shader-location = 4)
	pub tangents: *mut core::ffi::c_float,
	/// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
	pub colors: *mut core::ffi::c_uchar,
	/// Vertex indices (in case vertex data comes indexed)
	pub indices: *mut core::ffi::c_ushort,
	/// Animated vertex positions (after bones transformations)
	pub animVertices: *mut core::ffi::c_float,
	/// Animated normals (after bones transformations)
	pub animNormals: *mut core::ffi::c_float,
	/// Vertex bone ids, max 255 bone ids, up to 4 bones influence by vertex (skinning)
	pub boneIds: *mut core::ffi::c_uchar,
	/// Vertex bone weight, up to 4 bones influence by vertex (skinning)
	pub boneWeights: *mut core::ffi::c_float,
	/// OpenGL Vertex Array Object id
	pub vaoId: core::ffi::c_uint,
	/// OpenGL Vertex Buffer Objects id (default vertex data)
	pub vboId: *mut core::ffi::c_uint,
}

/// Shader
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Shader {
	/// Shader program id
	pub id: core::ffi::c_uint,
	/// Shader locations array (RL_MAX_SHADER_LOCATIONS)
	pub locs: *mut core::ffi::c_int,
}

/// MaterialMap
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MaterialMap {
	/// Material map texture
	pub texture: Texture2D,
	/// Material map color
	pub color: Color,
	/// Material map value
	pub value: core::ffi::c_float,
}

/// Material, includes shader and maps
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Material {
	/// Material shader
	pub shader: Shader,
	/// Material maps array (MAX_MATERIAL_MAPS)
	pub maps: *mut MaterialMap,
	/// Material generic parameters (if required)
	pub params: [core::ffi::c_float; 4],
}

/// Transform, vertex transformation data
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Transform {
	/// Translation
	pub translation: Vector3,
	/// Rotation
	pub rotation: Quaternion,
	/// Scale
	pub scale: Vector3,
}

/// Bone, skeletal animation bone
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BoneInfo {
	/// Bone name
	pub name: [core::ffi::c_char; 32],
	/// Bone parent
	pub parent: core::ffi::c_int,
}

/// Model, meshes, materials and animation data
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Model {
	/// Local transform matrix
	pub transform: Matrix,
	/// Number of meshes
	pub meshCount: core::ffi::c_int,
	/// Number of materials
	pub materialCount: core::ffi::c_int,
	/// Meshes array
	pub meshes: *mut Mesh,
	/// Materials array
	pub materials: *mut Material,
	/// Mesh material number
	pub meshMaterial: *mut core::ffi::c_int,
	/// Number of bones
	pub boneCount: core::ffi::c_int,
	/// Bones information (skeleton)
	pub bones: *mut BoneInfo,
	/// Bones base transformation (pose)
	pub bindPose: *mut Transform,
}

/// ModelAnimation
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ModelAnimation {
	/// Number of bones
	pub boneCount: core::ffi::c_int,
	/// Number of animation frames
	pub frameCount: core::ffi::c_int,
	/// Bones information (skeleton)
	pub bones: *mut BoneInfo,
	/// Poses array by frame
	pub framePoses: *mut *mut Transform,
}

/// Ray, ray for raycasting
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Ray {
	/// Ray position (origin)
	pub position: Vector3,
	/// Ray direction
	pub direction: Vector3,
}

/// RayCollision, ray hit information
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RayCollision {
	/// Did the ray hit something?
	pub hit: bool,
	/// Distance to the nearest hit
	pub distance: core::ffi::c_float,
	/// Point of the nearest hit
	pub point: Vector3,
	/// Surface normal of hit
	pub normal: Vector3,
}

/// BoundingBox
#[repr(C)]
#[derive(Clone, Debug)]
pub struct BoundingBox {
	/// Minimum vertex box-corner
	pub min: Vector3,
	/// Maximum vertex box-corner
	pub max: Vector3,
}

/// Wave, audio wave data
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Wave {
	/// Total number of frames (considering channels)
	pub frameCount: core::ffi::c_uint,
	/// Frequency (samples per second)
	pub sampleRate: core::ffi::c_uint,
	/// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
	pub sampleSize: core::ffi::c_uint,
	/// Number of channels (1-mono, 2-stereo, ...)
	pub channels: core::ffi::c_uint,
	/// Buffer data pointer
	pub data: *mut core::ffi::c_void,
}

/// AudioStream, custom audio stream
#[repr(C)]
#[derive(Clone, Debug)]
pub struct AudioStream {
	/// Pointer to internal data used by the audio system
	pub buffer: *mut rAudioBuffer,
	/// Pointer to internal data processor, useful for audio effects
	pub processor: *mut rAudioProcessor,
	/// Frequency (samples per second)
	pub sampleRate: core::ffi::c_uint,
	/// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
	pub sampleSize: core::ffi::c_uint,
	/// Number of channels (1-mono, 2-stereo, ...)
	pub channels: core::ffi::c_uint,
}

/// Sound
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Sound {
	/// Audio stream
	pub stream: AudioStream,
	/// Total number of frames (considering channels)
	pub frameCount: core::ffi::c_uint,
}

/// Music, audio stream, anything longer than ~10 seconds should be streamed
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Music {
	/// Audio stream
	pub stream: AudioStream,
	/// Total number of frames (considering channels)
	pub frameCount: core::ffi::c_uint,
	/// Music looping enable
	pub looping: bool,
	/// Type of music context (audio filetype)
	pub ctxType: core::ffi::c_int,
	/// Audio context data, depends on type
	pub ctxData: *mut core::ffi::c_void,
}

/// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VrDeviceInfo {
	/// Horizontal resolution in pixels
	pub hResolution: core::ffi::c_int,
	/// Vertical resolution in pixels
	pub vResolution: core::ffi::c_int,
	/// Horizontal size in meters
	pub hScreenSize: core::ffi::c_float,
	/// Vertical size in meters
	pub vScreenSize: core::ffi::c_float,
	/// Screen center in meters
	pub vScreenCenter: core::ffi::c_float,
	/// Distance between eye and display in meters
	pub eyeToScreenDistance: core::ffi::c_float,
	/// Lens separation distance in meters
	pub lensSeparationDistance: core::ffi::c_float,
	/// IPD (distance between pupils) in meters
	pub interpupillaryDistance: core::ffi::c_float,
	/// Lens distortion constant parameters
	pub lensDistortionValues: [core::ffi::c_float; 4],
	/// Chromatic aberration correction parameters
	pub chromaAbCorrection: [core::ffi::c_float; 4],
}

/// VrStereoConfig, VR stereo rendering configuration for simulator
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VrStereoConfig {
	/// VR projection matrices (per eye)
	pub projection: [Matrix; 2],
	/// VR view offset matrices (per eye)
	pub viewOffset: [Matrix; 2],
	/// VR left lens center
	pub leftLensCenter: [core::ffi::c_float; 2],
	/// VR right lens center
	pub rightLensCenter: [core::ffi::c_float; 2],
	/// VR left screen center
	pub leftScreenCenter: [core::ffi::c_float; 2],
	/// VR right screen center
	pub rightScreenCenter: [core::ffi::c_float; 2],
	/// VR distortion scale
	pub scale: [core::ffi::c_float; 2],
	/// VR distortion scale in
	pub scaleIn: [core::ffi::c_float; 2],
}

/// File path list
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FilePathList {
	/// Filepaths max entries
	pub capacity: core::ffi::c_uint,
	/// Filepaths entries count
	pub count: core::ffi::c_uint,
	/// Filepaths entries
	pub paths: *mut *mut core::ffi::c_char,
}

/// Quaternion, 4 components (Vector4 alias)
pub type Quaternion = Vector4;

/// Texture2D, same as Texture
pub type Texture2D = Texture;

/// TextureCubemap, same as Texture
pub type TextureCubemap = Texture;

/// RenderTexture2D, same as RenderTexture
pub type RenderTexture2D = RenderTexture;

/// Camera type fallback, defaults to Camera3D
pub type Camera = Camera3D;

/// System/Window config flags
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigFlags(pub(crate) u32);

bitflags::bitflags! {
	impl ConfigFlags: u32 {
		/// Set to try enabling V-Sync on GPU
		const VSYNC_HINT = 64;
		/// Set to run program in fullscreen
		const FULLSCREEN_MODE = 2;
		/// Set to allow resizable window
		const WINDOW_RESIZABLE = 4;
		/// Set to disable window decoration (frame and buttons)
		const WINDOW_UNDECORATED = 8;
		/// Set to hide window
		const WINDOW_HIDDEN = 128;
		/// Set to minimize window (iconify)
		const WINDOW_MINIMIZED = 512;
		/// Set to maximize window (expanded to monitor)
		const WINDOW_MAXIMIZED = 1024;
		/// Set to window non focused
		const WINDOW_UNFOCUSED = 2048;
		/// Set to window always on top
		const WINDOW_TOPMOST = 4096;
		/// Set to allow windows running while minimized
		const WINDOW_ALWAYS_RUN = 256;
		/// Set to allow transparent framebuffer
		const WINDOW_TRANSPARENT = 16;
		/// Set to support HighDPI
		const WINDOW_HIGHDPI = 8192;
		/// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
		const WINDOW_MOUSE_PASSTHROUGH = 16384;
		/// Set to try enabling MSAA 4X
		const MSAA_4X_HINT = 32;
		/// Set to try enabling interlaced video format (for V3D)
		const INTERLACED_HINT = 65536;

		const _ = !0;
	}
}

/// Trace log level
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TraceLogLevel {
	/// Display all logs
	All = 0,
	/// Trace logging, intended for internal use only
	Trace = 1,
	/// Debug logging, used for internal debugging, it should be disabled on release builds
	Debug = 2,
	/// Info logging, used for program execution info
	Info = 3,
	/// Warning logging, used on recoverable failures
	Warning = 4,
	/// Error logging, used on unrecoverable failures
	Error = 5,
	/// Fatal logging, used to abort program: exit(EXIT_FAILURE)
	Fatal = 6,
	/// Disable logging
	None = 7,
}

/// Keyboard keys (US keyboard layout)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyboardKey {
	/// Key: NULL, used for no key pressed
	Null = 0,
	/// Key: '
	Apostrophe = 39,
	/// Key: ,
	Comma = 44,
	/// Key: -
	Minus = 45,
	/// Key: .
	Period = 46,
	/// Key: /
	Slash = 47,
	/// Key: 0
	Zero = 48,
	/// Key: 1
	One = 49,
	/// Key: 2
	Two = 50,
	/// Key: 3
	Three = 51,
	/// Key: 4
	Four = 52,
	/// Key: 5
	Five = 53,
	/// Key: 6
	Six = 54,
	/// Key: 7
	Seven = 55,
	/// Key: 8
	Eight = 56,
	/// Key: 9
	Nine = 57,
	/// Key: ;
	Semicolon = 59,
	/// Key: =
	Equal = 61,
	/// Key: A | a
	A = 65,
	/// Key: B | b
	B = 66,
	/// Key: C | c
	C = 67,
	/// Key: D | d
	D = 68,
	/// Key: E | e
	E = 69,
	/// Key: F | f
	F = 70,
	/// Key: G | g
	G = 71,
	/// Key: H | h
	H = 72,
	/// Key: I | i
	I = 73,
	/// Key: J | j
	J = 74,
	/// Key: K | k
	K = 75,
	/// Key: L | l
	L = 76,
	/// Key: M | m
	M = 77,
	/// Key: N | n
	N = 78,
	/// Key: O | o
	O = 79,
	/// Key: P | p
	P = 80,
	/// Key: Q | q
	Q = 81,
	/// Key: R | r
	R = 82,
	/// Key: S | s
	S = 83,
	/// Key: T | t
	T = 84,
	/// Key: U | u
	U = 85,
	/// Key: V | v
	V = 86,
	/// Key: W | w
	W = 87,
	/// Key: X | x
	X = 88,
	/// Key: Y | y
	Y = 89,
	/// Key: Z | z
	Z = 90,
	/// Key: [
	LeftBracket = 91,
	/// Key: '\'
	Backslash = 92,
	/// Key: ]
	RightBracket = 93,
	/// Key: `
	Grave = 96,
	/// Key: Space
	Space = 32,
	/// Key: Esc
	Escape = 256,
	/// Key: Enter
	Enter = 257,
	/// Key: Tab
	Tab = 258,
	/// Key: Backspace
	Backspace = 259,
	/// Key: Ins
	Insert = 260,
	/// Key: Del
	Delete = 261,
	/// Key: Cursor right
	Right = 262,
	/// Key: Cursor left
	Left = 263,
	/// Key: Cursor down
	Down = 264,
	/// Key: Cursor up
	Up = 265,
	/// Key: Page up
	PageUp = 266,
	/// Key: Page down
	PageDown = 267,
	/// Key: Home
	Home = 268,
	/// Key: End
	End = 269,
	/// Key: Caps lock
	CapsLock = 280,
	/// Key: Scroll down
	ScrollLock = 281,
	/// Key: Num lock
	NumLock = 282,
	/// Key: Print screen
	PrintScreen = 283,
	/// Key: Pause
	Pause = 284,
	/// Key: F1
	F1 = 290,
	/// Key: F2
	F2 = 291,
	/// Key: F3
	F3 = 292,
	/// Key: F4
	F4 = 293,
	/// Key: F5
	F5 = 294,
	/// Key: F6
	F6 = 295,
	/// Key: F7
	F7 = 296,
	/// Key: F8
	F8 = 297,
	/// Key: F9
	F9 = 298,
	/// Key: F10
	F10 = 299,
	/// Key: F11
	F11 = 300,
	/// Key: F12
	F12 = 301,
	/// Key: Shift left
	LeftShift = 340,
	/// Key: Control left
	LeftControl = 341,
	/// Key: Alt left
	LeftAlt = 342,
	/// Key: Super left
	LeftSuper = 343,
	/// Key: Shift right
	RightShift = 344,
	/// Key: Control right
	RightControl = 345,
	/// Key: Alt right
	RightAlt = 346,
	/// Key: Super right
	RightSuper = 347,
	/// Key: KB menu
	KbMenu = 348,
	/// Key: Keypad 0
	Kp0 = 320,
	/// Key: Keypad 1
	Kp1 = 321,
	/// Key: Keypad 2
	Kp2 = 322,
	/// Key: Keypad 3
	Kp3 = 323,
	/// Key: Keypad 4
	Kp4 = 324,
	/// Key: Keypad 5
	Kp5 = 325,
	/// Key: Keypad 6
	Kp6 = 326,
	/// Key: Keypad 7
	Kp7 = 327,
	/// Key: Keypad 8
	Kp8 = 328,
	/// Key: Keypad 9
	Kp9 = 329,
	/// Key: Keypad .
	KpDecimal = 330,
	/// Key: Keypad /
	KpDivide = 331,
	/// Key: Keypad *
	KpMultiply = 332,
	/// Key: Keypad -
	KpSubtract = 333,
	/// Key: Keypad +
	KpAdd = 334,
	/// Key: Keypad Enter
	KpEnter = 335,
	/// Key: Keypad =
	KpEqual = 336,
	/// Key: Android back button
	Back = 4,
	/// Key: Android volume up button
	VolumeUp = 24,
	/// Key: Android volume down button
	VolumeDown = 25,
}

/// Mouse buttons
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
	/// Mouse button left
	Left = 0,
	/// Mouse button right
	Right = 1,
	/// Mouse button middle (pressed wheel)
	Middle = 2,
	/// Mouse button side (advanced mouse device)
	Side = 3,
	/// Mouse button extra (advanced mouse device)
	Extra = 4,
	/// Mouse button forward (advanced mouse device)
	Forward = 5,
	/// Mouse button back (advanced mouse device)
	Back = 6,
}

/// Mouse cursor
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseCursor {
	/// Default pointer shape
	Default = 0,
	/// Arrow shape
	Arrow = 1,
	/// Text writing cursor shape
	Ibeam = 2,
	/// Cross shape
	Crosshair = 3,
	/// Pointing hand cursor
	PointingHand = 4,
	/// Horizontal resize/move arrow shape
	ResizeEw = 5,
	/// Vertical resize/move arrow shape
	ResizeNs = 6,
	/// Top-left to bottom-right diagonal resize/move arrow shape
	ResizeNwse = 7,
	/// The top-right to bottom-left diagonal resize/move arrow shape
	ResizeNesw = 8,
	/// The omnidirectional resize/move cursor shape
	ResizeAll = 9,
	/// The operation-not-allowed shape
	NotAllowed = 10,
}

/// Gamepad buttons
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GamepadButton {
	/// Unknown button, just for error checking
	Unknown = 0,
	/// Gamepad left DPAD up button
	LeftFaceUp = 1,
	/// Gamepad left DPAD right button
	LeftFaceRight = 2,
	/// Gamepad left DPAD down button
	LeftFaceDown = 3,
	/// Gamepad left DPAD left button
	LeftFaceLeft = 4,
	/// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
	RightFaceUp = 5,
	/// Gamepad right button right (i.e. PS3: Square, Xbox: X)
	RightFaceRight = 6,
	/// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
	RightFaceDown = 7,
	/// Gamepad right button left (i.e. PS3: Circle, Xbox: B)
	RightFaceLeft = 8,
	/// Gamepad top/back trigger left (first), it could be a trailing button
	LeftTrigger1 = 9,
	/// Gamepad top/back trigger left (second), it could be a trailing button
	LeftTrigger2 = 10,
	/// Gamepad top/back trigger right (one), it could be a trailing button
	RightTrigger1 = 11,
	/// Gamepad top/back trigger right (second), it could be a trailing button
	RightTrigger2 = 12,
	/// Gamepad center buttons, left one (i.e. PS3: Select)
	MiddleLeft = 13,
	/// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
	Middle = 14,
	/// Gamepad center buttons, right one (i.e. PS3: Start)
	MiddleRight = 15,
	/// Gamepad joystick pressed button left
	LeftThumb = 16,
	/// Gamepad joystick pressed button right
	RightThumb = 17,
}

/// Gamepad axis
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GamepadAxis {
	/// Gamepad left stick X axis
	LeftX = 0,
	/// Gamepad left stick Y axis
	LeftY = 1,
	/// Gamepad right stick X axis
	RightX = 2,
	/// Gamepad right stick Y axis
	RightY = 3,
	/// Gamepad back trigger left, pressure level: [1..-1]
	LeftTrigger = 4,
	/// Gamepad back trigger right, pressure level: [1..-1]
	RightTrigger = 5,
}

/// Material map index
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MaterialMapIndex {
	/// Albedo material (same as: MATERIAL_MAP_DIFFUSE)
	Albedo = 0,
	/// Metalness material (same as: MATERIAL_MAP_SPECULAR)
	Metalness = 1,
	/// Normal material
	Normal = 2,
	/// Roughness material
	Roughness = 3,
	/// Ambient occlusion material
	Occlusion = 4,
	/// Emission material
	Emission = 5,
	/// Heightmap material
	Height = 6,
	/// Cubemap material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	Cubemap = 7,
	/// Irradiance material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	Irradiance = 8,
	/// Prefilter material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	Prefilter = 9,
	/// Brdf material
	Brdf = 10,
}

/// Shader location index
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShaderLocationIndex {
	/// Shader location: vertex attribute: position
	VertexPosition = 0,
	/// Shader location: vertex attribute: texcoord01
	VertexTexcoord01 = 1,
	/// Shader location: vertex attribute: texcoord02
	VertexTexcoord02 = 2,
	/// Shader location: vertex attribute: normal
	VertexNormal = 3,
	/// Shader location: vertex attribute: tangent
	VertexTangent = 4,
	/// Shader location: vertex attribute: color
	VertexColor = 5,
	/// Shader location: matrix uniform: model-view-projection
	MatrixMvp = 6,
	/// Shader location: matrix uniform: view (camera transform)
	MatrixView = 7,
	/// Shader location: matrix uniform: projection
	MatrixProjection = 8,
	/// Shader location: matrix uniform: model (transform)
	MatrixModel = 9,
	/// Shader location: matrix uniform: normal
	MatrixNormal = 10,
	/// Shader location: vector uniform: view
	VectorView = 11,
	/// Shader location: vector uniform: diffuse color
	ColorDiffuse = 12,
	/// Shader location: vector uniform: specular color
	ColorSpecular = 13,
	/// Shader location: vector uniform: ambient color
	ColorAmbient = 14,
	/// Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
	MapAlbedo = 15,
	/// Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
	MapMetalness = 16,
	/// Shader location: sampler2d texture: normal
	MapNormal = 17,
	/// Shader location: sampler2d texture: roughness
	MapRoughness = 18,
	/// Shader location: sampler2d texture: occlusion
	MapOcclusion = 19,
	/// Shader location: sampler2d texture: emission
	MapEmission = 20,
	/// Shader location: sampler2d texture: height
	MapHeight = 21,
	/// Shader location: samplerCube texture: cubemap
	MapCubemap = 22,
	/// Shader location: samplerCube texture: irradiance
	MapIrradiance = 23,
	/// Shader location: samplerCube texture: prefilter
	MapPrefilter = 24,
	/// Shader location: sampler2d texture: brdf
	MapBrdf = 25,
}

/// Shader uniform data type
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShaderUniformDataType {
	/// Shader uniform type: float
	Float = 0,
	/// Shader uniform type: vec2 (2 float)
	Vec2 = 1,
	/// Shader uniform type: vec3 (3 float)
	Vec3 = 2,
	/// Shader uniform type: vec4 (4 float)
	Vec4 = 3,
	/// Shader uniform type: int
	Int = 4,
	/// Shader uniform type: ivec2 (2 int)
	IVec2 = 5,
	/// Shader uniform type: ivec3 (3 int)
	IVec3 = 6,
	/// Shader uniform type: ivec4 (4 int)
	IVec4 = 7,
	/// Shader uniform type: sampler2d
	Sampler2D = 8,
}

/// Shader attribute data types
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShaderAttributeDataType {
	/// Shader attribute type: float
	Float = 0,
	/// Shader attribute type: vec2 (2 float)
	Vec2 = 1,
	/// Shader attribute type: vec3 (3 float)
	Vec3 = 2,
	/// Shader attribute type: vec4 (4 float)
	Vec4 = 3,
}

/// Pixel formats
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PixelFormat {
	/// 8 bit per pixel (no alpha)
	Grayscale = 1,
	/// 8*2 bpp (2 channels)
	GrayAlpha = 2,
	/// 16 bpp
	R5G6B5 = 3,
	/// 24 bpp
	R8G8B8 = 4,
	/// 16 bpp (1 bit alpha)
	R5G5B5A1 = 5,
	/// 16 bpp (4 bit alpha)
	R4G4B4A4 = 6,
	/// 32 bpp
	R8G8B8A8 = 7,
	/// 32 bpp (1 channel - float)
	R32 = 8,
	/// 32*3 bpp (3 channels - float)
	R32G32B32 = 9,
	/// 32*4 bpp (4 channels - float)
	R32G32B32A32 = 10,
	/// 4 bpp (no alpha)
	DXT1Rgb = 11,
	/// 4 bpp (1 bit alpha)
	DXT1Rgba = 12,
	/// 8 bpp
	DXT3Rgba = 13,
	/// 8 bpp
	DXT5Rgba = 14,
	/// 4 bpp
	ETC1Rgb = 15,
	/// 4 bpp
	ETC2Rgb = 16,
	/// 8 bpp
	ETC2EacRgba = 17,
	/// 4 bpp
	PvrtRgb = 18,
	/// 4 bpp
	PvrtRgba = 19,
	/// 8 bpp
	Astc4x4Rgba = 20,
	/// 2 bpp
	Astc8x8Rgba = 21,
}

/// Texture parameters: filter mode
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureFilter {
	/// No filter, just pixel approximation
	Point = 0,
	/// Linear filtering
	Bilinear = 1,
	/// Trilinear filtering (linear with mipmaps)
	Trilinear = 2,
	/// Anisotropic filtering 4x
	Anisotropic4x = 3,
	/// Anisotropic filtering 8x
	Anisotropic8x = 4,
	/// Anisotropic filtering 16x
	Anisotropic16x = 5,
}

/// Texture parameters: wrap mode
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureWrap {
	/// Repeats texture in tiled mode
	Repeat = 0,
	/// Clamps texture to edge pixel in tiled mode
	Clamp = 1,
	/// Mirrors and repeats the texture in tiled mode
	MirrorRepeat = 2,
	/// Mirrors and clamps to border the texture in tiled mode
	MirrorClamp = 3,
}

/// Cubemap layouts
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CubemapLayout {
	/// Automatically detect layout type
	AutoDetect = 0,
	/// Layout is defined by a vertical line with faces
	LineVertical = 1,
	/// Layout is defined by a horizontal line with faces
	LineHorizontal = 2,
	/// Layout is defined by a 3x4 cross with cubemap faces
	CrossThreeByFour = 3,
	/// Layout is defined by a 4x3 cross with cubemap faces
	CrossFourByThree = 4,
	/// Layout is defined by a panorama image (equirrectangular map)
	Panorama = 5,
}

/// Font type, defines generation method
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontType {
	/// Default font generation, anti-aliased
	Default = 0,
	/// Bitmap font generation, no anti-aliasing
	Bitmap = 1,
	/// SDF font generation, requires external shader
	Sdf = 2,
}

/// Color blending modes (pre-defined)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BlendMode {
	/// Blend textures considering alpha (default)
	Alpha = 0,
	/// Blend textures adding colors
	Additive = 1,
	/// Blend textures multiplying colors
	Multiplied = 2,
	/// Blend textures adding colors (alternative)
	AddColors = 3,
	/// Blend textures subtracting colors (alternative)
	SubtractColors = 4,
	/// Blend premultiplied textures considering alpha
	AlphaPremultiply = 5,
	/// Blend textures using custom src/dst factors (use rlSetBlendFactors())
	Custom = 6,
	/// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
	CustomSeparate = 7,
}

/// Gesture
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gesture(pub(crate) u32);

bitflags::bitflags! {
	impl Gesture: u32 {
		/// No gesture
		const NONE = 0;
		/// Tap gesture
		const TAP = 1;
		/// Double tap gesture
		const DOUBLETAP = 2;
		/// Hold gesture
		const HOLD = 4;
		/// Drag gesture
		const DRAG = 8;
		/// Swipe right gesture
		const SWIPE_RIGHT = 16;
		/// Swipe left gesture
		const SWIPE_LEFT = 32;
		/// Swipe up gesture
		const SWIPE_UP = 64;
		/// Swipe down gesture
		const SWIPE_DOWN = 128;
		/// Pinch in gesture
		const PINCH_IN = 256;
		/// Pinch out gesture
		const PINCH_OUT = 512;

		const _ = !0;
	}
}

/// Camera system modes
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CameraMode {
	/// Custom camera
	Custom = 0,
	/// Free camera
	Free = 1,
	/// Orbital camera
	Orbital = 2,
	/// First person camera
	FirstPerson = 3,
	/// Third person camera
	ThirdPerson = 4,
}

/// Camera projection
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CameraProjection {
	/// Perspective projection
	Perspective = 0,
	/// Orthographic projection
	Orthographic = 1,
}

/// N-patch layout
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NPatchLayout {
	/// Npatch layout: 3x3 tiles
	NinePatch = 0,
	/// Npatch layout: 1x3 tiles
	ThreePatchVertical = 1,
	/// Npatch layout: 3x1 tiles
	ThreePatchHorizontal = 2,
}
/// Logging: Redirect trace log messages
pub type TraceLogCallback = Option<unsafe extern "C" fn(logLevel: core::ffi::c_int, text: *const core::ffi::c_char, args: *mut core::ffi::c_void, )>;
/// FileIO: Load binary data
pub type LoadFileDataCallback = Option<unsafe extern "C" fn(fileName: *const core::ffi::c_char, bytesRead: *mut core::ffi::c_uint, ) -> *mut core::ffi::c_uchar>;
/// FileIO: Save binary data
pub type SaveFileDataCallback = Option<unsafe extern "C" fn(fileName: *const core::ffi::c_char, data: *mut core::ffi::c_void, bytesToWrite: core::ffi::c_uint, ) -> bool>;
/// FileIO: Load text data
pub type LoadFileTextCallback = Option<unsafe extern "C" fn(fileName: *const core::ffi::c_char, ) -> *mut core::ffi::c_char>;
/// FileIO: Save text data
pub type SaveFileTextCallback = Option<unsafe extern "C" fn(fileName: *const core::ffi::c_char, text: *mut core::ffi::c_char, ) -> bool>;
/// 
pub type AudioCallback = Option<unsafe extern "C" fn(bufferData: *mut core::ffi::c_void, frames: core::ffi::c_uint, )>;

extern "C" {
	/// Initialize window and OpenGL context
	pub fn InitWindow(width: core::ffi::c_int, height: core::ffi::c_int, title: *const core::ffi::c_char, );
	/// Check if KEY_ESCAPE pressed or Close icon pressed
	pub fn WindowShouldClose() -> bool;
	/// Close window and unload OpenGL context
	pub fn CloseWindow();
	/// Check if window has been initialized successfully
	pub fn IsWindowReady() -> bool;
	/// Check if window is currently fullscreen
	pub fn IsWindowFullscreen() -> bool;
	/// Check if window is currently hidden (only PLATFORM_DESKTOP)
	pub fn IsWindowHidden() -> bool;
	/// Check if window is currently minimized (only PLATFORM_DESKTOP)
	pub fn IsWindowMinimized() -> bool;
	/// Check if window is currently maximized (only PLATFORM_DESKTOP)
	pub fn IsWindowMaximized() -> bool;
	/// Check if window is currently focused (only PLATFORM_DESKTOP)
	pub fn IsWindowFocused() -> bool;
	/// Check if window has been resized last frame
	pub fn IsWindowResized() -> bool;
	/// Check if one specific window flag is enabled
	pub fn IsWindowState(flag: core::ffi::c_uint, ) -> bool;
	/// Set window configuration state using flags (only PLATFORM_DESKTOP)
	pub fn SetWindowState(flags: core::ffi::c_uint, );
	/// Clear window configuration state flags
	pub fn ClearWindowState(flags: core::ffi::c_uint, );
	/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
	pub fn ToggleFullscreen();
	/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
	pub fn MaximizeWindow();
	/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
	pub fn MinimizeWindow();
	/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
	pub fn RestoreWindow();
	/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
	pub fn SetWindowIcon(image: Image, );
	/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
	pub fn SetWindowIcons(images: *mut Image, count: core::ffi::c_int, );
	/// Set title for window (only PLATFORM_DESKTOP)
	pub fn SetWindowTitle(title: *const core::ffi::c_char, );
	/// Set window position on screen (only PLATFORM_DESKTOP)
	pub fn SetWindowPosition(x: core::ffi::c_int, y: core::ffi::c_int, );
	/// Set monitor for the current window (fullscreen mode)
	pub fn SetWindowMonitor(monitor: core::ffi::c_int, );
	/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
	pub fn SetWindowMinSize(width: core::ffi::c_int, height: core::ffi::c_int, );
	/// Set window dimensions
	pub fn SetWindowSize(width: core::ffi::c_int, height: core::ffi::c_int, );
	/// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
	pub fn SetWindowOpacity(opacity: core::ffi::c_float, );
	/// Get native window handle
	pub fn GetWindowHandle() -> *mut core::ffi::c_void;
	/// Get current screen width
	pub fn GetScreenWidth() -> core::ffi::c_int;
	/// Get current screen height
	pub fn GetScreenHeight() -> core::ffi::c_int;
	/// Get current render width (it considers HiDPI)
	pub fn GetRenderWidth() -> core::ffi::c_int;
	/// Get current render height (it considers HiDPI)
	pub fn GetRenderHeight() -> core::ffi::c_int;
	/// Get number of connected monitors
	pub fn GetMonitorCount() -> core::ffi::c_int;
	/// Get current connected monitor
	pub fn GetCurrentMonitor() -> core::ffi::c_int;
	/// Get specified monitor position
	pub fn GetMonitorPosition(monitor: core::ffi::c_int, ) -> Vector2;
	/// Get specified monitor width (current video mode used by monitor)
	pub fn GetMonitorWidth(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get specified monitor height (current video mode used by monitor)
	pub fn GetMonitorHeight(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get specified monitor physical width in millimetres
	pub fn GetMonitorPhysicalWidth(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get specified monitor physical height in millimetres
	pub fn GetMonitorPhysicalHeight(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get specified monitor refresh rate
	pub fn GetMonitorRefreshRate(monitor: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get window position XY on monitor
	pub fn GetWindowPosition() -> Vector2;
	/// Get window scale DPI factor
	pub fn GetWindowScaleDPI() -> Vector2;
	/// Get the human-readable, UTF-8 encoded name of the primary monitor
	pub fn GetMonitorName(monitor: core::ffi::c_int, ) -> *const core::ffi::c_char;
	/// Set clipboard text content
	pub fn SetClipboardText(text: *const core::ffi::c_char, );
	/// Get clipboard text content
	pub fn GetClipboardText() -> *const core::ffi::c_char;
	/// Enable waiting for events on EndDrawing(), no automatic event polling
	pub fn EnableEventWaiting();
	/// Disable waiting for events on EndDrawing(), automatic events polling
	pub fn DisableEventWaiting();
	/// Swap back buffer with front buffer (screen drawing)
	pub fn SwapScreenBuffer();
	/// Register all input events
	pub fn PollInputEvents();
	/// Wait for some time (halt program execution)
	pub fn WaitTime(seconds: core::ffi::c_double, );
	/// Shows cursor
	pub fn ShowCursor();
	/// Hides cursor
	pub fn HideCursor();
	/// Check if cursor is not visible
	pub fn IsCursorHidden() -> bool;
	/// Enables cursor (unlock cursor)
	pub fn EnableCursor();
	/// Disables cursor (lock cursor)
	pub fn DisableCursor();
	/// Check if cursor is on the screen
	pub fn IsCursorOnScreen() -> bool;
	/// Set background color (framebuffer clear color)
	pub fn ClearBackground(color: Color, );
	/// Setup canvas (framebuffer) to start drawing
	pub fn BeginDrawing();
	/// End canvas drawing and swap buffers (double buffering)
	pub fn EndDrawing();
	/// Begin 2D mode with custom camera (2D)
	pub fn BeginMode2D(camera: Camera2D, );
	/// Ends 2D mode with custom camera
	pub fn EndMode2D();
	/// Begin 3D mode with custom camera (3D)
	pub fn BeginMode3D(camera: Camera3D, );
	/// Ends 3D mode and returns to default 2D orthographic mode
	pub fn EndMode3D();
	/// Begin drawing to render texture
	pub fn BeginTextureMode(target: RenderTexture2D, );
	/// Ends drawing to render texture
	pub fn EndTextureMode();
	/// Begin custom shader drawing
	pub fn BeginShaderMode(shader: Shader, );
	/// End custom shader drawing (use default shader)
	pub fn EndShaderMode();
	/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
	pub fn BeginBlendMode(mode: core::ffi::c_int, );
	/// End blending mode (reset to default: alpha blending)
	pub fn EndBlendMode();
	/// Begin scissor mode (define screen area for following drawing)
	pub fn BeginScissorMode(x: core::ffi::c_int, y: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, );
	/// End scissor mode
	pub fn EndScissorMode();
	/// Begin stereo rendering (requires VR simulator)
	pub fn BeginVrStereoMode(config: VrStereoConfig, );
	/// End stereo rendering (requires VR simulator)
	pub fn EndVrStereoMode();
	/// Load VR stereo config for VR simulator device parameters
	pub fn LoadVrStereoConfig(device: VrDeviceInfo, ) -> VrStereoConfig;
	/// Unload VR stereo config
	pub fn UnloadVrStereoConfig(config: VrStereoConfig, );
	/// Load shader from files and bind default locations
	pub fn LoadShader(vsFileName: *const core::ffi::c_char, fsFileName: *const core::ffi::c_char, ) -> Shader;
	/// Load shader from code strings and bind default locations
	pub fn LoadShaderFromMemory(vsCode: *const core::ffi::c_char, fsCode: *const core::ffi::c_char, ) -> Shader;
	/// Check if a shader is ready
	pub fn IsShaderReady(shader: Shader, ) -> bool;
	/// Get shader uniform location
	pub fn GetShaderLocation(shader: Shader, uniformName: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Get shader attribute location
	pub fn GetShaderLocationAttrib(shader: Shader, attribName: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Set shader uniform value
	pub fn SetShaderValue(shader: Shader, locIndex: core::ffi::c_int, value: *const core::ffi::c_void, uniformType: core::ffi::c_int, );
	/// Set shader uniform value vector
	pub fn SetShaderValueV(shader: Shader, locIndex: core::ffi::c_int, value: *const core::ffi::c_void, uniformType: core::ffi::c_int, count: core::ffi::c_int, );
	/// Set shader uniform value (matrix 4x4)
	pub fn SetShaderValueMatrix(shader: Shader, locIndex: core::ffi::c_int, mat: Matrix, );
	/// Set shader uniform value for texture (sampler2d)
	pub fn SetShaderValueTexture(shader: Shader, locIndex: core::ffi::c_int, texture: Texture2D, );
	/// Unload shader from GPU memory (VRAM)
	pub fn UnloadShader(shader: Shader, );
	/// Get a ray trace from mouse position
	pub fn GetMouseRay(mousePosition: Vector2, camera: Camera, ) -> Ray;
	/// Get camera transform matrix (view matrix)
	pub fn GetCameraMatrix(camera: Camera, ) -> Matrix;
	/// Get camera 2d transform matrix
	pub fn GetCameraMatrix2D(camera: Camera2D, ) -> Matrix;
	/// Get the screen space position for a 3d world space position
	pub fn GetWorldToScreen(position: Vector3, camera: Camera, ) -> Vector2;
	/// Get the world space position for a 2d camera screen space position
	pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D, ) -> Vector2;
	/// Get size position for a 3d world space position
	pub fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: core::ffi::c_int, height: core::ffi::c_int, ) -> Vector2;
	/// Get the screen space position for a 2d camera world space position
	pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D, ) -> Vector2;
	/// Set target FPS (maximum)
	pub fn SetTargetFPS(fps: core::ffi::c_int, );
	/// Get current FPS
	pub fn GetFPS() -> core::ffi::c_int;
	/// Get time in seconds for last frame drawn (delta time)
	pub fn GetFrameTime() -> core::ffi::c_float;
	/// Get elapsed time in seconds since InitWindow()
	pub fn GetTime() -> core::ffi::c_double;
	/// Get a random value between min and max (both included)
	pub fn GetRandomValue(min: core::ffi::c_int, max: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Set the seed for the random number generator
	pub fn SetRandomSeed(seed: core::ffi::c_uint, );
	/// Takes a screenshot of current screen (filename extension defines format)
	pub fn TakeScreenshot(fileName: *const core::ffi::c_char, );
	/// Setup init configuration flags (view FLAGS)
	pub fn SetConfigFlags(flags: core::ffi::c_uint, );
	/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
	pub fn TraceLog(logLevel: core::ffi::c_int, text: *const core::ffi::c_char, ..., );
	/// Set the current threshold (minimum) log level
	pub fn SetTraceLogLevel(logLevel: core::ffi::c_int, );
	/// Internal memory allocator
	pub fn MemAlloc(size: core::ffi::c_uint, ) -> *mut core::ffi::c_void;
	/// Internal memory reallocator
	pub fn MemRealloc(ptr: *mut core::ffi::c_void, size: core::ffi::c_uint, ) -> *mut core::ffi::c_void;
	/// Internal memory free
	pub fn MemFree(ptr: *mut core::ffi::c_void, );
	/// Open URL with default system browser (if available)
	pub fn OpenURL(url: *const core::ffi::c_char, );
	/// Set custom trace log
	pub fn SetTraceLogCallback(callback: TraceLogCallback, );
	/// Set custom file binary data loader
	pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback, );
	/// Set custom file binary data saver
	pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback, );
	/// Set custom file text data loader
	pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback, );
	/// Set custom file text data saver
	pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback, );
	/// Load file data as byte array (read)
	pub fn LoadFileData(fileName: *const core::ffi::c_char, bytesRead: *mut core::ffi::c_uint, ) -> *mut core::ffi::c_uchar;
	/// Unload file data allocated by LoadFileData()
	pub fn UnloadFileData(data: *mut core::ffi::c_uchar, );
	/// Save data to file from byte array (write), returns true on success
	pub fn SaveFileData(fileName: *const core::ffi::c_char, data: *mut core::ffi::c_void, bytesToWrite: core::ffi::c_uint, ) -> bool;
	/// Export data to code (.h), returns true on success
	pub fn ExportDataAsCode(data: *const core::ffi::c_uchar, size: core::ffi::c_uint, fileName: *const core::ffi::c_char, ) -> bool;
	/// Load text data from file (read), returns a '\0' terminated string
	pub fn LoadFileText(fileName: *const core::ffi::c_char, ) -> *mut core::ffi::c_char;
	/// Unload file text data allocated by LoadFileText()
	pub fn UnloadFileText(text: *mut core::ffi::c_char, );
	/// Save text data to file (write), string must be '\0' terminated, returns true on success
	pub fn SaveFileText(fileName: *const core::ffi::c_char, text: *mut core::ffi::c_char, ) -> bool;
	/// Check if file exists
	pub fn FileExists(fileName: *const core::ffi::c_char, ) -> bool;
	/// Check if a directory path exists
	pub fn DirectoryExists(dirPath: *const core::ffi::c_char, ) -> bool;
	/// Check file extension (including point: .png, .wav)
	pub fn IsFileExtension(fileName: *const core::ffi::c_char, ext: *const core::ffi::c_char, ) -> bool;
	/// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
	pub fn GetFileLength(fileName: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Get pointer to extension for a filename string (includes dot: '.png')
	pub fn GetFileExtension(fileName: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get pointer to filename for a path string
	pub fn GetFileName(filePath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get filename string without extension (uses static string)
	pub fn GetFileNameWithoutExt(filePath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get full path for a given fileName with path (uses static string)
	pub fn GetDirectoryPath(filePath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get previous directory path for a given path (uses static string)
	pub fn GetPrevDirectoryPath(dirPath: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get current working directory (uses static string)
	pub fn GetWorkingDirectory() -> *const core::ffi::c_char;
	/// Get the directory if the running application (uses static string)
	pub fn GetApplicationDirectory() -> *const core::ffi::c_char;
	/// Change working directory, return true on success
	pub fn ChangeDirectory(dir: *const core::ffi::c_char, ) -> bool;
	/// Check if a given path is a file or a directory
	pub fn IsPathFile(path: *const core::ffi::c_char, ) -> bool;
	/// Load directory filepaths
	pub fn LoadDirectoryFiles(dirPath: *const core::ffi::c_char, ) -> FilePathList;
	/// Load directory filepaths with extension filtering and recursive directory scan
	pub fn LoadDirectoryFilesEx(basePath: *const core::ffi::c_char, filter: *const core::ffi::c_char, scanSubdirs: bool, ) -> FilePathList;
	/// Unload filepaths
	pub fn UnloadDirectoryFiles(files: FilePathList, );
	/// Check if a file has been dropped into window
	pub fn IsFileDropped() -> bool;
	/// Load dropped filepaths
	pub fn LoadDroppedFiles() -> FilePathList;
	/// Unload dropped filepaths
	pub fn UnloadDroppedFiles(files: FilePathList, );
	/// Get file modification time (last write time)
	pub fn GetFileModTime(fileName: *const core::ffi::c_char, ) -> core::ffi::c_long;
	/// Compress data (DEFLATE algorithm), memory must be MemFree()
	pub fn CompressData(data: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, compDataSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_uchar;
	/// Decompress data (DEFLATE algorithm), memory must be MemFree()
	pub fn DecompressData(compData: *const core::ffi::c_uchar, compDataSize: core::ffi::c_int, dataSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_uchar;
	/// Encode data to Base64 string, memory must be MemFree()
	pub fn EncodeDataBase64(data: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, outputSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_char;
	/// Decode Base64 string data, memory must be MemFree()
	pub fn DecodeDataBase64(data: *const core::ffi::c_uchar, outputSize: *mut core::ffi::c_int, ) -> *mut core::ffi::c_uchar;
	/// Check if a key has been pressed once
	pub fn IsKeyPressed(key: core::ffi::c_int, ) -> bool;
	/// Check if a key is being pressed
	pub fn IsKeyDown(key: core::ffi::c_int, ) -> bool;
	/// Check if a key has been released once
	pub fn IsKeyReleased(key: core::ffi::c_int, ) -> bool;
	/// Check if a key is NOT being pressed
	pub fn IsKeyUp(key: core::ffi::c_int, ) -> bool;
	/// Set a custom key to exit program (default is ESC)
	pub fn SetExitKey(key: core::ffi::c_int, );
	/// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
	pub fn GetKeyPressed() -> core::ffi::c_int;
	/// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
	pub fn GetCharPressed() -> core::ffi::c_int;
	/// Check if a gamepad is available
	pub fn IsGamepadAvailable(gamepad: core::ffi::c_int, ) -> bool;
	/// Get gamepad internal name id
	pub fn GetGamepadName(gamepad: core::ffi::c_int, ) -> *const core::ffi::c_char;
	/// Check if a gamepad button has been pressed once
	pub fn IsGamepadButtonPressed(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
	/// Check if a gamepad button is being pressed
	pub fn IsGamepadButtonDown(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
	/// Check if a gamepad button has been released once
	pub fn IsGamepadButtonReleased(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
	/// Check if a gamepad button is NOT being pressed
	pub fn IsGamepadButtonUp(gamepad: core::ffi::c_int, button: core::ffi::c_int, ) -> bool;
	/// Get the last gamepad button pressed
	pub fn GetGamepadButtonPressed() -> core::ffi::c_int;
	/// Get gamepad axis count for a gamepad
	pub fn GetGamepadAxisCount(gamepad: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get axis movement value for a gamepad axis
	pub fn GetGamepadAxisMovement(gamepad: core::ffi::c_int, axis: core::ffi::c_int, ) -> core::ffi::c_float;
	/// Set internal gamepad mappings (SDL_GameControllerDB)
	pub fn SetGamepadMappings(mappings: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Check if a mouse button has been pressed once
	pub fn IsMouseButtonPressed(button: core::ffi::c_int, ) -> bool;
	/// Check if a mouse button is being pressed
	pub fn IsMouseButtonDown(button: core::ffi::c_int, ) -> bool;
	/// Check if a mouse button has been released once
	pub fn IsMouseButtonReleased(button: core::ffi::c_int, ) -> bool;
	/// Check if a mouse button is NOT being pressed
	pub fn IsMouseButtonUp(button: core::ffi::c_int, ) -> bool;
	/// Get mouse position X
	pub fn GetMouseX() -> core::ffi::c_int;
	/// Get mouse position Y
	pub fn GetMouseY() -> core::ffi::c_int;
	/// Get mouse position XY
	pub fn GetMousePosition() -> Vector2;
	/// Get mouse delta between frames
	pub fn GetMouseDelta() -> Vector2;
	/// Set mouse position XY
	pub fn SetMousePosition(x: core::ffi::c_int, y: core::ffi::c_int, );
	/// Set mouse offset
	pub fn SetMouseOffset(offsetX: core::ffi::c_int, offsetY: core::ffi::c_int, );
	/// Set mouse scaling
	pub fn SetMouseScale(scaleX: core::ffi::c_float, scaleY: core::ffi::c_float, );
	/// Get mouse wheel movement for X or Y, whichever is larger
	pub fn GetMouseWheelMove() -> core::ffi::c_float;
	/// Get mouse wheel movement for both X and Y
	pub fn GetMouseWheelMoveV() -> Vector2;
	/// Set mouse cursor
	pub fn SetMouseCursor(cursor: core::ffi::c_int, );
	/// Get touch position X for touch point 0 (relative to screen size)
	pub fn GetTouchX() -> core::ffi::c_int;
	/// Get touch position Y for touch point 0 (relative to screen size)
	pub fn GetTouchY() -> core::ffi::c_int;
	/// Get touch position XY for a touch point index (relative to screen size)
	pub fn GetTouchPosition(index: core::ffi::c_int, ) -> Vector2;
	/// Get touch point identifier for given index
	pub fn GetTouchPointId(index: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get number of touch points
	pub fn GetTouchPointCount() -> core::ffi::c_int;
	/// Enable a set of gestures using flags
	pub fn SetGesturesEnabled(flags: core::ffi::c_uint, );
	/// Check if a gesture have been detected
	pub fn IsGestureDetected(gesture: core::ffi::c_int, ) -> bool;
	/// Get latest detected gesture
	pub fn GetGestureDetected() -> core::ffi::c_int;
	/// Get gesture hold time in milliseconds
	pub fn GetGestureHoldDuration() -> core::ffi::c_float;
	/// Get gesture drag vector
	pub fn GetGestureDragVector() -> Vector2;
	/// Get gesture drag angle
	pub fn GetGestureDragAngle() -> core::ffi::c_float;
	/// Get gesture pinch delta
	pub fn GetGesturePinchVector() -> Vector2;
	/// Get gesture pinch angle
	pub fn GetGesturePinchAngle() -> core::ffi::c_float;
	/// Update camera position for selected mode
	pub fn UpdateCamera(camera: *mut Camera, mode: core::ffi::c_int, );
	/// Update camera movement/rotation
	pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: core::ffi::c_float, );
	/// Set texture and rectangle to be used on shapes drawing
	pub fn SetShapesTexture(texture: Texture2D, source: Rectangle, );
	/// Draw a pixel
	pub fn DrawPixel(posX: core::ffi::c_int, posY: core::ffi::c_int, color: Color, );
	/// Draw a pixel (Vector version)
	pub fn DrawPixelV(position: Vector2, color: Color, );
	/// Draw a line
	pub fn DrawLine(startPosX: core::ffi::c_int, startPosY: core::ffi::c_int, endPosX: core::ffi::c_int, endPosY: core::ffi::c_int, color: Color, );
	/// Draw a line (Vector version)
	pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color, );
	/// Draw a line defining thickness
	pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: core::ffi::c_float, color: Color, );
	/// Draw a line using cubic-bezier curves in-out
	pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: core::ffi::c_float, color: Color, );
	/// Draw line using quadratic bezier curves with a control point
	pub fn DrawLineBezierQuad(startPos: Vector2, endPos: Vector2, controlPos: Vector2, thick: core::ffi::c_float, color: Color, );
	/// Draw line using cubic bezier curves with 2 control points
	pub fn DrawLineBezierCubic(startPos: Vector2, endPos: Vector2, startControlPos: Vector2, endControlPos: Vector2, thick: core::ffi::c_float, color: Color, );
	/// Draw lines sequence
	pub fn DrawLineStrip(points: *mut Vector2, pointCount: core::ffi::c_int, color: Color, );
	/// Draw a color-filled circle
	pub fn DrawCircle(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_float, color: Color, );
	/// Draw a piece of a circle
	pub fn DrawCircleSector(center: Vector2, radius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color, );
	/// Draw circle sector outline
	pub fn DrawCircleSectorLines(center: Vector2, radius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color, );
	/// Draw a gradient-filled circle
	pub fn DrawCircleGradient(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_float, color1: Color, color2: Color, );
	/// Draw a color-filled circle (Vector version)
	pub fn DrawCircleV(center: Vector2, radius: core::ffi::c_float, color: Color, );
	/// Draw circle outline
	pub fn DrawCircleLines(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_float, color: Color, );
	/// Draw ellipse
	pub fn DrawEllipse(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radiusH: core::ffi::c_float, radiusV: core::ffi::c_float, color: Color, );
	/// Draw ellipse outline
	pub fn DrawEllipseLines(centerX: core::ffi::c_int, centerY: core::ffi::c_int, radiusH: core::ffi::c_float, radiusV: core::ffi::c_float, color: Color, );
	/// Draw ring
	pub fn DrawRing(center: Vector2, innerRadius: core::ffi::c_float, outerRadius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color, );
	/// Draw ring outline
	pub fn DrawRingLines(center: Vector2, innerRadius: core::ffi::c_float, outerRadius: core::ffi::c_float, startAngle: core::ffi::c_float, endAngle: core::ffi::c_float, segments: core::ffi::c_int, color: Color, );
	/// Draw a color-filled rectangle
	pub fn DrawRectangle(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color: Color, );
	/// Draw a color-filled rectangle (Vector version)
	pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color, );
	/// Draw a color-filled rectangle
	pub fn DrawRectangleRec(rec: Rectangle, color: Color, );
	/// Draw a color-filled rectangle with pro parameters
	pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: core::ffi::c_float, color: Color, );
	/// Draw a vertical-gradient-filled rectangle
	pub fn DrawRectangleGradientV(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color1: Color, color2: Color, );
	/// Draw a horizontal-gradient-filled rectangle
	pub fn DrawRectangleGradientH(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color1: Color, color2: Color, );
	/// Draw a gradient-filled rectangle with custom vertex colors
	pub fn DrawRectangleGradientEx(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color, );
	/// Draw rectangle outline
	pub fn DrawRectangleLines(posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color: Color, );
	/// Draw rectangle outline with extended parameters
	pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: core::ffi::c_float, color: Color, );
	/// Draw rectangle with rounded edges
	pub fn DrawRectangleRounded(rec: Rectangle, roundness: core::ffi::c_float, segments: core::ffi::c_int, color: Color, );
	/// Draw rectangle with rounded edges outline
	pub fn DrawRectangleRoundedLines(rec: Rectangle, roundness: core::ffi::c_float, segments: core::ffi::c_int, lineThick: core::ffi::c_float, color: Color, );
	/// Draw a color-filled triangle (vertex in counter-clockwise order!)
	pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color, );
	/// Draw triangle outline (vertex in counter-clockwise order!)
	pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color, );
	/// Draw a triangle fan defined by points (first vertex is the center)
	pub fn DrawTriangleFan(points: *mut Vector2, pointCount: core::ffi::c_int, color: Color, );
	/// Draw a triangle strip defined by points
	pub fn DrawTriangleStrip(points: *mut Vector2, pointCount: core::ffi::c_int, color: Color, );
	/// Draw a regular polygon (Vector version)
	pub fn DrawPoly(center: Vector2, sides: core::ffi::c_int, radius: core::ffi::c_float, rotation: core::ffi::c_float, color: Color, );
	/// Draw a polygon outline of n sides
	pub fn DrawPolyLines(center: Vector2, sides: core::ffi::c_int, radius: core::ffi::c_float, rotation: core::ffi::c_float, color: Color, );
	/// Draw a polygon outline of n sides with extended parameters
	pub fn DrawPolyLinesEx(center: Vector2, sides: core::ffi::c_int, radius: core::ffi::c_float, rotation: core::ffi::c_float, lineThick: core::ffi::c_float, color: Color, );
	/// Check collision between two rectangles
	pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle, ) -> bool;
	/// Check collision between two circles
	pub fn CheckCollisionCircles(center1: Vector2, radius1: core::ffi::c_float, center2: Vector2, radius2: core::ffi::c_float, ) -> bool;
	/// Check collision between circle and rectangle
	pub fn CheckCollisionCircleRec(center: Vector2, radius: core::ffi::c_float, rec: Rectangle, ) -> bool;
	/// Check if point is inside rectangle
	pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle, ) -> bool;
	/// Check if point is inside circle
	pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: core::ffi::c_float, ) -> bool;
	/// Check if point is inside a triangle
	pub fn CheckCollisionPointTriangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2, ) -> bool;
	/// Check if point is within a polygon described by array of vertices
	pub fn CheckCollisionPointPoly(point: Vector2, points: *mut Vector2, pointCount: core::ffi::c_int, ) -> bool;
	/// Check the collision between two lines defined by two points each, returns collision point by reference
	pub fn CheckCollisionLines(startPos1: Vector2, endPos1: Vector2, startPos2: Vector2, endPos2: Vector2, collisionPoint: *mut Vector2, ) -> bool;
	/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
	pub fn CheckCollisionPointLine(point: Vector2, p1: Vector2, p2: Vector2, threshold: core::ffi::c_int, ) -> bool;
	/// Get collision rectangle for two rectangles collision
	pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle, ) -> Rectangle;
	/// Load image from file into CPU memory (RAM)
	pub fn LoadImage(fileName: *const core::ffi::c_char, ) -> Image;
	/// Load image from RAW file data
	pub fn LoadImageRaw(fileName: *const core::ffi::c_char, width: core::ffi::c_int, height: core::ffi::c_int, format: core::ffi::c_int, headerSize: core::ffi::c_int, ) -> Image;
	/// Load image sequence from file (frames appended to image.data)
	pub fn LoadImageAnim(fileName: *const core::ffi::c_char, frames: *mut core::ffi::c_int, ) -> Image;
	/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
	pub fn LoadImageFromMemory(fileType: *const core::ffi::c_char, fileData: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, ) -> Image;
	/// Load image from GPU texture data
	pub fn LoadImageFromTexture(texture: Texture2D, ) -> Image;
	/// Load image from screen buffer and (screenshot)
	pub fn LoadImageFromScreen() -> Image;
	/// Check if an image is ready
	pub fn IsImageReady(image: Image, ) -> bool;
	/// Unload image from CPU memory (RAM)
	pub fn UnloadImage(image: Image, );
	/// Export image data to file, returns true on success
	pub fn ExportImage(image: Image, fileName: *const core::ffi::c_char, ) -> bool;
	/// Export image as code file defining an array of bytes, returns true on success
	pub fn ExportImageAsCode(image: Image, fileName: *const core::ffi::c_char, ) -> bool;
	/// Generate image: plain color
	pub fn GenImageColor(width: core::ffi::c_int, height: core::ffi::c_int, color: Color, ) -> Image;
	/// Generate image: vertical gradient
	pub fn GenImageGradientV(width: core::ffi::c_int, height: core::ffi::c_int, top: Color, bottom: Color, ) -> Image;
	/// Generate image: horizontal gradient
	pub fn GenImageGradientH(width: core::ffi::c_int, height: core::ffi::c_int, left: Color, right: Color, ) -> Image;
	/// Generate image: radial gradient
	pub fn GenImageGradientRadial(width: core::ffi::c_int, height: core::ffi::c_int, density: core::ffi::c_float, inner: Color, outer: Color, ) -> Image;
	/// Generate image: checked
	pub fn GenImageChecked(width: core::ffi::c_int, height: core::ffi::c_int, checksX: core::ffi::c_int, checksY: core::ffi::c_int, col1: Color, col2: Color, ) -> Image;
	/// Generate image: white noise
	pub fn GenImageWhiteNoise(width: core::ffi::c_int, height: core::ffi::c_int, factor: core::ffi::c_float, ) -> Image;
	/// Generate image: perlin noise
	pub fn GenImagePerlinNoise(width: core::ffi::c_int, height: core::ffi::c_int, offsetX: core::ffi::c_int, offsetY: core::ffi::c_int, scale: core::ffi::c_float, ) -> Image;
	/// Generate image: cellular algorithm, bigger tileSize means bigger cells
	pub fn GenImageCellular(width: core::ffi::c_int, height: core::ffi::c_int, tileSize: core::ffi::c_int, ) -> Image;
	/// Generate image: grayscale image from text data
	pub fn GenImageText(width: core::ffi::c_int, height: core::ffi::c_int, text: *const core::ffi::c_char, ) -> Image;
	/// Create an image duplicate (useful for transformations)
	pub fn ImageCopy(image: Image, ) -> Image;
	/// Create an image from another image piece
	pub fn ImageFromImage(image: Image, rec: Rectangle, ) -> Image;
	/// Create an image from text (default font)
	pub fn ImageText(text: *const core::ffi::c_char, fontSize: core::ffi::c_int, color: Color, ) -> Image;
	/// Create an image from text (custom sprite font)
	pub fn ImageTextEx(font: Font, text: *const core::ffi::c_char, fontSize: core::ffi::c_float, spacing: core::ffi::c_float, tint: Color, ) -> Image;
	/// Convert image data to desired format
	pub fn ImageFormat(image: *mut Image, newFormat: core::ffi::c_int, );
	/// Convert image to POT (power-of-two)
	pub fn ImageToPOT(image: *mut Image, fill: Color, );
	/// Crop an image to a defined rectangle
	pub fn ImageCrop(image: *mut Image, crop: Rectangle, );
	/// Crop image depending on alpha value
	pub fn ImageAlphaCrop(image: *mut Image, threshold: core::ffi::c_float, );
	/// Clear alpha channel to desired color
	pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: core::ffi::c_float, );
	/// Apply alpha mask to image
	pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image, );
	/// Premultiply alpha channel
	pub fn ImageAlphaPremultiply(image: *mut Image, );
	/// Apply Gaussian blur using a box blur approximation
	pub fn ImageBlurGaussian(image: *mut Image, blurSize: core::ffi::c_int, );
	/// Resize image (Bicubic scaling algorithm)
	pub fn ImageResize(image: *mut Image, newWidth: core::ffi::c_int, newHeight: core::ffi::c_int, );
	/// Resize image (Nearest-Neighbor scaling algorithm)
	pub fn ImageResizeNN(image: *mut Image, newWidth: core::ffi::c_int, newHeight: core::ffi::c_int, );
	/// Resize canvas and fill with color
	pub fn ImageResizeCanvas(image: *mut Image, newWidth: core::ffi::c_int, newHeight: core::ffi::c_int, offsetX: core::ffi::c_int, offsetY: core::ffi::c_int, fill: Color, );
	/// Compute all mipmap levels for a provided image
	pub fn ImageMipmaps(image: *mut Image, );
	/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
	pub fn ImageDither(image: *mut Image, rBpp: core::ffi::c_int, gBpp: core::ffi::c_int, bBpp: core::ffi::c_int, aBpp: core::ffi::c_int, );
	/// Flip image vertically
	pub fn ImageFlipVertical(image: *mut Image, );
	/// Flip image horizontally
	pub fn ImageFlipHorizontal(image: *mut Image, );
	/// Rotate image clockwise 90deg
	pub fn ImageRotateCW(image: *mut Image, );
	/// Rotate image counter-clockwise 90deg
	pub fn ImageRotateCCW(image: *mut Image, );
	/// Modify image color: tint
	pub fn ImageColorTint(image: *mut Image, color: Color, );
	/// Modify image color: invert
	pub fn ImageColorInvert(image: *mut Image, );
	/// Modify image color: grayscale
	pub fn ImageColorGrayscale(image: *mut Image, );
	/// Modify image color: contrast (-100 to 100)
	pub fn ImageColorContrast(image: *mut Image, contrast: core::ffi::c_float, );
	/// Modify image color: brightness (-255 to 255)
	pub fn ImageColorBrightness(image: *mut Image, brightness: core::ffi::c_int, );
	/// Modify image color: replace color
	pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color, );
	/// Load color data from image as a Color array (RGBA - 32bit)
	pub fn LoadImageColors(image: Image, ) -> *mut Color;
	/// Load colors palette from image as a Color array (RGBA - 32bit)
	pub fn LoadImagePalette(image: Image, maxPaletteSize: core::ffi::c_int, colorCount: *mut core::ffi::c_int, ) -> *mut Color;
	/// Unload color data loaded with LoadImageColors()
	pub fn UnloadImageColors(colors: *mut Color, );
	/// Unload colors palette loaded with LoadImagePalette()
	pub fn UnloadImagePalette(colors: *mut Color, );
	/// Get image alpha border rectangle
	pub fn GetImageAlphaBorder(image: Image, threshold: core::ffi::c_float, ) -> Rectangle;
	/// Get image pixel color at (x, y) position
	pub fn GetImageColor(image: Image, x: core::ffi::c_int, y: core::ffi::c_int, ) -> Color;
	/// Clear image background with given color
	pub fn ImageClearBackground(dst: *mut Image, color: Color, );
	/// Draw pixel within an image
	pub fn ImageDrawPixel(dst: *mut Image, posX: core::ffi::c_int, posY: core::ffi::c_int, color: Color, );
	/// Draw pixel within an image (Vector version)
	pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color, );
	/// Draw line within an image
	pub fn ImageDrawLine(dst: *mut Image, startPosX: core::ffi::c_int, startPosY: core::ffi::c_int, endPosX: core::ffi::c_int, endPosY: core::ffi::c_int, color: Color, );
	/// Draw line within an image (Vector version)
	pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color, );
	/// Draw a filled circle within an image
	pub fn ImageDrawCircle(dst: *mut Image, centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_int, color: Color, );
	/// Draw a filled circle within an image (Vector version)
	pub fn ImageDrawCircleV(dst: *mut Image, center: Vector2, radius: core::ffi::c_int, color: Color, );
	/// Draw circle outline within an image
	pub fn ImageDrawCircleLines(dst: *mut Image, centerX: core::ffi::c_int, centerY: core::ffi::c_int, radius: core::ffi::c_int, color: Color, );
	/// Draw circle outline within an image (Vector version)
	pub fn ImageDrawCircleLinesV(dst: *mut Image, center: Vector2, radius: core::ffi::c_int, color: Color, );
	/// Draw rectangle within an image
	pub fn ImageDrawRectangle(dst: *mut Image, posX: core::ffi::c_int, posY: core::ffi::c_int, width: core::ffi::c_int, height: core::ffi::c_int, color: Color, );
	/// Draw rectangle within an image (Vector version)
	pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color, );
	/// Draw rectangle within an image
	pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color, );
	/// Draw rectangle lines within an image
	pub fn ImageDrawRectangleLines(dst: *mut Image, rec: Rectangle, thick: core::ffi::c_int, color: Color, );
	/// Draw a source image within a destination image (tint applied to source)
	pub fn ImageDraw(dst: *mut Image, src: Image, srcRec: Rectangle, dstRec: Rectangle, tint: Color, );
	/// Draw text (using default font) within an image (destination)
	pub fn ImageDrawText(dst: *mut Image, text: *const core::ffi::c_char, posX: core::ffi::c_int, posY: core::ffi::c_int, fontSize: core::ffi::c_int, color: Color, );
	/// Draw text (custom sprite font) within an image (destination)
	pub fn ImageDrawTextEx(dst: *mut Image, font: Font, text: *const core::ffi::c_char, position: Vector2, fontSize: core::ffi::c_float, spacing: core::ffi::c_float, tint: Color, );
	/// Load texture from file into GPU memory (VRAM)
	pub fn LoadTexture(fileName: *const core::ffi::c_char, ) -> Texture2D;
	/// Load texture from image data
	pub fn LoadTextureFromImage(image: Image, ) -> Texture2D;
	/// Load cubemap from image, multiple image cubemap layouts supported
	pub fn LoadTextureCubemap(image: Image, layout: core::ffi::c_int, ) -> TextureCubemap;
	/// Load texture for rendering (framebuffer)
	pub fn LoadRenderTexture(width: core::ffi::c_int, height: core::ffi::c_int, ) -> RenderTexture2D;
	/// Check if a texture is ready
	pub fn IsTextureReady(texture: Texture2D, ) -> bool;
	/// Unload texture from GPU memory (VRAM)
	pub fn UnloadTexture(texture: Texture2D, );
	/// Check if a render texture is ready
	pub fn IsRenderTextureReady(target: RenderTexture2D, ) -> bool;
	/// Unload render texture from GPU memory (VRAM)
	pub fn UnloadRenderTexture(target: RenderTexture2D, );
	/// Update GPU texture with new data
	pub fn UpdateTexture(texture: Texture2D, pixels: *const core::ffi::c_void, );
	/// Update GPU texture rectangle with new data
	pub fn UpdateTextureRec(texture: Texture2D, rec: Rectangle, pixels: *const core::ffi::c_void, );
	/// Generate GPU mipmaps for a texture
	pub fn GenTextureMipmaps(texture: *mut Texture2D, );
	/// Set texture scaling filter mode
	pub fn SetTextureFilter(texture: Texture2D, filter: core::ffi::c_int, );
	/// Set texture wrapping mode
	pub fn SetTextureWrap(texture: Texture2D, wrap: core::ffi::c_int, );
	/// Draw a Texture2D
	pub fn DrawTexture(texture: Texture2D, posX: core::ffi::c_int, posY: core::ffi::c_int, tint: Color, );
	/// Draw a Texture2D with position defined as Vector2
	pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color, );
	/// Draw a Texture2D with extended parameters
	pub fn DrawTextureEx(texture: Texture2D, position: Vector2, rotation: core::ffi::c_float, scale: core::ffi::c_float, tint: Color, );
	/// Draw a part of a texture defined by a rectangle
	pub fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color, );
	/// Draw a part of a texture defined by a rectangle with 'pro' parameters
	pub fn DrawTexturePro(texture: Texture2D, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: core::ffi::c_float, tint: Color, );
	/// Draws a texture (or part of it) that stretches or shrinks nicely
	pub fn DrawTextureNPatch(texture: Texture2D, nPatchInfo: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: core::ffi::c_float, tint: Color, );
	/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
	pub fn Fade(color: Color, alpha: core::ffi::c_float, ) -> Color;
	/// Get hexadecimal value for a Color
	pub fn ColorToInt(color: Color, ) -> core::ffi::c_int;
	/// Get Color normalized as float [0..1]
	pub fn ColorNormalize(color: Color, ) -> Vector4;
	/// Get Color from normalized values [0..1]
	pub fn ColorFromNormalized(normalized: Vector4, ) -> Color;
	/// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
	pub fn ColorToHSV(color: Color, ) -> Vector3;
	/// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
	pub fn ColorFromHSV(hue: core::ffi::c_float, saturation: core::ffi::c_float, value: core::ffi::c_float, ) -> Color;
	/// Get color multiplied with another color
	pub fn ColorTint(color: Color, tint: Color, ) -> Color;
	/// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
	pub fn ColorBrightness(color: Color, factor: core::ffi::c_float, ) -> Color;
	/// Get color with contrast correction, contrast values between -1.0f and 1.0f
	pub fn ColorContrast(color: Color, contrast: core::ffi::c_float, ) -> Color;
	/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
	pub fn ColorAlpha(color: Color, alpha: core::ffi::c_float, ) -> Color;
	/// Get src alpha-blended into dst color with tint
	pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color, ) -> Color;
	/// Get Color structure from hexadecimal value
	pub fn GetColor(hexValue: core::ffi::c_uint, ) -> Color;
	/// Get Color from a source pixel pointer of certain format
	pub fn GetPixelColor(srcPtr: *mut core::ffi::c_void, format: core::ffi::c_int, ) -> Color;
	/// Set color formatted into destination pixel pointer
	pub fn SetPixelColor(dstPtr: *mut core::ffi::c_void, color: Color, format: core::ffi::c_int, );
	/// Get pixel data size in bytes for certain format
	pub fn GetPixelDataSize(width: core::ffi::c_int, height: core::ffi::c_int, format: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get the default Font
	pub fn GetFontDefault() -> Font;
	/// Load font from file into GPU memory (VRAM)
	pub fn LoadFont(fileName: *const core::ffi::c_char, ) -> Font;
	/// Load font from file with extended parameters, use NULL for fontChars and 0 for glyphCount to load the default character set
	pub fn LoadFontEx(fileName: *const core::ffi::c_char, fontSize: core::ffi::c_int, fontChars: *mut core::ffi::c_int, glyphCount: core::ffi::c_int, ) -> Font;
	/// Load font from Image (XNA style)
	pub fn LoadFontFromImage(image: Image, key: Color, firstChar: core::ffi::c_int, ) -> Font;
	/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
	pub fn LoadFontFromMemory(fileType: *const core::ffi::c_char, fileData: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, fontSize: core::ffi::c_int, fontChars: *mut core::ffi::c_int, glyphCount: core::ffi::c_int, ) -> Font;
	/// Check if a font is ready
	pub fn IsFontReady(font: Font, ) -> bool;
	/// Load font data for further use
	pub fn LoadFontData(fileData: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, fontSize: core::ffi::c_int, fontChars: *mut core::ffi::c_int, glyphCount: core::ffi::c_int, r#type: core::ffi::c_int, ) -> *mut GlyphInfo;
	/// Generate image font atlas using chars info
	pub fn GenImageFontAtlas(chars: *const GlyphInfo, recs: *mut *mut Rectangle, glyphCount: core::ffi::c_int, fontSize: core::ffi::c_int, padding: core::ffi::c_int, packMethod: core::ffi::c_int, ) -> Image;
	/// Unload font chars info data (RAM)
	pub fn UnloadFontData(chars: *mut GlyphInfo, glyphCount: core::ffi::c_int, );
	/// Unload font from GPU memory (VRAM)
	pub fn UnloadFont(font: Font, );
	/// Export font as code file, returns true on success
	pub fn ExportFontAsCode(font: Font, fileName: *const core::ffi::c_char, ) -> bool;
	/// Draw current FPS
	pub fn DrawFPS(posX: core::ffi::c_int, posY: core::ffi::c_int, );
	/// Draw text (using default font)
	pub fn DrawText(text: *const core::ffi::c_char, posX: core::ffi::c_int, posY: core::ffi::c_int, fontSize: core::ffi::c_int, color: Color, );
	/// Draw text using font and additional parameters
	pub fn DrawTextEx(font: Font, text: *const core::ffi::c_char, position: Vector2, fontSize: core::ffi::c_float, spacing: core::ffi::c_float, tint: Color, );
	/// Draw text using Font and pro parameters (rotation)
	pub fn DrawTextPro(font: Font, text: *const core::ffi::c_char, position: Vector2, origin: Vector2, rotation: core::ffi::c_float, fontSize: core::ffi::c_float, spacing: core::ffi::c_float, tint: Color, );
	/// Draw one character (codepoint)
	pub fn DrawTextCodepoint(font: Font, codepoint: core::ffi::c_int, position: Vector2, fontSize: core::ffi::c_float, tint: Color, );
	/// Draw multiple character (codepoint)
	pub fn DrawTextCodepoints(font: Font, codepoints: *const core::ffi::c_int, count: core::ffi::c_int, position: Vector2, fontSize: core::ffi::c_float, spacing: core::ffi::c_float, tint: Color, );
	/// Measure string width for default font
	pub fn MeasureText(text: *const core::ffi::c_char, fontSize: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Measure string size for Font
	pub fn MeasureTextEx(font: Font, text: *const core::ffi::c_char, fontSize: core::ffi::c_float, spacing: core::ffi::c_float, ) -> Vector2;
	/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
	pub fn GetGlyphIndex(font: Font, codepoint: core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
	pub fn GetGlyphInfo(font: Font, codepoint: core::ffi::c_int, ) -> GlyphInfo;
	/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
	pub fn GetGlyphAtlasRec(font: Font, codepoint: core::ffi::c_int, ) -> Rectangle;
	/// Load UTF-8 text encoded from codepoints array
	pub fn LoadUTF8(codepoints: *const core::ffi::c_int, length: core::ffi::c_int, ) -> *mut core::ffi::c_char;
	/// Unload UTF-8 text encoded from codepoints array
	pub fn UnloadUTF8(text: *mut core::ffi::c_char, );
	/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
	pub fn LoadCodepoints(text: *const core::ffi::c_char, count: *mut core::ffi::c_int, ) -> *mut core::ffi::c_int;
	/// Unload codepoints data from memory
	pub fn UnloadCodepoints(codepoints: *mut core::ffi::c_int, );
	/// Get total number of codepoints in a UTF-8 encoded string
	pub fn GetCodepointCount(text: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
	pub fn GetCodepoint(text: *const core::ffi::c_char, codepointSize: *mut core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
	pub fn GetCodepointNext(text: *const core::ffi::c_char, codepointSize: *mut core::ffi::c_int, ) -> core::ffi::c_int;
	/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
	pub fn GetCodepointPrevious(text: *const core::ffi::c_char, codepointSize: *mut core::ffi::c_int, ) -> core::ffi::c_int;
	/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
	pub fn CodepointToUTF8(codepoint: core::ffi::c_int, utf8Size: *mut core::ffi::c_int, ) -> *const core::ffi::c_char;
	/// Copy one string to another, returns bytes copied
	pub fn TextCopy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Check if two text string are equal
	pub fn TextIsEqual(text1: *const core::ffi::c_char, text2: *const core::ffi::c_char, ) -> bool;
	/// Get text length, checks for '\0' ending
	pub fn TextLength(text: *const core::ffi::c_char, ) -> core::ffi::c_uint;
	/// Text formatting with variables (sprintf() style)
	pub fn TextFormat(text: *const core::ffi::c_char, ..., ) -> *const core::ffi::c_char;
	/// Get a piece of a text string
	pub fn TextSubtext(text: *const core::ffi::c_char, position: core::ffi::c_int, length: core::ffi::c_int, ) -> *const core::ffi::c_char;
	/// Replace text string (WARNING: memory must be freed!)
	pub fn TextReplace(text: *mut core::ffi::c_char, replace: *const core::ffi::c_char, by: *const core::ffi::c_char, ) -> *mut core::ffi::c_char;
	/// Insert text in a position (WARNING: memory must be freed!)
	pub fn TextInsert(text: *const core::ffi::c_char, insert: *const core::ffi::c_char, position: core::ffi::c_int, ) -> *mut core::ffi::c_char;
	/// Join text strings with delimiter
	pub fn TextJoin(textList: *const *const core::ffi::c_char, count: core::ffi::c_int, delimiter: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Split text into multiple strings
	pub fn TextSplit(text: *const core::ffi::c_char, delimiter: core::ffi::c_char, count: *mut core::ffi::c_int, ) -> *const *const core::ffi::c_char;
	/// Append text at specific position and move cursor!
	pub fn TextAppend(text: *mut core::ffi::c_char, append: *const core::ffi::c_char, position: *mut core::ffi::c_int, );
	/// Find first text occurrence within a string
	pub fn TextFindIndex(text: *const core::ffi::c_char, find: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Get upper case version of provided string
	pub fn TextToUpper(text: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get lower case version of provided string
	pub fn TextToLower(text: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get Pascal case notation version of provided string
	pub fn TextToPascal(text: *const core::ffi::c_char, ) -> *const core::ffi::c_char;
	/// Get integer value from text (negative values not supported)
	pub fn TextToInteger(text: *const core::ffi::c_char, ) -> core::ffi::c_int;
	/// Draw a line in 3D world space
	pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color, );
	/// Draw a point in 3D space, actually a small line
	pub fn DrawPoint3D(position: Vector3, color: Color, );
	/// Draw a circle in 3D world space
	pub fn DrawCircle3D(center: Vector3, radius: core::ffi::c_float, rotationAxis: Vector3, rotationAngle: core::ffi::c_float, color: Color, );
	/// Draw a color-filled triangle (vertex in counter-clockwise order!)
	pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color, );
	/// Draw a triangle strip defined by points
	pub fn DrawTriangleStrip3D(points: *mut Vector3, pointCount: core::ffi::c_int, color: Color, );
	/// Draw cube
	pub fn DrawCube(position: Vector3, width: core::ffi::c_float, height: core::ffi::c_float, length: core::ffi::c_float, color: Color, );
	/// Draw cube (Vector version)
	pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color, );
	/// Draw cube wires
	pub fn DrawCubeWires(position: Vector3, width: core::ffi::c_float, height: core::ffi::c_float, length: core::ffi::c_float, color: Color, );
	/// Draw cube wires (Vector version)
	pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color, );
	/// Draw sphere
	pub fn DrawSphere(centerPos: Vector3, radius: core::ffi::c_float, color: Color, );
	/// Draw sphere with extended parameters
	pub fn DrawSphereEx(centerPos: Vector3, radius: core::ffi::c_float, rings: core::ffi::c_int, slices: core::ffi::c_int, color: Color, );
	/// Draw sphere wires
	pub fn DrawSphereWires(centerPos: Vector3, radius: core::ffi::c_float, rings: core::ffi::c_int, slices: core::ffi::c_int, color: Color, );
	/// Draw a cylinder/cone
	pub fn DrawCylinder(position: Vector3, radiusTop: core::ffi::c_float, radiusBottom: core::ffi::c_float, height: core::ffi::c_float, slices: core::ffi::c_int, color: Color, );
	/// Draw a cylinder with base at startPos and top at endPos
	pub fn DrawCylinderEx(startPos: Vector3, endPos: Vector3, startRadius: core::ffi::c_float, endRadius: core::ffi::c_float, sides: core::ffi::c_int, color: Color, );
	/// Draw a cylinder/cone wires
	pub fn DrawCylinderWires(position: Vector3, radiusTop: core::ffi::c_float, radiusBottom: core::ffi::c_float, height: core::ffi::c_float, slices: core::ffi::c_int, color: Color, );
	/// Draw a cylinder wires with base at startPos and top at endPos
	pub fn DrawCylinderWiresEx(startPos: Vector3, endPos: Vector3, startRadius: core::ffi::c_float, endRadius: core::ffi::c_float, sides: core::ffi::c_int, color: Color, );
	/// Draw a capsule with the center of its sphere caps at startPos and endPos
	pub fn DrawCapsule(startPos: Vector3, endPos: Vector3, radius: core::ffi::c_float, slices: core::ffi::c_int, rings: core::ffi::c_int, color: Color, );
	/// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
	pub fn DrawCapsuleWires(startPos: Vector3, endPos: Vector3, radius: core::ffi::c_float, slices: core::ffi::c_int, rings: core::ffi::c_int, color: Color, );
	/// Draw a plane XZ
	pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color, );
	/// Draw a ray line
	pub fn DrawRay(ray: Ray, color: Color, );
	/// Draw a grid (centered at (0, 0, 0))
	pub fn DrawGrid(slices: core::ffi::c_int, spacing: core::ffi::c_float, );
	/// Load model from files (meshes and materials)
	pub fn LoadModel(fileName: *const core::ffi::c_char, ) -> Model;
	/// Load model from generated mesh (default material)
	pub fn LoadModelFromMesh(mesh: Mesh, ) -> Model;
	/// Check if a model is ready
	pub fn IsModelReady(model: Model, ) -> bool;
	/// Unload model (including meshes) from memory (RAM and/or VRAM)
	pub fn UnloadModel(model: Model, );
	/// Compute model bounding box limits (considers all meshes)
	pub fn GetModelBoundingBox(model: Model, ) -> BoundingBox;
	/// Draw a model (with texture if set)
	pub fn DrawModel(model: Model, position: Vector3, scale: core::ffi::c_float, tint: Color, );
	/// Draw a model with extended parameters
	pub fn DrawModelEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: core::ffi::c_float, scale: Vector3, tint: Color, );
	/// Draw a model wires (with texture if set)
	pub fn DrawModelWires(model: Model, position: Vector3, scale: core::ffi::c_float, tint: Color, );
	/// Draw a model wires (with texture if set) with extended parameters
	pub fn DrawModelWiresEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: core::ffi::c_float, scale: Vector3, tint: Color, );
	/// Draw bounding box (wires)
	pub fn DrawBoundingBox(r#box: BoundingBox, color: Color, );
	/// Draw a billboard texture
	pub fn DrawBillboard(camera: Camera, texture: Texture2D, position: Vector3, size: core::ffi::c_float, tint: Color, );
	/// Draw a billboard texture defined by source
	pub fn DrawBillboardRec(camera: Camera, texture: Texture2D, source: Rectangle, position: Vector3, size: Vector2, tint: Color, );
	/// Draw a billboard texture defined by source and rotation
	pub fn DrawBillboardPro(camera: Camera, texture: Texture2D, source: Rectangle, position: Vector3, up: Vector3, size: Vector2, origin: Vector2, rotation: core::ffi::c_float, tint: Color, );
	/// Upload mesh vertex data in GPU and provide VAO/VBO ids
	pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool, );
	/// Update mesh vertex data in GPU for a specific buffer index
	pub fn UpdateMeshBuffer(mesh: Mesh, index: core::ffi::c_int, data: *const core::ffi::c_void, dataSize: core::ffi::c_int, offset: core::ffi::c_int, );
	/// Unload mesh data from CPU and GPU
	pub fn UnloadMesh(mesh: Mesh, );
	/// Draw a 3d mesh with material and transform
	pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix, );
	/// Draw multiple mesh instances with material and different transforms
	pub fn DrawMeshInstanced(mesh: Mesh, material: Material, transforms: *const Matrix, instances: core::ffi::c_int, );
	/// Export mesh data to file, returns true on success
	pub fn ExportMesh(mesh: Mesh, fileName: *const core::ffi::c_char, ) -> bool;
	/// Compute mesh bounding box limits
	pub fn GetMeshBoundingBox(mesh: Mesh, ) -> BoundingBox;
	/// Compute mesh tangents
	pub fn GenMeshTangents(mesh: *mut Mesh, );
	/// Generate polygonal mesh
	pub fn GenMeshPoly(sides: core::ffi::c_int, radius: core::ffi::c_float, ) -> Mesh;
	/// Generate plane mesh (with subdivisions)
	pub fn GenMeshPlane(width: core::ffi::c_float, length: core::ffi::c_float, resX: core::ffi::c_int, resZ: core::ffi::c_int, ) -> Mesh;
	/// Generate cuboid mesh
	pub fn GenMeshCube(width: core::ffi::c_float, height: core::ffi::c_float, length: core::ffi::c_float, ) -> Mesh;
	/// Generate sphere mesh (standard sphere)
	pub fn GenMeshSphere(radius: core::ffi::c_float, rings: core::ffi::c_int, slices: core::ffi::c_int, ) -> Mesh;
	/// Generate half-sphere mesh (no bottom cap)
	pub fn GenMeshHemiSphere(radius: core::ffi::c_float, rings: core::ffi::c_int, slices: core::ffi::c_int, ) -> Mesh;
	/// Generate cylinder mesh
	pub fn GenMeshCylinder(radius: core::ffi::c_float, height: core::ffi::c_float, slices: core::ffi::c_int, ) -> Mesh;
	/// Generate cone/pyramid mesh
	pub fn GenMeshCone(radius: core::ffi::c_float, height: core::ffi::c_float, slices: core::ffi::c_int, ) -> Mesh;
	/// Generate torus mesh
	pub fn GenMeshTorus(radius: core::ffi::c_float, size: core::ffi::c_float, radSeg: core::ffi::c_int, sides: core::ffi::c_int, ) -> Mesh;
	/// Generate trefoil knot mesh
	pub fn GenMeshKnot(radius: core::ffi::c_float, size: core::ffi::c_float, radSeg: core::ffi::c_int, sides: core::ffi::c_int, ) -> Mesh;
	/// Generate heightmap mesh from image data
	pub fn GenMeshHeightmap(heightmap: Image, size: Vector3, ) -> Mesh;
	/// Generate cubes-based map mesh from image data
	pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3, ) -> Mesh;
	/// Load materials from model file
	pub fn LoadMaterials(fileName: *const core::ffi::c_char, materialCount: *mut core::ffi::c_int, ) -> *mut Material;
	/// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
	pub fn LoadMaterialDefault() -> Material;
	/// Check if a material is ready
	pub fn IsMaterialReady(material: Material, ) -> bool;
	/// Unload material from GPU memory (VRAM)
	pub fn UnloadMaterial(material: Material, );
	/// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
	pub fn SetMaterialTexture(material: *mut Material, mapType: core::ffi::c_int, texture: Texture2D, );
	/// Set material for a mesh
	pub fn SetModelMeshMaterial(model: *mut Model, meshId: core::ffi::c_int, materialId: core::ffi::c_int, );
	/// Load model animations from file
	pub fn LoadModelAnimations(fileName: *const core::ffi::c_char, animCount: *mut core::ffi::c_uint, ) -> *mut ModelAnimation;
	/// Update model animation pose
	pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: core::ffi::c_int, );
	/// Unload animation data
	pub fn UnloadModelAnimation(anim: ModelAnimation, );
	/// Unload animation array data
	pub fn UnloadModelAnimations(animations: *mut ModelAnimation, count: core::ffi::c_uint, );
	/// Check model animation skeleton match
	pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation, ) -> bool;
	/// Check collision between two spheres
	pub fn CheckCollisionSpheres(center1: Vector3, radius1: core::ffi::c_float, center2: Vector3, radius2: core::ffi::c_float, ) -> bool;
	/// Check collision between two bounding boxes
	pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox, ) -> bool;
	/// Check collision between box and sphere
	pub fn CheckCollisionBoxSphere(r#box: BoundingBox, center: Vector3, radius: core::ffi::c_float, ) -> bool;
	/// Get collision info between ray and sphere
	pub fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: core::ffi::c_float, ) -> RayCollision;
	/// Get collision info between ray and box
	pub fn GetRayCollisionBox(ray: Ray, r#box: BoundingBox, ) -> RayCollision;
	/// Get collision info between ray and mesh
	pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix, ) -> RayCollision;
	/// Get collision info between ray and triangle
	pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3, ) -> RayCollision;
	/// Get collision info between ray and quad
	pub fn GetRayCollisionQuad(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3, ) -> RayCollision;
	/// Initialize audio device and context
	pub fn InitAudioDevice();
	/// Close the audio device and context
	pub fn CloseAudioDevice();
	/// Check if audio device has been initialized successfully
	pub fn IsAudioDeviceReady() -> bool;
	/// Set master volume (listener)
	pub fn SetMasterVolume(volume: core::ffi::c_float, );
	/// Load wave data from file
	pub fn LoadWave(fileName: *const core::ffi::c_char, ) -> Wave;
	/// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
	pub fn LoadWaveFromMemory(fileType: *const core::ffi::c_char, fileData: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, ) -> Wave;
	/// Checks if wave data is ready
	pub fn IsWaveReady(wave: Wave, ) -> bool;
	/// Load sound from file
	pub fn LoadSound(fileName: *const core::ffi::c_char, ) -> Sound;
	/// Load sound from wave data
	pub fn LoadSoundFromWave(wave: Wave, ) -> Sound;
	/// Checks if a sound is ready
	pub fn IsSoundReady(sound: Sound, ) -> bool;
	/// Update sound buffer with new data
	pub fn UpdateSound(sound: Sound, data: *const core::ffi::c_void, sampleCount: core::ffi::c_int, );
	/// Unload wave data
	pub fn UnloadWave(wave: Wave, );
	/// Unload sound
	pub fn UnloadSound(sound: Sound, );
	/// Export wave data to file, returns true on success
	pub fn ExportWave(wave: Wave, fileName: *const core::ffi::c_char, ) -> bool;
	/// Export wave sample data to code (.h), returns true on success
	pub fn ExportWaveAsCode(wave: Wave, fileName: *const core::ffi::c_char, ) -> bool;
	/// Play a sound
	pub fn PlaySound(sound: Sound, );
	/// Stop playing a sound
	pub fn StopSound(sound: Sound, );
	/// Pause a sound
	pub fn PauseSound(sound: Sound, );
	/// Resume a paused sound
	pub fn ResumeSound(sound: Sound, );
	/// Check if a sound is currently playing
	pub fn IsSoundPlaying(sound: Sound, ) -> bool;
	/// Set volume for a sound (1.0 is max level)
	pub fn SetSoundVolume(sound: Sound, volume: core::ffi::c_float, );
	/// Set pitch for a sound (1.0 is base level)
	pub fn SetSoundPitch(sound: Sound, pitch: core::ffi::c_float, );
	/// Set pan for a sound (0.5 is center)
	pub fn SetSoundPan(sound: Sound, pan: core::ffi::c_float, );
	/// Copy a wave to a new wave
	pub fn WaveCopy(wave: Wave, ) -> Wave;
	/// Crop a wave to defined samples range
	pub fn WaveCrop(wave: *mut Wave, initSample: core::ffi::c_int, finalSample: core::ffi::c_int, );
	/// Convert wave data to desired format
	pub fn WaveFormat(wave: *mut Wave, sampleRate: core::ffi::c_int, sampleSize: core::ffi::c_int, channels: core::ffi::c_int, );
	/// Load samples data from wave as a 32bit float data array
	pub fn LoadWaveSamples(wave: Wave, ) -> *mut core::ffi::c_float;
	/// Unload samples data loaded with LoadWaveSamples()
	pub fn UnloadWaveSamples(samples: *mut core::ffi::c_float, );
	/// Load music stream from file
	pub fn LoadMusicStream(fileName: *const core::ffi::c_char, ) -> Music;
	/// Load music stream from data
	pub fn LoadMusicStreamFromMemory(fileType: *const core::ffi::c_char, data: *const core::ffi::c_uchar, dataSize: core::ffi::c_int, ) -> Music;
	/// Checks if a music stream is ready
	pub fn IsMusicReady(music: Music, ) -> bool;
	/// Unload music stream
	pub fn UnloadMusicStream(music: Music, );
	/// Start music playing
	pub fn PlayMusicStream(music: Music, );
	/// Check if music is playing
	pub fn IsMusicStreamPlaying(music: Music, ) -> bool;
	/// Updates buffers for music streaming
	pub fn UpdateMusicStream(music: Music, );
	/// Stop music playing
	pub fn StopMusicStream(music: Music, );
	/// Pause music playing
	pub fn PauseMusicStream(music: Music, );
	/// Resume playing paused music
	pub fn ResumeMusicStream(music: Music, );
	/// Seek music to a position (in seconds)
	pub fn SeekMusicStream(music: Music, position: core::ffi::c_float, );
	/// Set volume for music (1.0 is max level)
	pub fn SetMusicVolume(music: Music, volume: core::ffi::c_float, );
	/// Set pitch for a music (1.0 is base level)
	pub fn SetMusicPitch(music: Music, pitch: core::ffi::c_float, );
	/// Set pan for a music (0.5 is center)
	pub fn SetMusicPan(music: Music, pan: core::ffi::c_float, );
	/// Get music time length (in seconds)
	pub fn GetMusicTimeLength(music: Music, ) -> core::ffi::c_float;
	/// Get current music time played (in seconds)
	pub fn GetMusicTimePlayed(music: Music, ) -> core::ffi::c_float;
	/// Load audio stream (to stream raw audio pcm data)
	pub fn LoadAudioStream(sampleRate: core::ffi::c_uint, sampleSize: core::ffi::c_uint, channels: core::ffi::c_uint, ) -> AudioStream;
	/// Checks if an audio stream is ready
	pub fn IsAudioStreamReady(stream: AudioStream, ) -> bool;
	/// Unload audio stream and free memory
	pub fn UnloadAudioStream(stream: AudioStream, );
	/// Update audio stream buffers with data
	pub fn UpdateAudioStream(stream: AudioStream, data: *const core::ffi::c_void, frameCount: core::ffi::c_int, );
	/// Check if any audio stream buffers requires refill
	pub fn IsAudioStreamProcessed(stream: AudioStream, ) -> bool;
	/// Play audio stream
	pub fn PlayAudioStream(stream: AudioStream, );
	/// Pause audio stream
	pub fn PauseAudioStream(stream: AudioStream, );
	/// Resume audio stream
	pub fn ResumeAudioStream(stream: AudioStream, );
	/// Check if audio stream is playing
	pub fn IsAudioStreamPlaying(stream: AudioStream, ) -> bool;
	/// Stop audio stream
	pub fn StopAudioStream(stream: AudioStream, );
	/// Set volume for audio stream (1.0 is max level)
	pub fn SetAudioStreamVolume(stream: AudioStream, volume: core::ffi::c_float, );
	/// Set pitch for audio stream (1.0 is base level)
	pub fn SetAudioStreamPitch(stream: AudioStream, pitch: core::ffi::c_float, );
	/// Set pan for audio stream (0.5 is centered)
	pub fn SetAudioStreamPan(stream: AudioStream, pan: core::ffi::c_float, );
	/// Default size for new audio streams
	pub fn SetAudioStreamBufferSizeDefault(size: core::ffi::c_int, );
	/// Audio thread callback to request new data
	pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback, );
	/// Attach audio stream processor to stream
	pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback, );
	/// Detach audio stream processor from stream
	pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback, );
	/// Attach audio stream processor to the entire audio pipeline
	pub fn AttachAudioMixedProcessor(processor: AudioCallback, );
	/// Detach audio stream processor from the entire audio pipeline
	pub fn DetachAudioMixedProcessor(processor: AudioCallback, );
}

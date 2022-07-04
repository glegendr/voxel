use ogl33::*;

/// The types of buffer object that you can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
  /// Array Buffers holds arrays of vertex data for drawing.
  Array = GL_ARRAY_BUFFER as isize,
  /// Element Array Buffers hold indexes of what vertexes to use for drawing.
  ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

/// Basic wrapper for a [Buffer
/// Object](https://www.khronos.org/opengl/wiki/Buffer_Object).
pub struct Buffer(pub GLuint);
impl Buffer {
  /// Makes a new vertex buffer
  pub fn new() -> Option<Self> {
    let mut vbo = 0;
    unsafe {
      glGenBuffers(1, &mut vbo);
    }
    if vbo != 0 {
      Some(Self(vbo))
    } else {
      None
    }
  }

  /// Bind this vertex buffer for the given type
  pub fn bind(&self, ty: BufferType) {
    unsafe { glBindBuffer(ty as GLenum, self.0) }
  }

  /// Clear the current vertex buffer binding for the given type.
  pub fn clear_binding(ty: BufferType) {
    unsafe { glBindBuffer(ty as GLenum, 0) }
  }
}

/// Places a slice of data into a previously-bound buffer.
pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
  unsafe {
    glBufferData(
      ty as GLenum,
      data.len().try_into().unwrap(),
      data.as_ptr().cast(),
      usage,
    );
  }
}
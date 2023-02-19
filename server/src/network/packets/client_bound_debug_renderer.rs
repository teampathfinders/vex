use bytes::{BufMut, BytesMut};
use common::{Serialize, VResult, Vector3f, Vector4f, WriteExtensions};

use super::ConnectedPacket;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugRendererAction {
    Clear = 1,
    AddCube,
}

#[derive(Debug, Clone)]
pub struct ClientBoundDebugRenderer<'a> {
    /// Action to perform.
    pub action: DebugRendererAction,
    /// Text to display above the debug renderer.
    pub text: &'a str,
    /// Position of the renderer.
    pub position: Vector3f,
    /// Colour of the debug renderer.
    /// Every component should range from 0-1.
    pub color: Vector4f,
    /// How long the renderer will last in milliseconds.
    pub duration: i64,
}

impl ConnectedPacket for ClientBoundDebugRenderer<'_> {
    const ID: u32 = 0xa4;
}

impl Serialize for ClientBoundDebugRenderer<'_> {
    fn serialize(&self) -> VResult<BytesMut> {
        let mut buffer = BytesMut::new();

        buffer.put_i32_le(self.action as i32);
        if self.action == DebugRendererAction::AddCube {
            buffer.put_string(self.text);
            buffer.put_vec3f(&self.position);
            buffer.put_vec4f(&self.color);
            buffer.put_i64_le(self.duration);
        }

        Ok(buffer)
    }
}

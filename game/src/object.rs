use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Clone)]
pub struct NetworkObject {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    #[deku(update = "self.id.len()")]
    id_len: usize,
    #[deku(count = "id_len")]
    pub id: Vec<u8>,
}

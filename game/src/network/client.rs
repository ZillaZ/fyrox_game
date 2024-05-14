use crate::object::NetworkObject;
use std::{net::TcpStream, io::{Write, Read}};
use deku::prelude::*;
use fyrox::core::algebra::Vector3;
use crate::session::*;

#[derive(DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum Reason {
    #[deku(id = "0x1")]
    IdInUse,
    #[deku(id = "0x2")]
    InvalidRequestFormat,
    #[deku(id = "0x3")]
    InvalidIdFormat,
    #[deku(id = "0x4")]
    InvalidPassword,
    #[deku(id = "0x5")]
    IdDoesntExist,
    #[deku(id = "0x6")]
    WrongPassword
}

impl ToString for Reason {
    fn to_string(&self) -> String {
        use Reason::*;
        match self {
            IdInUse => "The given ID is already in use.".into(),
            InvalidRequestFormat => "The request is invalid.".into(),
            InvalidIdFormat => "The given ID is invalid".into(),
            InvalidPassword => "The given password is invalid".into(),
            IdDoesntExist => "There is no session with the given ID".into(),
            WrongPassword => "The given password is incorrect".into()
        }
    }
}

#[derive(DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ServerResponse {
    #[deku(id = "0x1")]
    Ok(ResponseSignal),
    #[deku(id = "0x2")]
    InvalidRequest(Reason),
}
#[derive(Clone, DekuRead, DekuWrite)]
pub struct PlayerSignal {
    desired_mov: [f32; 3],
    desired_rot: [f32; 2],
    camera_radius: f32,
}

impl PlayerSignal {
    pub fn new(desired_mov: Vector3<f32>, desired_rot: [f32; 2], camera_radius: f32) -> Self {
        Self {
            desired_mov: [desired_mov.x, desired_mov.y, desired_mov.z],
            desired_rot,
            camera_radius,
        }
    }
}


#[derive(Clone, Debug, DekuRead, DekuWrite)]
pub struct ResponseSignal {
    #[deku(update = "self.players.len()")]
    pub player_count: usize,
    #[deku(update = "self.objects.len()")]
    pub object_count: usize,
    pub translation: [f32; 3],
    pub camera_pos: [f32; 3],
    pub camera_target: [f32; 3],
    pub fwd: [f32; 3],
    pub right: [f32; 3],
    #[deku(count = "player_count")]
    pub players: Vec<ResponseSignal>,
    #[deku(count = "object_count")]
    pub objects: Vec<NetworkObject>,
}

impl ResponseSignal {
    pub fn new(
        translation: Vector3<f32>,
        camera_pos: Vector3<f32>,
        camera_target: Vector3<f32>,
        fwd: Vector3<f32>,
        right: Vector3<f32>,
    ) -> Self {
        Self {
            player_count: 0,
            object_count: 0,
            translation: [translation.x, translation.y, translation.z],
            camera_pos: [camera_pos.x, camera_pos.y, camera_pos.z],
            camera_target: [camera_target.x, camera_target.y, camera_target.z],
            fwd: [fwd.x, fwd.y, fwd.z],
            right: [right.x, right.y, right.z],
            players: Vec::new(),
            objects: Vec::new(),
        }
    }
}

impl Default for ResponseSignal {
    fn default() -> Self {
        Self::new(
            Vector3::zeros(),
            Vector3::zeros(),
            Vector3::zeros(),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0),
        )
    }
}

pub fn connect(
    stream: &mut TcpStream,
    desired_translation: Vector3<f32>,
    desired_rotation: [f32; 2]
) -> ResponseSignal {
    let mut buf = [0; 1024];
    let update_data = PlayerSignal::new(desired_translation, desired_rotation, 5.0);
    stream
        .write(&update_data.to_bytes().unwrap())
        .unwrap();
    let _response = stream.read(&mut buf).unwrap();
    let parsed = ResponseSignal::from_bytes((&buf, 0)).unwrap();
    println!("{:?}", parsed.1);
    parsed.1
}

pub fn get_stream() -> TcpStream {
    TcpStream::connect("127.0.0.1:9001").unwrap()
}

pub fn create_game(id: &str, password: &str, stream: &mut TcpStream) -> ServerResponse {
    let request = ServerRequest::NewSession(NewSessionRequest::new(&id, &password));
    stream.write(&request.to_bytes().unwrap()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    ServerResponse::from_bytes((&buffer, 0)).unwrap().1
}

pub fn join_game(id: &str, password: &str, stream: &mut TcpStream) -> JoinResponse {
    let request = ServerRequest::JoinSession(JoinSessionRequest::new(id, password));
    stream.write(&request.to_bytes().unwrap()).unwrap();
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    JoinResponse::from_bytes((&buffer, 0)).unwrap().1
}

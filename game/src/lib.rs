//! Game project.
use fyrox::{
    core::{pool::Handle, algebra::Vector2},
    event::Event,
    gui::{message::UiMessage, UserInterface, screen::ScreenBuilder, widget::{WidgetBuilder, WidgetMessage}, button::{ButtonBuilder, ButtonMessage, Button, ButtonContent}, UiNode, canvas::CanvasBuilder, grid::GridBuilder, border::BorderBuilder, stack_panel::StackPanelBuilder, BuildContext, VerticalAlignment, text::{Text, TextMessage, TextBuilder}, text_box::TextBoxBuilder},
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::Scene,
};

use player::Player;
use std::{path::Path, net::TcpStream};
use network::*;

pub mod menus;
pub mod network;
pub mod object;
pub mod player;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {
        _context.serialization_context.script_constructors.add::<Player>("Player");
    }

    fn create_instance(&self, scene_path: Option<&str>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(scene_path, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    active_menu: Handle<UiNode>,
    stream: TcpStream
}

impl Game {
    pub fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
        let mut ctx = context.user_interface.build_ctx();
        let active_menu = menus::draw::build_main_menu(&mut ctx);
        let stream = client::get_stream();
        Self {
            scene: Handle::NONE,
            active_menu,
            stream
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
    ) {
        let ui = &mut context.user_interface;
        menus::events::match_button_click(&mut self.stream, ui, message);
    }

    fn on_scene_begin_loading(&mut self, path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}

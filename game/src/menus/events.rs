use fyrox::gui::text_box::TextBox;

use crate::{*, network::{client::*, session::JoinResponse}};

pub fn match_button_click(stream: &mut TcpStream, ui: &mut UserInterface, message: &UiMessage) {
    if let Some(ButtonMessage::Click) = message.data() {
        let name = ui.node(message.destination).name.clone();
        match name.as_str() {
            "CreateButton" => {
                destroy_active_menu(ui, message);
                let mut ctx = ui.build_ctx();
                menus::draw::build_create_menu(&mut ctx);
            },
            "JoinButton" => {
                destroy_active_menu(ui, message);
                let mut ctx = ui.build_ctx();
                menus::draw::build_join_menu(&mut ctx);
            },
            "BackButton" => {
                destroy_active_menu(ui, message);
                let mut ctx = ui.build_ctx();
                menus::draw::build_main_menu(&mut ctx);
            },
            "CreateGame" => {
                let session_id = get_textbox_data(ui, "IdBox", message.destination);
                let session_password = get_textbox_data(ui, "PasswordBox", message.destination);
                let response = create_game(&session_id, &session_password, stream);
                match response {
                    ServerResponse::Ok(_) => destroy_active_menu(ui, message),
                    _ => {}
                }
            }
            "JoinGame" => {
               let session_id = get_textbox_data(ui, "IdBox", message.destination);
                let session_password = get_textbox_data(ui, "PasswordBox", message.destination);
                let response = join_game(&session_id, &session_password, stream);
                match response {
                    JoinResponse::Err(_) => destroy_active_menu(ui, message),
                    _ => {}
                }
            },
            _ => ()
        }

    }
}

fn destroy_active_menu(ui: &mut UserInterface, message: &UiMessage) {
    let widget = ui.find_by_name_up(message.destination, "ScreenWrapper");
    ui.send_message(
        WidgetMessage::remove(widget, fyrox::gui::message::MessageDirection::ToWidget)
    );
}

fn get_textbox_data(ui: &mut UserInterface, name: &str, start: Handle<UiNode>) -> String {
    let root = ui.find_by_name_up(start, "SessionInfo");
    let textbox_handle = ui.find_by_name_down(root, name);
    let textbox = ui.node(textbox_handle);
    textbox.cast::<TextBox>().unwrap().text()
}

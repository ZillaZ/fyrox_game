
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*, algebra::{Vector3, Vector4}},
    event::{Event, WindowEvent, ElementState}, script::{ScriptContext, ScriptDeinitContext, ScriptTrait}, keyboard::{KeyCode, PhysicalKey},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "dc449087-7376-41d7-ba44-68171b66e635")]
#[visit(optional)]
pub struct Player {
    pub position: Vector3<f32>,
    pub rotation: Vector4<f32>,
    desired_translation: Vector3<f32>,
    desired_rotation: Vector4<f32>
}

impl ScriptTrait for Player {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        match event {
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event, .. }, .. } => {
                let pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(code) = event.physical_key {
                    self.desired_translation += match code {
                        KeyCode::KeyW => Vector3::new(0.0, 0.0, 1.0),
                        KeyCode::KeyA => Vector3::new(-1.0, 0.0, 0.0),
                        KeyCode::KeyS => Vector3::new(0.0, 0.0, -1.0),
                        KeyCode::KeyD => Vector3::new(1.0, 0.0, 0.0),
                        _ => Vector3::new(0.0, 0.0, 0.0)
                    };
                    self.desired_translation = self.desired_translation.normalize();
                }
            },
            _ => {}
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {

    }
}

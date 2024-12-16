use bevy::prelude::*;
use bevy_input::keyboard::KeyboardInput;
use bevy_text_popup::{TextPopupButton, TextPopupEvent, TextPopupLocation, TextPopupPlugin, TextPopupTimeout};



#[derive(Resource)]
pub struct PopupQueue {
    messages: Vec<String>,
}

#[derive(Resource)]
pub struct PopupState {
    pub is_popup_active: bool,
}
pub fn welcome_setup(mut commands: Commands) {
    let messages = vec![
        "Welcome, Cliff!".to_string(),
        "Use the Arrow Keys to move around.".to_string(),
        "Your goal is to clean up the clutter around the office.".to_string(),
        "Good luck and have fun!".to_string(),
        "...".to_string(),
        "Feel free to add more dialogue".to_string(),
        "Maybe we could do more storytelling in these first few seconds of the game?".to_string(),
    ];    
    commands.insert_resource(PopupQueue { messages: messages.into_iter().rev().collect() });
    commands.insert_resource(PopupState { is_popup_active: false });

}


pub fn handle_next_popup(
    mut text_popup_events: EventWriter<TextPopupEvent>,
    mut popup_queue: ResMut<PopupQueue>,
    mut popup_state: ResMut<PopupState>,
    //keys: Res<ButtonInput<KeyCode>>,
) {
    if popup_state.is_popup_active {
        return;
    }

    if let Some(next_message) = popup_queue.messages.pop() {
        popup_state.is_popup_active = true;
        trigger_popup(&mut text_popup_events, &next_message);
    }

}


pub fn trigger_popup(
    text_popup_events: &mut EventWriter<TextPopupEvent>,
    content: &str,
) {
    let event = TextPopupEvent {
        
        location: (TextPopupLocation::Center),
        content: content.to_string(),
        confirm_button: Some(TextPopupButton {
            font_size: 18.0,
            text: "OK".to_string(),
            action: |commands, root_entity| {
                commands.entity(root_entity).despawn_recursive();
                commands.insert_resource(PopupState { is_popup_active: false });

            },
            ..Default::default()
        }),
        ..Default::default()
    };
    text_popup_events.send(event);
}

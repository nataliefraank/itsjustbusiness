use bevy::prelude::*;
use bevy_input::keyboard::KeyboardInput;
use bevy_text_popup::{TextPopupButton, TextPopupEvent, TextPopupLocation, TextPopupPlugin, TextPopupTimeout};
use bevy_kira_audio::prelude::*;
use crate::audio::ButtonPress;
use crate::audio::ButtonPressTriggered;




#[derive(Resource)]
pub struct PopupQueue {
    messages: Vec<String>,
}

#[derive(Resource)]
pub struct PopupState {
    pub is_popup_active: bool,
}


#[derive(Resource)]
pub struct ButtonPressState {
    pub triggered: bool,
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
    commands.insert_resource(ButtonPressState { triggered: false });

}


pub fn handle_next_popup(
    mut text_popup_events: EventWriter<TextPopupEvent>,
    mut popup_queue: ResMut<PopupQueue>,
    mut popup_state: ResMut<PopupState>,
    //keys: Res<ButtonInput<KeyCode>>,
    button_press: Res<AudioChannel<ButtonPress>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut button_press_state: ResMut<ButtonPressState>
) {
    if popup_state.is_popup_active {
        return;
    }

    if let Some(next_message) = popup_queue.messages.pop() {
        popup_state.is_popup_active = true;
        trigger_popup(&mut text_popup_events, &next_message, button_press, asset_server, audio);
    }

}


pub fn trigger_popup(
    text_popup_events: &mut EventWriter<TextPopupEvent>,
    content: &str,
    button_press: Res<AudioChannel<ButtonPress>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    let event = TextPopupEvent {
        
        location: (TextPopupLocation::Center),
        content: content.to_string(),
        confirm_button: Some(TextPopupButton {
            font_size: 18.0,
            text: "OK".to_string(),
            action: |commands, root_entity| {
                commands.insert_resource(ButtonPressState { triggered: true });
                commands.entity(root_entity).despawn_recursive();
                commands.insert_resource(PopupState { is_popup_active: false });

            },
            ..Default::default()
        }),
        ..Default::default()
    };
    text_popup_events.send(event);
}

pub fn game_ui(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    
    text_popup_events.send(TextPopupEvent {
        content: "TASKS COMPLETED: 0".to_string(),
        font_size: 25.0,
        background_color: Color::BLACK.with_alpha(0.0),
        border_color: Color::BLACK.with_alpha(0.0),
        location: TextPopupLocation::TopLeft,
        padding: UiRect {
            left: Val::Px(200.0),
            right: Val::Px(20.0),
            top: Val::Px(5.0),
            bottom: Val::Px(10.0),
        },
        ..default()
    });

    text_popup_events.send(TextPopupEvent {
        content: "TIME: 05:01 P.M.".to_string(),
        font_size: 25.0,
        background_color: Color::BLACK.with_alpha(0.0),
        border_color: Color::BLACK.with_alpha(0.0),
        location: TextPopupLocation::TopRight,
        padding: UiRect {
            left: Val::Px(20.0),
            right: Val::Px(200.0),
            top: Val::Px(5.0),
            bottom: Val::Px(10.0),
        },
        ..default()
    });

    text_popup_events.send(TextPopupEvent {
        content: "LEVEL 1".to_string(),
        font_size: 25.0,
        background_color: Color::BLACK.with_alpha(0.0),
        border_color: Color::BLACK.with_alpha(0.0),
        location: TextPopupLocation::Top,
        padding: UiRect {
            left: Val::Px(50.0),
            right: Val::Px(20.0),
            top: Val::Px(5.0),
            bottom: Val::Px(10.0),
        },
        ..default()
    });
}
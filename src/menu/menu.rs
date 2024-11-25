use bevy::prelude::*;
use crate::*;
use bevy::color::palettes::css::ALICE_BLUE;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;

const NORMAL_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const TEXT_COLOR: Color = Color::srgb(0.95, 0.09, 0.08);

const BACKGROUND_COLOR : Srgba = ALICE_BLUE;


use bevy::prelude::Component;

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    FollowApple,
    TargetSelection,
    // SettingsDisplay,
    // SettingsSound,
    // BackToMainMenu,
    // BackToSettings,
    Quit,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub struct OnGameScreen;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}


pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<TaskState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }

                MenuButtonAction::FollowApple => {
                    game_state.set(TaskState::FollowApple);
                    menu_state.set(MenuState::Disabled);
                }

                MenuButtonAction::TargetSelection => {
                    game_state.set(TaskState::TargetSelection);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}


// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
mut interaction_query: Query<
(&Interaction, &mut UiImage, Option<&SelectedOption>),
(Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image, selected) in &mut interaction_query {
        image.color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON,
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON,
            (Interaction::Hovered, None) => HOVERED_BUTTON,
            (Interaction::None, None) => NORMAL_BUTTON,
        }
    }
}

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>, mut windows: Query<&mut Window>) {
    menu_state.set(MenuState::Main);
    windows.single_mut().cursor.visible = true;
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Task Selection",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::FollowApple,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/apple.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Apple",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::TargetSelection,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/target.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Target",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}

pub fn get_to_menu(mut keyboard_input_events: EventReader<KeyboardInput>,
    mut next_state: ResMut<NextState<TaskState>>) {


for event in keyboard_input_events.read() {
if event.state == ButtonState::Pressed {
    if event.key_code == KeyCode::Escape { next_state.set(TaskState::Menu); }
}
// println!("Changing network state #{:?} ----> #{:?} || event keycode : {:?}", state.get(), next_state, event.key_code);
}
}


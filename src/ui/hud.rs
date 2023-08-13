use crate::prelude::*;

#[derive(Component)]
struct HPText;

#[derive(Component)]
struct HPBar;
#[derive(Component)]
struct LogUI;
fn buttom_hud(mut commands: Commands, font_manager: Res<FontManager>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(5.),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    bottom: Val::Percent(15.),
                    border: UiRect::all(Val::Px(3.0)),
                    // display:Display::Flex,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(0.65, 0.65, 0.65)),
                ..Default::default()
            },
            TopUINode,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    //hp text
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(35.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    style: Style {
                                        height: Val::Px(20. * 1.),
                                        // Set height to font size * number of text lines
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            bottom: Val::Auto,
                                            top: Val::Auto,
                                        },
                                        ..Default::default()
                                    },
                                    text: Text::from_section(
                                        "HP: 17 / 20".to_string(),
                                        TextStyle {
                                            font_size: 20.0,
                                            font: font_manager.font.clone(),
                                            color: Color::rgb(0.99, 0.99, 0.99),
                                        },
                                    ),
                                    ..Default::default()
                                },
                                HPText,
                            ));
                        });
                    //outside hp bar
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(63.0),
                                height: Val::Px(20. * 1.),
                                border: UiRect::all(Val::Px(5.0)),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    bottom: Val::Auto,
                                    top: Val::Auto,
                                },
                                ..Default::default()
                            },
                            background_color: Color::rgb(0.5, 0.1, 0.1).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(50.0),
                                        height: Val::Percent(100.0),
                                        ..Default::default()
                                    },
                                    background_color: Color::rgb(0.99, 0.1, 0.1).into(),
                                    ..Default::default()
                                },
                                HPBar,
                            ));
                        });
                });
        });
    //覆う箱みたいな感じ
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(15.),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    border: UiRect::all(Val::Px(3.0)),
                    //display:Display::Flex,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(0.65, 0.65, 0.65)),
                ..Default::default()
            },
            TopUINode,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),

                        left: Val::Percent(0.0),

                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::OLIVE),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text {
                                // Construct a `Vec` of `TextSection`s
                                sections: vec![
                                    TextSection {
                                        value: "Log...".to_string(),
                                        style: TextStyle {
                                            font: font_manager.font.clone(),
                                            font_size: 19.0, // BEVY bug, return to 20 in 0.11.1
                                            color: Color::YELLOW,
                                        },
                                    },
                                    TextSection {
                                        value: "Use the arrow keys to move.".to_string(),
                                        style: TextStyle {
                                            font: font_manager.font.clone(),
                                            font_size: 20.0,
                                            color: Color::YELLOW,
                                        },
                                    },
                                    TextSection {
                                        value: "Use the arrow keys to move.".to_string(),
                                        style: TextStyle {
                                            font: font_manager.font.clone(),
                                            font_size: 20.0,
                                            color: Color::YELLOW,
                                        },
                                    },
                                    TextSection {
                                        value: "Use the arrow keys to move.".to_string(),
                                        style: TextStyle {
                                            font: font_manager.font.clone(),
                                            font_size: 20.0,
                                            color: Color::YELLOW,
                                        },
                                    },
                                ],
                                alignment: TextAlignment::Left,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        LogUI,
                    ));
                });
        });
}
fn update_hp_text_and_bar(
    mut text_query: Query<&mut Text, With<HPText>>,
    mut bar_query: Query<&mut Style, With<HPBar>>,
    player_query: Query<&Health, With<Player>>,
    // player_query: Query<&Health, (With<Player>, Or<(Changed<Health>, Changed<HPText>)>)>,
) {
    for player_hp in player_query.iter() {
        let (current, max) = (player_hp.current, player_hp.max);

        // update HP text
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("HP: {} / {}", current, max);
        }

        // update HP bar
        let bar_fill = (current as f32 / max as f32) * 100.0;
        for mut bar in bar_query.iter_mut() {
            bar.width = Val::Percent(bar_fill);
        }
    }
}
fn update_game_log(game_log: Res<GameLog>, mut text_query: Query<&mut Text, With<LogUI>>) {
    for mut text in text_query.iter_mut() {
        for (i, entry) in game_log.entries.iter().enumerate() {
            text.sections[i].value = entry.clone();
        }
    }
}
pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, buttom_hud).add_systems(
            Update,
            (
                update_hp_text_and_bar,
                update_game_log,
                // update_dungeonleveltext
            )
                .run_if(in_state(TurnState::AwaitingInput)),
        );
    }
}

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::ORIGINAL_JOE_POSITION;
use crate::PLAYER_NAME;
use crate::dialogue::components::*;
use crate::dialogue::events::*;
use crate::dialogue::resources::*;
use crate::dialogue::*;

pub fn setup_dialogue(mut commands: Commands) {
    let dialogue_json = include_str!("../../assets/dialogue.json");

    let dialogue_data: DialogueData = serde_json::from_str(dialogue_json).unwrap();

    commands.insert_resource(DialogueState {
        active: false,
        current_text: String::new(),
        current_speaker: String::new(),
    });

    commands.insert_resource(DialogueLog::default());

    commands.spawn((
        DialogueManager {
            original_camera_transform: None,
            current_scene: String::new(),
            current_line: 0,
            dialogue_data,
            auto_mode: false,
            waiting_for_choice: false,
            current_choices: Vec::new(),
            ephemeral_line: None,
        },
        DialogueTimer(Timer::from_seconds(0.0, TimerMode::Once)),
    ));
}

pub fn handle_start_dialogue(
    mut events: EventReader<StartDialogueEvent>,
    mut dialogue_query: Query<&mut DialogueManager>,
    mut dialogue_state: ResMut<DialogueState>,
    mut next_events: EventWriter<NextDialogueEvent>,
    camera_query: Query<&Transform, With<DialogueCamera>>,
) {
    for event in events.read() {
        let camera_transform = camera_query.get_single().unwrap();
        let mut manager = dialogue_query.get_single_mut().unwrap();
        manager.original_camera_transform = Some(*camera_transform);
        manager.current_scene = event.scene_name.clone();
        manager.current_line = 0;
        dialogue_state.active = true;
        next_events.send(NextDialogueEvent);
    }
}

pub fn handle_next_dialogue(
    mut events: EventReader<NextDialogueEvent>,
    mut dialogue_query: Query<(&mut DialogueManager, &mut DialogueTimer)>,
    mut dialogue_state: ResMut<DialogueState>,
    mut camera_query: Query<&mut Transform, (With<DialogueCamera>, Without<DialogueTarget>)>,
    mut dialogue_log: ResMut<DialogueLog>,
    mut action_events: EventWriter<ActionTriggerEvent>,
    targets_query: Query<(&Transform, &Speaker), (With<DialogueTarget>, Without<DialogueCamera>)>,
) {
    for _ in events.read() {
        let (mut manager, mut timer) = dialogue_query.get_single_mut().unwrap();

        if manager.waiting_for_choice {
            return;
        }

        if let Some(line) = manager.ephemeral_line.take() {
            dialogue_state.current_text = line.text.clone();
            dialogue_state.current_speaker = line.speaker.clone();

            dialogue_log.entries.push(DialogueLogEntry::Line {
                speaker: line.speaker.clone(),
                text: line.text.clone(),
            });

            if let Some(camera_target) = &line.camera_target {
                let mut camera_transform = camera_query.get_single_mut().unwrap();
                for (target_transform, speaker) in targets_query.iter() {
                    if speaker.name == *camera_target {
                        let target_pos = target_transform.translation;
                        camera_transform.translation = target_pos + Vec3::new(0.0, 2.0, 5.0);
                        camera_transform.look_at(target_pos + Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
                        break;
                    }
                }
            }

            if manager.auto_mode {
                if let Some(auto_time) = line.auto_time {
                    timer.0 = Timer::from_seconds(auto_time, TimerMode::Once);
                }
            }

            return;
        }

        if let Some(scene) = manager.dialogue_data.scenes.get(&manager.current_scene) {
            if manager.current_line < scene.len() {
                let line = &scene[manager.current_line];

                dialogue_state.current_text = line.text.clone();
                dialogue_state.current_speaker = line.speaker.clone();

                dialogue_log.entries.push(DialogueLogEntry::Line {
                    speaker: line.speaker.clone(),
                    text: line.text.clone(),
                });

                if let Some(camera_target) = &line.camera_target {
                    let mut camera_transform = camera_query.get_single_mut().unwrap();
                    for (target_transform, speaker) in targets_query.iter() {
                        if speaker.name == *camera_target {
                            let target_pos = target_transform.translation;
                            camera_transform.translation = target_pos + Vec3::new(0.0, 2.0, 5.0);
                            camera_transform
                                .look_at(target_pos + Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
                            break;
                        }
                    }
                }

                if let Some(triggers) = &line.triggers {
                    for trigger in triggers {
                        action_events.send(ActionTriggerEvent(trigger.clone()));
                    }
                }

                if let Some(choices) = &line.choices {
                    let choices_clone = choices.clone();
                    manager.waiting_for_choice = true;
                    manager.current_choices = choices_clone;
                } else {
                    if manager.auto_mode {
                        if let Some(auto_time) = line.auto_time {
                            timer.0 = Timer::from_seconds(auto_time, TimerMode::Once);
                        }
                    }

                    manager.current_line += 1;
                }
            } else {
                if let Some(original_transform) = manager.original_camera_transform {
                    let mut camera_transform = camera_query.get_single_mut().unwrap();
                    *camera_transform = original_transform;
                }

                dialogue_state.active = false;
                dialogue_state.current_text.clear();
                dialogue_state.current_speaker.clear();
            }
        }
    }
}

pub fn handle_auto_dialogue(
    time: Res<Time>,
    mut dialogue_query: Query<(&DialogueManager, &mut DialogueTimer)>,
    dialogue_state: Res<DialogueState>,
    mut next_events: EventWriter<NextDialogueEvent>,
) {
    if !dialogue_state.active {
        return;
    }

    if let Ok((manager, mut timer)) = dialogue_query.get_single_mut() {
        if manager.auto_mode && !manager.waiting_for_choice {
            timer.0.tick(time.delta());
            if timer.0.finished() {
                next_events.send(NextDialogueEvent);
            }
        }
    }
}

pub fn handle_toggle_auto(
    mut events: EventReader<ToggleAutoEvent>,
    mut dialogue_query: Query<&mut DialogueManager>,
) {
    for _ in events.read() {
        if let Ok(mut manager) = dialogue_query.get_single_mut() {
            manager.auto_mode = !manager.auto_mode;
        }
    }
}

pub fn handle_reset_scene(
    mut query: Query<&mut Transform, With<Actor>>,
    mut events: EventReader<ResetSceneEvent>,
) {
    for _ in events.read() {
        let mut transform = query.get_single_mut().unwrap();
        transform.translation = ORIGINAL_JOE_POSITION;
    }
}

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_events: EventWriter<NextDialogueEvent>,
    mut auto_events: EventWriter<ToggleAutoEvent>,
    mut start_events: EventWriter<StartDialogueEvent>,
    mut dialogue_log: ResMut<DialogueLog>,
    dialogue_state: Res<DialogueState>,
) {
    if keys.just_pressed(KeyCode::Space) && dialogue_state.active {
        next_events.send(NextDialogueEvent);
    }

    if keys.just_pressed(KeyCode::KeyA) {
        auto_events.send(ToggleAutoEvent);
    }

    if keys.just_pressed(KeyCode::KeyT) {
        if !dialogue_state.active {
            start_events.send(StartDialogueEvent {
                scene_name: "intro".to_string(),
            });
        }
    }

    if keys.just_pressed(KeyCode::KeyL) {
        dialogue_log.show_log = !dialogue_log.show_log;
    }
}

pub fn handle_choice(
    mut events: EventReader<MakeChoiceEvent>,
    mut dialogue_query: Query<&mut DialogueManager>,
    mut next_events: EventWriter<NextDialogueEvent>,
    mut dialogue_log: ResMut<DialogueLog>,
    mut action_events: EventWriter<ActionTriggerEvent>,
) {
    for event in events.read() {
        let mut manager = dialogue_query.get_single_mut().unwrap();
        if manager.waiting_for_choice && event.choice_index < manager.current_choices.len() {
            dialogue_log.entries.push(DialogueLogEntry::Choices {
                options: manager.current_choices.clone(),
                selected_index: event.choice_index,
            });

            let choice = manager.current_choices[event.choice_index].clone();

            if let Some(triggers) = &choice.triggers {
                for trigger in triggers {
                    action_events.send(ActionTriggerEvent(trigger.clone()));
                }
            }

            let player_line = DialogueLine {
                speaker: PLAYER_NAME.to_string(),
                text: choice
                    .player_text
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| choice.text.clone()),
                auto_time: choice.player_text_auto_time.clone(),
                camera_target: Some(PLAYER_NAME.to_string()),
                choices: None,
                triggers: None,
            };

            manager.ephemeral_line = Some(player_line);

            if let Some(next_scene) = &choice.next_scene {
                manager.current_scene = next_scene.clone();
                manager.current_line = 0;
            } else if let Some(next_line) = choice.next_line {
                manager.current_line = next_line;
            } else {
                manager.current_line += 1;
            }

            manager.waiting_for_choice = false;
            manager.current_choices.clear();

            next_events.send(NextDialogueEvent);
        }
    }
}

pub fn handle_move_to_trigger(
    mut commands: Commands,
    mut events: EventReader<ActionTriggerEvent>,
    mut query: Query<(Entity, &Actor)>,
) {
    for event in events.read() {
        if event.0.trigger_type != "move_to" {
            continue;
        }

        for (entity, actor) in query.iter_mut() {
            if actor.name == event.0.target {
                let params = &event.0.params;

                if let (Some(x), Some(y), Some(z)) = (
                    params.get("x").and_then(|v| v.as_f64()),
                    params.get("y").and_then(|v| v.as_f64()),
                    params.get("z").and_then(|v| v.as_f64()),
                ) {
                    let target_pos = Vec3::new(x as f32, y as f32, z as f32);
                    let speed = params
                        .get("speed")
                        .and_then(|v| v.as_f64())
                        .map(|s| s as f32)
                        .unwrap_or(5.0);

                    commands.entity(entity).insert(MovementGoal {
                        target: target_pos,
                        speed,
                    });

                    break;
                }
            }
        }
    }
}

pub fn process_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &MovementGoal)>,
) {
    for (entity, mut transform, goal) in query.iter_mut() {
        let direction = goal.target - transform.translation;
        let distance = direction.length();
        let step = goal.speed * time.delta_secs();

        if distance <= step {
            transform.translation = goal.target;
            commands.entity(entity).remove::<MovementGoal>();
        } else {
            transform.translation += direction.normalize() * step;
        }
    }
}

pub fn dialogue_ui(
    mut contexts: EguiContexts,
    dialogue_state: Res<DialogueState>,
    dialogue_query: Query<&DialogueManager>,
    mut next_events: EventWriter<NextDialogueEvent>,
    mut auto_events: EventWriter<ToggleAutoEvent>,
    mut start_events: EventWriter<StartDialogueEvent>,
    mut dialogue_log: ResMut<DialogueLog>,
    mut choice_events: EventWriter<MakeChoiceEvent>,
    mut reset_events: EventWriter<ResetSceneEvent>,
) {
    let manager = dialogue_query.get_single().unwrap();

    egui::Window::new("Dialogue")
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -50.0))
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.set_min_width(600.0);

            if dialogue_state.active {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(&dialogue_state.current_speaker)
                            .heading()
                            .color(egui::Color32::YELLOW),
                    );

                    ui.separator();

                    ui.label(
                        egui::RichText::new(&dialogue_state.current_text)
                            .size(16.0)
                            .color(egui::Color32::WHITE),
                    );

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Next").clicked() {
                            next_events.send(NextDialogueEvent);
                        }

                        let auto_text = if manager.auto_mode {
                            "Auto: ON"
                        } else {
                            "Auto: OFF"
                        };
                        if ui.button(auto_text).clicked() {
                            auto_events.send(ToggleAutoEvent);
                        }

                        if ui.button("Log").clicked() {
                            dialogue_log.show_log = !dialogue_log.show_log;
                        }
                    });
                });
            } else {
                ui.vertical(|ui| {
                    ui.label("Press T to start dialogue or click button below");
                    if ui.button("Start Dialogue").clicked() {
                        start_events.send(StartDialogueEvent {
                            scene_name: "intro".to_string(),
                        });
                    }
                    if ui.button("Reset Scene").clicked() {
                        reset_events.send(ResetSceneEvent);
                    }
                });
            }
        });

    if dialogue_state.active && manager.waiting_for_choice {
        egui::Window::new("Choice")
            .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(210.0, -150.0))
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.set_min_width(150.0);

                ui.vertical(|ui| {
                    for (i, choice) in manager.current_choices.iter().enumerate() {
                        if ui.button(&choice.text).clicked() {
                            choice_events.send(MakeChoiceEvent { choice_index: i });
                        }
                    }
                });
            });
    }
}

pub fn dialogue_log_ui(mut contexts: EguiContexts, dialogue_log: Res<DialogueLog>) {
    if !dialogue_log.show_log {
        return;
    }

    egui::Window::new("Log")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-20.0, 20.0))
        .resizable(true)
        .collapsible(true)
        .default_width(400.0)
        .max_height(500.0)
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for entry in &dialogue_log.entries {
                        match entry {
                            DialogueLogEntry::Line { speaker, text } => {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(speaker)
                                            .strong()
                                            .color(egui::Color32::YELLOW),
                                    );
                                    ui.label(text);
                                });
                            }
                            DialogueLogEntry::Choices {
                                options,
                                selected_index,
                            } => {
                                ui.vertical(|ui| {
                                    for (i, choice) in options.iter().enumerate() {
                                        let choice_text =
                                            choice.player_text.as_ref().unwrap_or(&choice.text);

                                        let text_color = if i == *selected_index {
                                            egui::Color32::WHITE
                                        } else {
                                            egui::Color32::GRAY
                                        };

                                        ui.label(
                                            egui::RichText::new(choice_text)
                                                .color(text_color)
                                                .italics(),
                                        );
                                    }
                                });
                            }
                        }
                        ui.separator();
                    }
                });
        });
}

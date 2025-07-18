use bevy::prelude::*;

use crate::dialogue::{DialogueChoice, DialogueData, DialogueLine};

#[derive(Component)]
pub struct DialogueManager {
    pub original_camera_transform: Option<Transform>,
    pub current_scene: String,
    pub current_line: usize,
    pub dialogue_data: DialogueData,
    pub auto_mode: bool,
    pub waiting_for_choice: bool,
    pub current_choices: Vec<DialogueChoice>,
    pub ephemeral_line: Option<DialogueLine>,
}

#[derive(Component)]
pub struct DialogueTimer(pub Timer);

#[derive(Component)]
pub struct DialogueCamera;

#[derive(Component)]
pub struct Speaker {
    pub name: String,
}

#[derive(Component)]
pub struct DialogueTarget;

#[derive(Component)]
pub struct Actor {
    pub name: String,
}

#[derive(Component)]
pub struct MovementGoal {
    pub target: Vec3,
    pub speed: f32,
}

use bevy::prelude::*;

use crate::dialogue::DialogueTrigger;

#[derive(Event)]
pub struct ResetSceneEvent;

#[derive(Event)]
pub struct StartDialogueEvent {
    pub scene_name: String,
}

#[derive(Event)]
pub struct NextDialogueEvent;

#[derive(Event)]
pub struct ToggleAutoEvent;

#[derive(Event)]
pub struct MakeChoiceEvent {
    pub choice_index: usize,
}

#[derive(Event, Debug)]
pub struct ActionTriggerEvent(pub DialogueTrigger);

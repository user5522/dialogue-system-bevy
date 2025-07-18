pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use events::*;
use serde::{Deserialize, Serialize};
use systems::*;

use bevy::{prelude::*, utils::HashMap};
use bevy_egui::EguiPlugin;

#[derive(Clone)]
pub enum DialogueLogEntry {
    Line {
        speaker: String,
        text: String,
    },
    Choices {
        options: Vec<DialogueChoice>,
        selected_index: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueData {
    pub scenes: HashMap<String, Vec<DialogueLine>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    #[serde(default)]
    pub player_text: Option<String>,
    pub player_text_auto_time: Option<f32>,
    pub next_scene: Option<String>,
    pub next_line: Option<usize>,
    pub triggers: Option<Vec<DialogueTrigger>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
    pub auto_time: Option<f32>,
    pub camera_target: Option<String>,
    pub choices: Option<Vec<DialogueChoice>>,
    pub triggers: Option<Vec<DialogueTrigger>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogueTrigger {
    #[serde(rename = "type")] // renamed to type since type is a rust keyword
    pub trigger_type: String,
    pub target: String,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_event::<StartDialogueEvent>()
            .add_event::<NextDialogueEvent>()
            .add_event::<ToggleAutoEvent>()
            .add_event::<MakeChoiceEvent>()
            .add_event::<ActionTriggerEvent>()
            .add_event::<ResetSceneEvent>()
            .add_systems(Startup, setup_dialogue)
            .add_systems(
                Update,
                (
                    handle_start_dialogue,
                    handle_next_dialogue,
                    handle_choice,
                    handle_input,
                    handle_toggle_auto,
                    dialogue_ui,
                    dialogue_log_ui,
                    handle_auto_dialogue,
                    handle_reset_scene,
                ),
            )
            .add_systems(Update, (handle_move_to_trigger, process_movement));
    }
}

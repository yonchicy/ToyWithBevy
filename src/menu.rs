use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin};
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
            // or after the `EguiSet::BeginPass` system (which belongs to the `CoreSet::PreUpdate` set).
            .add_systems(Update, ui_example_system);
    }
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

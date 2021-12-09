use std::collections::BTreeSet;

use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct TemplateApp {
    code: String,
    displayed_output: String,
    output_history: Vec<HistoryItem>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HistoryItem{
    code: String,
    output: String,
}
// impl Default for TemplateApp {
//     fn default() -> Self {
//         Self {
//             // Example stuff:
//             label: "Hello World!".to_owned(),
// 	    code: "".to_owned(),
//             value: 2.7,
//         }
//     }
// }

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("history").show(ctx, |ui| {
            ui.heading("History");
	    for (ind, hist_item) in self.output_history.iter().enumerate() {
		ui.collapsing(
		    format!("({}) {}", ind, hist_item.code),
		    |ui| { ui.label(&hist_item.output); }
		);
		ui.separator();
	    }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Your code here:");
	    ui.code_editor(&mut self.code);
	    let prev = self.output_history.first().cloned();
	    if ui.button("Execute").clicked() {
		let output = "output".to_string();
		if let Some(prev_item) = prev {
		    if self.code != prev_item.code{
			self.output_history
			    .insert(0,
				    HistoryItem{ code: self.code.clone(), output: output.clone() }
			);
		    }
		} else {
		    self.output_history
			.insert(0,
				HistoryItem{ code: self.code.clone(), output: output.clone() }
			);
		}
		self.displayed_output = self.code.clone();
		ui.monospace(output);
	    }
	});
    }
}

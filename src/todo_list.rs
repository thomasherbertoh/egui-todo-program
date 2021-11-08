use serde_json::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Debug)]
pub struct TodoList {
	todos: Vec<String>,
	add_todo: String,
}

impl Default for TodoList {
	fn default() -> Self {
		TodoList::load_from_json()
	}
}

impl epi::App for TodoList {
	fn name(&self) -> &str {
		"simple todolist app"
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
		// let Self { todos, add_todo } = self;

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				egui::menu::menu(ui, "File", |ui| {
					if ui.button("Quit").clicked() {
						self.save_to_json().unwrap();
						frame.quit();
					}
				});
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("todo list");
			for (i, todo) in self.todos.clone().iter().enumerate() {
				ui.horizontal(|ui| {
					ui.label(todo);
					if ui.button("remove todo").clicked() {
						self.todos.remove(i);
						self.save_to_json().unwrap();
					}
				});
			}
			egui::warn_if_debug_build(ui);
		});

		egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Add a todo");
				ui.text_edit_singleline(&mut self.add_todo);
				if ui.button("Add todo").clicked() && self.add_todo.len() != 0 {
					self.todos.push(self.add_todo.clone());
					self.save_to_json().unwrap();
					self.add_todo.clear();
				}
			});
		});
	}
}

impl TodoList {
	fn save_to_json(&self) -> Result<(), Error> {
		let serialised = serde_json::to_string_pretty(&self.todos)?;
		std::fs::write("todos.json", serialised).unwrap();
		Ok(())
	}

	fn load_from_json() -> Self {
		let file = File::open("todos.json");

		match file {
			Ok(file) => {
				let reader = BufReader::new(file);
				let todos: Vec<String> = serde_json::from_reader(reader).unwrap_or(vec![]);
				Self {
					todos: todos,
					add_todo: String::new(),
				}
			}
			Err(_) => Self {
				todos: Vec::new(),
				add_todo: String::new(),
			},
		}
	}
}

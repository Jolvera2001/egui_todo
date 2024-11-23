use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
            ..Default::default()
    };

    eframe::run_native(
        "My Todo List",
        options, 
        Box::new(|cc| {
            Ok(Box::new(TodoApp::default()))
        }),
    )
}

struct TodoApp {
    todos: Vec<Todo>,
    new_todo_input: String,
}

struct Todo {
    content: String,
    is_complete: bool,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            todos: Vec::new(),
            new_todo_input: String::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_todo_input)
                    .changed();
                if ui.button("Add to List").clicked() 
                    || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.new_todo_input.trim().is_empty() {
                        let new_todo = Todo {
                            content: self.new_todo_input.clone(),
                            is_complete: false,
                        };
                        self.todos.push(new_todo);
                        self.new_todo_input.clear();
                    }
                }
            });
            ui.add_space(8.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for todo in &mut self.todos {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut todo.is_complete, "");
                        if todo.is_complete {
                            ui.label(egui::RichText::new(&todo.content).strikethrough());
                        } else {
                            ui.label(&todo.content);      
                        }
                    });
               }; 
            });

            ui.add_space(8.0);
            ui.label(format!("Total todos: {}", self.todos.len()));
        });
    }
}
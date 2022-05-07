mod todo_list;

fn main() {
    let app = todo_list::TodoList::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Todo List", native_options, Box::new(|_| Box::new(app)));
}

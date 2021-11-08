mod todo_list;

fn main() {
    let app = todo_list::TodoList::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

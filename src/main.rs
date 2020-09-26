use cursive::views::{Button, Checkbox, Dialog, LinearLayout, TextArea, TextView};
use cursive::Cursive;
use std::collections::HashMap;

// Include Symbols:( e.g. @#$% )
// Include Numbers:( e.g. 123456 )
// Include Lowercase Characters:( e.g. abcdefgh )
// Include Uppercase Characters:( e.g. ABCDEFGH )
// Exclude Similar Characters:( e.g. i, l, 1, L, o, 0, O )
// Exclude Ambiguous Characters:( { } [ ] ( ) / \ ' " ` ~ , ; : . < > )

fn main() {
    let mut siv = cursive::default();

    let options_state: HashMap<&str, bool> = [
        ("include_symbols", true),
        ("include_numbers", true),
        ("include_lowercase", true),
        ("include_uppercase", true),
        ("exclude_similar", true),
        ("exclude_ambiguous", true),
    ]
    .iter()
    .cloned()
    .collect();

    siv.set_user_data::<HashMap<&str, bool>>(options_state.clone());

    fn handle_check(key: &'static str) -> Box<dyn Fn(&mut Cursive, bool)> {
        Box::new(move |cursive: &mut Cursive, state: bool| -> () {
            cursive.with_user_data(|data: &mut HashMap<&str, bool>| -> () {
                data.insert(key, state);
            });
        })
    };

    // generate column of checkboxes from state keys.
    let mut buttons = LinearLayout::vertical();
    let checkboxes = &options_state.keys().map(|key| {
        Checkbox::new()
            .with_checked(true)
            .on_change(handle_check(key))
    });
    checkboxes.clone().for_each(|c| buttons.add_child(c));

    // generate column of labels for checkboxes.
    let labels = LinearLayout::vertical()
        .child(TextView::new("Include Symbols:"))
        .child(TextView::new("Include Numbers:"))
        .child(TextView::new("Include Lowercase Characters:"))
        .child(TextView::new("Include Uppercase Characters:"))
        .child(TextView::new("Exclude Similar Characters:"))
        .child(TextView::new("Include Ambiguous Characters:"));

    let top_row = LinearLayout::horizontal().child(labels).child(buttons);

    let layout = LinearLayout::vertical()
        .child(top_row)
        .child(Button::new("Quit", |s| s.quit()));

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(layout).title("Pukka - Password Generator"));

    // Starts the event loop.
    siv.run();
}

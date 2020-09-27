use cursive::views::{Button, Checkbox, Dialog, LinearLayout, TextContent, TextView};
use cursive::Cursive;
use std::collections::HashMap;

mod password;

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

    // TODO - default width, replace with multiline
    let password_result_field = TextContent::new("");

    fn handle_check(key: &'static str) -> Box<dyn Fn(&mut Cursive, bool)> {
        Box::new(move |cursive: &mut Cursive, state: bool| -> () {
            cursive.with_user_data(|data: &mut HashMap<&str, bool>| -> () {
                data.insert(key, state);
            });
        })
    };

    // TODO - copy to clipboard on generate
    fn handle_generate(result_field: TextContent) -> Box<dyn Fn(&mut Cursive)> {
        Box::new(
            move |cursive: &mut Cursive| -> () {
                let data = cursive.user_data::<HashMap<&str, bool>>().expect("Error");
                let pw = password::from_options(data);
                result_field.set_content(pw);
            }
        )
    }

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

    let checkboxes_row = LinearLayout::horizontal().child(labels).child(buttons);

    let password_row = LinearLayout::horizontal()
        .child(TextView::new_with_content(password_result_field.clone()))
        .child(Button::new("Generate", handle_generate(password_result_field)));


    let buttons_row = LinearLayout::horizontal()
        .child(Button::new("Quit", |s| s.quit()));

    let layout = LinearLayout::vertical()
        .child(checkboxes_row)
        .child(password_row)
        .child(buttons_row);

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(layout).title("Pukka - Password Generator"));

    // Starts the event loop.
    siv.run();
}

extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView, Button, DummyView, EditView, LinearLayout, SelectView, BoxView};
use cursive::traits::*;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());


    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_id("select")
        .fixed_size((10,5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(select)
        .child(DummyView)
        .child(buttons))
        .title("select a profile"));
    siv.run()
}



fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_id("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        }).unwrap()
    }


    s.add_layer(Dialog::around(EditView::new()
        .with_id("name")
        .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name = s.call_on_id("name", |view: &mut EditView| {
                view.get_content()
            }).unwrap();
            ok(s, &name);
            // what do we do now
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }))
}
fn delete_name(s:&mut Cursive) {
    let mut select = s.find_id::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &String) {
    println!("Submited");
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {} \nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit))
}

fn show_next(s: &mut Cursive){
    s.pop_layer();
    s.add_layer(Dialog::text("Are you Gay? 😏")
        .title("Question 1")
        .button("Yes", |s| show_answer(s, "I Knew It Lmao 😉"))
        .button("No!", |s| show_answer(s, "Oh I meant as in like happy,👀 are you happy? 😅"))
        .button("Uh?", |s| s.add_layer(Dialog::text("try again"))));

}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
                    .title("Results")
                    .button("Finish", |s| s.quit()))
}
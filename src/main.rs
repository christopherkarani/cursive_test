extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("Hello Christopher")
        .title("Hello World")
        .button("Quit", |s|show_next(s)));



    siv.run()
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
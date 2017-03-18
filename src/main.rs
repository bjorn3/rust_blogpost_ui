extern crate cursive;
#[macro_use]
extern crate lazy_static;

extern crate reqwest;
extern crate scraper;

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod post;
pub mod blogs;

use std::sync::Mutex;

use cursive::Cursive;
use cursive::views::*;

use post::Post;

lazy_static! {
    static ref POSTS: Mutex<Vec<Post>> = Mutex::new(Vec::new());
}

fn main() {
    POSTS.lock().unwrap().extend(blogs::manishearth::extract());
    POSTS.lock().unwrap().extend(blogs::steveklabnik::extract());

    println!("{:#?}", *POSTS.lock().unwrap());

    let mut siv = Cursive::new();
    let root_screen_id = siv.add_active_screen();

    let mut posts_view = SelectView::new()
        .on_submit(|s, &i| {
            if i == 0 {
                s.quit();
            }
            s.add_active_screen();
            let text_view = TextView::new(POSTS.lock().unwrap()[(i-1) as usize].content.clone());
            s.add_fullscreen_layer(text_view);
        });
    posts_view.add_item("Quit", 0);
    for (i, post) in POSTS.lock().unwrap().iter().enumerate() {
        posts_view.add_item(post.title.clone(), i+1);
    }

    siv.add_fullscreen_layer(posts_view);

    siv.add_global_callback('q', move |s| s.set_screen(root_screen_id));

    siv.run();
}

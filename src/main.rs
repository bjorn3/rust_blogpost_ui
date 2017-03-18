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
use cursive::views::{TextView, ListView, SelectView};

use post::Post;

lazy_static! {
    static ref POSTS: Mutex<Vec<Post>> = Mutex::new(Vec::new());
}

fn main() {
    POSTS.lock().unwrap().extend(blogs::manishearth::extract());
    //posts.extend(blogs::steveklabnik::extract());

    println!("{:#?}", *POSTS.lock().unwrap());

    let mut siv = Cursive::new();

    let mut posts_view = SelectView::new()
        .on_submit(|s, &i| {
            let screen = s.add_active_screen();
            s.add_fullscreen_layer(TextView::new(POSTS.lock().unwrap()[i as usize].content.clone()));
        });
    for (i, post) in POSTS.lock().unwrap().iter().enumerate() {
        posts_view.add_item(post.title.clone(), i);
    }

    siv.add_fullscreen_layer(posts_view);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

extern crate cursive;
extern crate reqwest;
extern crate scraper;

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod post;
pub mod blogs;

use cursive::Cursive;
use cursive::views::{TextView, ListView, SelectView};

fn main() {
    let mut posts = Vec::new();
    posts.extend(blogs::manishearth::extract());
    //posts.extend(blogs::steveklabnik::extract());

    println!("{:#?}", posts);

    let mut siv = Cursive::new();

    let mut posts_view = SelectView::new()
        .on_submit(|s, post: &post::Post| {
            let screen = s.add_active_screen();
            s.add_fullscreen_layer(TextView::new(post.content.clone()));
        });
    for (i, post) in posts.iter().enumerate() {
        posts_view.add_item(post.title.clone(), post.clone());
    }

    siv.add_fullscreen_layer(posts_view);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

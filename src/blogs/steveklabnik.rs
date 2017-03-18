//! http://words.steveklabnik.com/

use std::io::Read;

use reqwest;
use scraper::{Html, Selector};

use post::Post;

pub fn extract() -> Vec<Post> {
    let mut posts = Vec::new();

    for i in 0..3 {
        let mut resp = reqwest::get(&format!("http://words.steveklabnik.com/page/{}", i)).unwrap();
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();
        let document = Html::parse_document(&body);
        
        let article_sel = Selector::parse("article.post.user_show").unwrap();
        let title_sel = Selector::parse("h1.article_title a").unwrap();
        let content_sel = Selector::parse("article.post").unwrap();

        for element in document.select(&article_sel).skip(1) {
            let title_el = element.select(&title_sel).next().unwrap();
            let title = title_el.text().next().unwrap();
            let url = title_el.value().attr("href").unwrap();

            let mut content_resp = reqwest::get(&format!("http://manishearth.github.io{}", url)).unwrap();
            let mut body = String::new();
            content_resp.read_to_string(&mut body).unwrap();
            let content_document = Html::parse_document(&body);

            let content = content_document.select(&content_sel).next().unwrap().inner_html();

            println!("{}: {} ({})", title, url, content.chars().take(20).collect::<String>());

            posts.push(Post{
                url: url.to_string(),
                title: title.to_string(),
                content: content
            })
        }
    }

    posts
}

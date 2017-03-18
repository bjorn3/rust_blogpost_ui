//! http://words.steveklabnik.com/

use std::io::Read;

use reqwest;
use scraper::{Html, Selector};

use post::Post;

pub fn extract() -> Vec<Post> {
    let mut posts = Vec::new();

    for i in 1..20 {
    //for i in 1..2 {
        println!("Page {}", i);
        let mut resp = reqwest::get(&format!("http://words.steveklabnik.com/page/{}", i)).unwrap();
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();
        let document = Html::parse_document(&body);
        
        let article_sel = Selector::parse("article.post.user_show").unwrap();
        let title_sel = Selector::parse("h1.article_title a").unwrap();
        let content_sel = Selector::parse("article.post").unwrap();

        for element in document.select(&article_sel) {
            let title_el = element.select(&title_sel).next().unwrap();
            let title = title_el.text().next().unwrap();
            let url = title_el.value().attr("href").unwrap();

            let mut content_resp = reqwest::get(&format!("http:{}", url)).unwrap();
            let mut body = String::new();
            content_resp.read_to_string(&mut body).unwrap();
            let content_document = Html::parse_document(&body);

            let content_el = content_document.select(&content_sel).next().unwrap();
            let content = content_el.inner_html();

            println!("steveklabnik {}: {} ({})", title, url, content.chars().take(20).collect::<String>());

            posts.push(Post{
                url: url.to_string(),
                title: title.to_string(),
                content: content
            })
        }
    }

    posts
}

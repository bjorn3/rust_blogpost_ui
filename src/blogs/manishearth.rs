//! http://manishearth.github.io/

use std::io::Read;

use reqwest;
use scraper::{Html, Selector};

use post::Post;

pub fn extract() -> Vec<Post> {
    let mut resp = reqwest::get("http://manishearth.github.io/blog/archives/").unwrap();
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();
    let document = Html::parse_document(&body);
    
    let article_sel = Selector::parse("article").unwrap();
    let title_sel = Selector::parse("h1 a").unwrap();
    let content_sel = Selector::parse("div.entry-content").unwrap();

    let mut posts = Vec::new();

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

    posts
}

/*pub fn extract() -> Vec<Post> {
    #[derive(Deserialize, Debug)]
    struct GithubContentDir {
        name: String,
        download_url: String,
    }

    let mut posts = Vec::new();

    let client = reqwest::Client::new().unwrap();

    let mut all_posts: Vec<GithubContentDir> =
        client.get("https://api.github.com/repos/manishearth/manishearth.github.io/contents/source/_posts?ref=source")
        .send().unwrap().json().unwrap();
    
    for post in all_posts {
        let resp = client.get(post.download_url).send().unwrap();
        let mut body = String::new();
        resp.read_to_string(&mut body);
    }

    println!("{:#?}", all_posts);

    posts

}*/
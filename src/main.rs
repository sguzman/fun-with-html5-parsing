extern crate reqwest;
extern crate select;

static URL: &str = "http://23.95.221.108/page/2";

fn main() {
    let body = reqwest::get(URL).unwrap().text().unwrap();
    let doc = select::document::Document::from(body.as_str());
    let sel = doc.find(select::predicate::Name("article"));

    for node in sel {
        let id = node.attr("id").unwrap();
        println!("{} ({})", node.name().unwrap(), id);
    }
}

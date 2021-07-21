use markdown::to_html;
use markdown_parser::{parse_format, Format};
use std::fs;
use yaml_rust::{YamlLoader};

pub fn parse_markdown() {
    let content = fs::read_to_string("./assets/test.md").unwrap();
    let md = parse_format(&content, Format::YAML).unwrap();
    // 取到了metadata和md正文
    let markdown_source = md.content();
    let metadata = md.front_matter();
    let html = to_html(markdown_source);
    println!("\nhtml:\n{}", html);
    let data = YamlLoader::load_from_str(metadata).unwrap();
    println!("\ndata:\n{:?}", data);
}

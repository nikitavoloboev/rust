#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Result;
use markdown::{mdast::Node, to_mdast, ParseOptions};
use std::{collections::VecDeque, net::SocketAddr};

mod test;
mod wiki;

// TODO: &'a looks ugly, is there a better way?
#[derive(Debug, PartialEq)]
pub struct TopicStruct {
    title: String,
    content: String,
}

// parse markdown file, extract topic
pub fn parse_md_content_as_topic<'a>(markdown_string: &'a str) -> Result<TopicStruct> {
    let options = ParseOptions::default();
    let ast = to_mdast(markdown_string, &options).map_err(anyhow::Error::msg)?;
    let mut nodes = VecDeque::new();
    nodes.push_back(ast);

    let mut title = None;
    let mut content = String::new();
    let mut collecting_content = false;

    while let Some(node) = nodes.pop_front() {
        match node {
            Node::Heading(ref heading) => {
                if title.is_none() {
                    title = Some(
                        heading
                            .children
                            .iter()
                            .map(|child| match child {
                                Node::Text(text) => text.value.clone(),
                                _ => String::new(),
                            })
                            .collect::<Vec<String>>()
                            .join(" "),
                    );
                    collecting_content = true;
                } else {
                    // Found another heading, we can stop collecting content
                    collecting_content = false;
                }
            }
            Node::Paragraph(ref para) => {
                if collecting_content {
                    content.push_str(
                        &para
                            .children
                            .iter()
                            .map(|child| match child {
                                Node::Text(text) => text.value.clone(),
                                Node::Link(link) => link.url.clone(),
                                _ => String::new(),
                            })
                            .collect::<Vec<String>>()
                            .join(" "),
                    );
                    content.push('\n');
                }
            }
            _ => {}
        }

        if let Some(children) = node.children() {
            for child in children.iter().cloned() {
                nodes.push_back(child);
            }
        }
    }

    let title_str = title.ok_or_else(|| anyhow::Error::msg("Failed to extract title"))?;
    Ok(TopicStruct {
        title: title_str,
        content: content.trim().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        test::get_test_folder_path,
        wiki::{get_content_of_file, get_md_files},
    };

    use super::*;

    #[test]
    fn test_md_file_with_heading_and_content_only() {
        let test_folder_path = get_test_folder_path();
        println!("{}", test_folder_path.display());

        // Attempt to convert to a str, will return None if the path is not valid UTF-8
        if let Some(path_str) = test_folder_path.to_str() {
            let paths = get_md_files(path_str);
            if !paths.is_empty() {
                let content = get_content_of_file(&paths[0]);
                let topic = parse_md_content_as_topic(&content).unwrap();
                assert_eq!(
                    topic,
                    TopicStruct {
                        title: "Hardware".to_string(),
                        content: "[Digital Design and Computer Architecture course](https://safari.ethz.ch/digitaltechnik/spring2021/doku.php?id=start), [From Nand to Tetris](https://github.com/ghaiklor/nand-2-tetris) are great.".to_string()
                    }
                );
            }
        } else {
            println!("Path is not valid UTF-8");
        }
    }
}

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Result;
use markdown::{to_mdast, ParseOptions};

mod test;
mod wiki;

// TODO: &'a looks ugly, is there a better way?
#[derive(Debug, PartialEq)]
pub struct TopicStruct<'a> {
    title: &'a str,
    content: &'a str,
}

// parse markdown file, extract topic
pub fn parse_md_content_as_topic<'a>(markdown_string: &'a str) -> Result<TopicStruct<'a>> {
    // let options = ParseOptions::default();
    // let tree = to_mdast(markdown_string, &ParseOptions::default())?;
    // println!("{:?}", tree);

    let title = "Physics";
    let content = "Physics is great";
    Ok(TopicStruct { title, content })
}

#[cfg(test)]
mod tests {
    use crate::{
        test::get_test_folder_path,
        wiki::{get_content_of_file, get_md_files},
    };

    use super::*;

    #[test]
    fn test_md_file_with_heading_and_content() {
        let test_folder_path = get_test_folder_path();

        // Attempt to convert to a str, will return None if the path is not valid UTF-8
        if let Some(path_str) = test_folder_path.to_str() {
            let paths = get_md_files(path_str);
            if !paths.is_empty() {
                let content = get_content_of_file(&paths[0]);
                let topic = parse_md_content_as_topic(&content);
                println!("{}", "runs");
                // assert_eq!(
                //     topic,
                //     TopicStruct {
                //         title: "Physics",
                //         content: "Physics is great."
                //     }
                // );
            }
        } else {
            println!("Path is not valid UTF-8");
        }
    }
}

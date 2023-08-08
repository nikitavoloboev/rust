#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Result;
use markdown::{to_mdast, ParseOptions};

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
    use crate::test::get_test_folder_path;
    use crate::wiki::{get_content_of_file, get_md_files};

    use super::*; // Bring the parent module into scope

    #[test]
    fn test_md_file_with_heading_and_content() {
        let test_folder_path = get_test_folder_path();

        // Attempt to convert to a str, will return None if the path is not valid UTF-8
        if let Some(path_str) = test_folder_path.to_str() {
            let paths = get_md_files(path_str);
            if !paths.is_empty() {
                let content = get_content_of_file(&paths[0]);
                let topic = parse_md_content_as_topic(&content);
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
    // fn test_file_with_front_matter_heading_content_notes_links() {
}

// #[test]
// fn test_file_with_front_matter_heading_content_notes_links() {
//     // const testFolderPath = getTestFolderPath()
//     // const filePaths = await markdownFilePaths(testFolderPath)
//     // const solidTestFile = filePaths[2] // solid.md file in seed/wiki/test folder
//     // const solidTopic = await parseMdFile(solidTestFile)
//     // expect(solidTopic!.topicName).toBe("solid")
//     // expect(solidTopic!.prettyTopicName).toBe("SolidJS")

//     get_test_folder_path();

//     assert_eq!("..", ".");
// }

// pub async fn markdown_file_paths(
//     directory_path: &str,
//     ignore_list: Vec<String>,
// ) -> io::Result<Vec<String>> {
//     let mut files_to_process = vec![];
//     let entries = fs::read_dir(directory_path)?;

//     for entry in entries {
//         let entry = entry?;
//         let path = entry.path();
//         let file_name = entry
//             .file_name()
//             .to_string_lossy()
//             .to_string()
//             .to_lowercase();
//         let full_path = format!("{}", path.display());

//         if path.is_dir() {
//             let sub_dir_files = markdown_file_paths(&full_path, ignore_list.clone()).await?;
//             files_to_process.extend(sub_dir_files);
//         } else if path.is_file()
//             && path.extension() == Some(std::ffi::OsStr::new("md"))
//             && !ignore_list.contains(&file_name)
//         {
//             files_to_process.push(full_path);
//         }
//     }
//     Ok(files_to_process)
// }

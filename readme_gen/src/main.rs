use md_gen::md_gen::markdown::Markdown;
use std::collections::HashMap;
use std::{fs, io};

fn md_link_gen(title: &str, link: &str) -> String {
    return format!("[{}]({})", title, link);
}

fn main() -> io::Result<()> {
    let mut hash_map: HashMap<_, Vec<_>> = HashMap::new();

    let groups = fs::read_dir("./")?
        .map(|entries| entries.map(|e| e.path()))
        .into_iter()
        .filter(|e| e.as_ref().unwrap().to_str().unwrap().starts_with("./_"))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for group in groups.iter() {
        if group.is_dir() {
            let questions = fs::read_dir(group)?
                .map(|entries| entries.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?;
            hash_map.insert(group, questions);
        }
    }

    let mut md = Markdown::new("README.md");

    md.add_main_heading("leetcode-records-by-rust");
    md.add_text("🐒 recording leetcode answer by rust for me.");
    md.add_table_headers(vec!["groups", "questions"]);
    for (group, mut questions) in hash_map.into_iter() {
        let group_cell = md_link_gen(
            group.to_str().unwrap().get(2..).unwrap(),
            group.to_str().unwrap().get(2..).unwrap(),
        );
        let mut questions_cell = vec![];
        questions.sort_by(|a, b| {
            let title_a = a
                .to_str()
                .unwrap()
                .get(group.to_str().unwrap().len() + 2..)
                .unwrap();
            let title_b = b
                .to_str()
                .unwrap()
                .get(group.to_str().unwrap().len() + 2..)
                .unwrap();

            title_a
                .get(..title_a.find('_').unwrap_or(0))
                .unwrap()
                .parse::<i32>()
                .unwrap()
                .cmp(
                    &(title_b
                        .get(..title_b.find('_').unwrap_or(0))
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()),
                )
        });
        for question in questions.iter() {
            let title = question
                .to_str()
                .unwrap()
                .get(group.to_str().unwrap().len() + 2..)
                .unwrap();
            questions_cell.push(md_link_gen(
                title.get(..title.find('_').unwrap_or(0)).unwrap(),
                question.to_str().unwrap().get(2..).unwrap(),
            ));
        }
        md.add_table_row(vec![&group_cell, &questions_cell.join(", ")]);
    }
    md.write()
}

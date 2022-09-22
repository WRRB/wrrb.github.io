//bin/true; rustc -o "/tmp/$0.bin" 1>&2 "$0" && "/tmp/$0.bin" "$@"; exit $?

use std::io::Write;
use std::{assert, env, fs, process};

static INDEX: &str = "index.md";

fn main() {
    assert!(
        root_is_valid(),
        "You are running this in the wrong folder, only wrrb.github.io is allowed"
    );
    clean_docs();
    let posts = list_posts();
    let written_posts = create_tmp_index(posts);
    run_pandoc(written_posts);
    delete_tmp_index();
}

struct PostName {
    date: String,
    title: String,
}

fn clean_docs(){
    let path = "docs/";
    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();
}

fn root_is_valid() -> bool {
    env::current_dir()
        .expect("Something went wrong getting the current dir")
        .display()
        .to_string()
        .contains(&String::from("wrrb.github.io"))
}

fn list_posts() -> Vec<PostName> {
    let mut post_names: Vec<PostName> = Vec::new();
    for file in fs::read_dir(".").unwrap() {
        let post_name = file.unwrap().path().display().to_string();
        if post_name.contains(&String::from(".md"))
            && !post_name.contains(&String::from("nav"))
            && !post_name.contains(&String::from("index"))
        {
            let und = String::from("_");
            let mut split = post_name.split(&und);
            let pn = PostName {
                date: split.next().unwrap().to_string(),
                title: split.next().unwrap().to_string(),
            };
            post_names.push(pn);
        }
    }
    post_names
}

fn create_tmp_index(posts: Vec<PostName>) -> Vec<PostName>{
    let mut file = fs::File::create(INDEX).expect("Cannot create tmp index");
    writeln!(&mut file, "### Wim Berchmans").unwrap();
    writeln!(&mut file, "> \"Words are wind\"").unwrap();
    writeln!(&mut file, "").unwrap();
    for post in &posts {
        writeln!(&mut file, "{}\n", format_link(post)).unwrap();
    }
    posts
}

fn format_link(post: &PostName) -> String {
    format!(
        "[{} {}]({}_{})",
        post.date,
        post.title.replace("-", " ").replace(".md", ""),
        post.date,
        post.title.replace(".md", ".html")
    )
}

fn run_pandoc(posts: Vec<PostName>) {
    let output = process::Command::new("pandoc")
        .arg(INDEX)
        .arg("-s")
        .arg("-o")
        .arg("docs/index.html")
        .output()
        .expect("Couldn't convert index");
    println!("{:?}", output);
    for post in posts {
        let output_post = process::Command::new("pandoc")
            .arg("nav.md")
            .arg(format!("{}_{}", post.date, post.title))
            .arg("-s")
            .arg("-o")
            .arg(format!("docs/{}_{}", post.date, post.title.replace(".md", ".html")))
            .output()
            .expect("Couldn't convert index");
        println!("{:?}", output_post);
    }
}

fn delete_tmp_index() {
    fs::remove_file(INDEX).expect("Unable to remove index file")
}

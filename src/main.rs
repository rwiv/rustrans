use rustrans::utils::path;
use rustrans::utils::file;
use anyhow::Result;
use rustrans::translator::Translator;
use rustrans::translator::deepl::DeeplClient;

#[tokio::main]
async fn main() {
    let result = run().await;
    if let Err(err) = result {
        eprintln!("{:?}", err);
    }
}

async fn run() -> Result<()> {
    let project_root = path::get_project_root_path();
    let file_path = project_root.join("tests").join("test1.txt");
    let strings = file::read_lines(&file_path)?;
    let targets = strings.iter().map(|s| s.as_str()).collect();

    let client = DeeplClient {};
    let translator = Translator{ client };
    let translated = translator.translate(&targets, 20).await;

    let mut result = Vec::new();
    for (before, after) in translated {
        result.push(format!("{}\n{}", before, after));
    }

    let new_file_path = project_root.join("tests").join("test2.txt");
    file::write_lines(&result, &new_file_path)?;
    Ok(())
}

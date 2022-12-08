use {{project-name}}::*;

#[tokio::main]
async fn launch() -> anyhow::Result<()> {
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let _app = app::Application::hi();

    launch()
}

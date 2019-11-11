use log::info;
use std::process::Command;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

#[tokio::test]
async fn run_postman() {
    pretty_env_logger::init();

    info!("starting the test");
    let (tx, rx) = oneshot::channel();
    let runtime = Runtime::new().unwrap();
    let executor = runtime.executor();
    executor.spawn(async {
        // let _ = server::serve(7171, rx).await.expect("Error running the server");
        let _ = server::serve(7171).await.expect("Error running the server");
    });

    // TODO: use a channel to communicate that the server is ready rather than waiting
    std::thread::yield_now();
    std::thread::sleep(std::time::Duration::from_millis(25));

    info!("starting the postman collection test suite");
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir("./tests")
            .args(&["/D", "/C"])
            .arg("newman run test-suite.postman_collection.json -e test-default-env.postman_environment.json")
            .status()
            .expect("\n\nFailed to execute newman. Ensure newman is installed properly (npm -i -g newman)\n\n")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg("newman run test-suite.postman_collection.json -e test-default-env.postman_environment.json")
        .status()
        .expect("\n\nFailed to execute newman. Ensure newman is installed properly (npm -i -g newman)\n\n")
    };

    let _ = tx.send(());
    assert!(status.success())
}

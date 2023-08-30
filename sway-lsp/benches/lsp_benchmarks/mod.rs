pub mod compile;
pub mod requests;
pub mod token_map;

use lsp_types::Url;
use std::path::PathBuf;
use sway_lsp::core::session::{self, Session};

pub fn compile_test_project() -> (Url, Session) {
    let mut session = Session::new();
    // Load the test project
    let uri = Url::from_file_path(benchmark_dir().join("src/main.sw")).unwrap();
    session.handle_open_file(&uri);
    // Compile the project and write the parse result to the session
    let parse_result = session::parse_project(&uri).unwrap();
    session.write_parse_result(parse_result);
    (uri, session)
}

pub fn sway_workspace_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn benchmark_dir() -> PathBuf {
    sway_workspace_dir().join("sway-lsp/tests/fixtures/benchmark")
}

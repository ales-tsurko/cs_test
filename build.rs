use ethers_solc::{Project, ProjectPathsConfig};

fn main() {
    let project = Project::builder()
        .paths(
            ProjectPathsConfig::hardhat(format!("{}/solidity", env!("CARGO_MANIFEST_DIR")))
                .unwrap(),
        )
        .build()
        .unwrap();
    let _output = project.compile().unwrap();

    project.rerun_if_sources_changed();
}

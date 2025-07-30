
#[derive(Debug)]
pub struct CliCommand {
    command: String,
    model: String,
    path_to_file: String
}

impl CliCommand {
    pub fn new(command: String, model: String, path_to_file: String) -> Self {
        CliCommand { command, model, path_to_file }
    }
}






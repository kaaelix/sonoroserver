#[derive(Debug, Default)]
pub struct GmCommandOutput {
    pub capture: bool,
    pub output: String,
}

impl GmCommandOutput {
    pub fn record(&mut self, content: &str) {
        if self.capture {
            if !self.output.is_empty() {
                self.output.push('\n');
            }
            self.output.push_str(content);
            return;
        }
        tracing::info!(target: "gm", "{content}");
    }
}

pub fn chat_command_response(state: &mut GmCommandOutput, content: &str) {
    state.record(content);
}

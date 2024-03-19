mod chat_message;
pub mod cli_config;
pub mod error;
pub mod kafka;

pub mod utils {
    pub fn clean_line(line: &mut String) {
        *line = line
            .trim_end_matches('\n')
            .trim_end_matches('\r')
            .trim_start()
            .to_string();
    }
}

use super::io_utils::{read_lines};

pub struct Config {
    mode: String,
    coordinator_info: String,
    participant_info: Vec<String>,
}

impl Config {
    pub fn mode(&self) -> &String { &self.mode }
    pub fn coordinator_info(&self) -> &String { &self.coordinator_info }
    pub fn participant_info(&self) -> &Vec<String> { &self.participant_info }
    pub fn new(path: &str) -> Self {
        let mut mode = String::new();
        let mut coordinator_info = String::new();
        let mut participant_info = Vec::new();
        if let Ok(lines) = read_lines(path) {
            for line in lines {
                match line {
                    Ok(res) => {
                        if res.starts_with("mode") {
                            mode = res.replace("mode ", "");
                        } else if res.starts_with("coordinator_info") {
                            coordinator_info = res.replace("coordinator_info ", "").trim().to_string();
                        } else if res.starts_with("participant_info") {
                            participant_info.push(res.replace("participant_info ", "").trim().to_string());
                        };
                    }
                    Err(_) => ()
                }
            };
        };
        Config {
            mode,
            coordinator_info,
            participant_info,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Config;

    #[test]
    fn config_analysis() {
        let conf = Config::new("./src/participant.conf");
        assert_eq!(conf.mode, "participant");
        assert_eq!(conf.coordinator_info, "127.0.0.1:8001");
        match conf.participant_info.get(0) {
            Some(x) => assert_eq!(x, "127.0.0.1:8002"),
            None => ()
        }
    }
}
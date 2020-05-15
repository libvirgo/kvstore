pub struct Protocol {
    method: String,
    key: Vec<String>,
    value: String,
}

impl Protocol {
    pub fn method(&self) -> &String { &self.method }
    pub fn key(&self) -> &Vec<String> { &self.key }
    pub fn value(&self) -> &String { &self.value }
    pub fn new(string: &str) -> Self {
        let method;
        let mut key = Vec::new();
        let mut value;
        let mut iter = string.split_whitespace();
        method = iter.next().unwrap_or_default().to_uppercase();
        if method.eq(&"DEL".to_string()) {
            for k in iter {
                key.push(k.to_string());
            };
            value = String::new();
        } else if method.eq(&"GET".to_string()) {
            key.push(iter.next().unwrap_or_default().to_string());
            value = String::new();
        } else {
            key.push(iter.next().unwrap_or_default().to_string());
            value = String::new();
            for str in iter {
                value.push_str(str);
                value.push(' ');
            };
            value = value.trim().to_string();
        };
        Protocol {
            method,
            key,
            value,
        }
    }
    pub fn to_resp(&self) -> String {
        let mut res: String = String::new();
        res.push_str("*");
        if self.method.eq(&"SET".to_string()) {
            res.push_str((self.value.split_whitespace().count() + 2).to_string().as_str());
            res.push_str("\r\n$3\r\nSET\r\n$");
            res.push_str(add_str(self.key.get(0)).as_str());
            let mut str = self.value.replace("'","");
            str = str.replace("\"","");
            for value in str.split_whitespace() {
                res.push('$');
                res.push_str(value.len().to_string().as_str());
                res.push_str("\r\n");
                res.push_str(value);
                res.push_str("\r\n");
            }
        };
        if self.method.eq(&"GET".to_string()) {
            res.push_str("2\r\n$3\r\nGET\r\n$");
            res.push_str(add_str(self.key.get(0)).as_str());
        };
        if self.method.eq(&"DEL".to_string()) {
            res.push_str((self.key.len() + 1).to_string().as_str());
            res.push_str("\r\n$3\r\nDEL\r\n$");
            for k in &self.key {
                res.push_str(k.len().to_string().as_str());
                res.push_str("\r\n");
                res.push_str(k.as_str());
                res.push_str("\r\n$");
            }
            res.pop();
        }
        res
    }
}

fn add_str(key: Option<&String>) -> String {
    let mut res = String::new();
    match key {
        Some(k) => {
            res.push_str(k.len().to_string().as_str());
            res.push_str("\r\n");
            res.push_str(k);
            res.push_str("\r\n");
        }
        None => ()
    };
    res
}

#[cfg(test)]
mod test{
    use super::Protocol;
    #[test]
    fn set_analysis(){
        let protocol = Protocol::new("set hello 'hello world'");
        assert_eq!(protocol.to_resp(),"*4\r\n$3\r\nSET\r\n$5\r\nhello\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        let protocol=Protocol::new("get hello");
        assert_eq!(protocol.to_resp(),"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n");
        let protocol=Protocol::new("del hello world");
        assert_eq!(protocol.to_resp(),"*3\r\n$3\r\nDEL\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
    }
}
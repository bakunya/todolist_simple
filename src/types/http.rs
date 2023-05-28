use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HttpObject {
    pub method: Option<String>,
    pub query_string: Option<String>,
    pub http_ver: Option<String>,
    pub host: Option<String>,
    pub http_headers: Option<Vec<Vec<String>>>,
    pub body: Option<String>,
    pub file: Option<String>,
    pub file_ext: Option<String>,
    pub file_valid: Option<bool>
}

impl HttpObject {
    fn put_method(&mut self, value: Vec<&str>) {
        let mut splited = value.get(0).unwrap().split(" ");
        self.method = Some(splited.nth(0).unwrap().to_string());
    }

    fn put_query(&mut self, value: Vec<&str>) {
        let mut splited = value.get(0).unwrap().split(" ");
        let query = splited.nth(1);
        if !query.unwrap().to_string().split("?").nth(1).is_none() {
            self.query_string = Some(
                query
                    .unwrap()
                    .to_string()
                    .split("?")
                    .nth(1)
                    .unwrap()
                    .to_string(),
            );
        }
    }

    fn put_http_ver(&mut self, value: Vec<&str>) {
        let mut splited = value.get(0).unwrap().split(" ");
        self.http_ver = Some(splited.nth(2).unwrap().to_string());
    }

    fn put_host(&mut self, value: Vec<&str>) {
        self.host = Some(
            value
                .get(1)
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap()
                .trim()
                .to_string(),
        );
    }

    fn put_body(&mut self, value: Vec<&str>) {
        self.body = Some(value.get(value.len() - 1).unwrap().replace("\0", ""));
    }

    fn put_http_headers(&mut self, value: Vec<&str>) {
        let mut i = 2;

        loop {
            if i >= value.len() - 2 {
                break;
            }

            match self.clone().http_headers {
                Some(mut headers) => {
                    headers.push(value.get(i).unwrap().trim().to_string().split(": ").map(|v| v.to_string()).collect::<Vec<String>>());
                    self.http_headers = Some(headers);
                }
                None => {
                    self.http_headers = Some(vec![value.get(i).unwrap().trim().to_string().split(": ").map(|v| v.to_string()).collect::<Vec<String>>()]);
                }
            }

            i += 1;
        }
    }

    fn put_http_file(&mut self, value: Vec<&str>) {
        let path = value.get(0).unwrap().split(" ").map(|v| v.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string();
        let mut file = path.split("/").last().unwrap().to_string();        

        if file.is_empty() {
            file = "index.html".to_lowercase();
        }

        let file_ext = file.split(".").last().unwrap().to_string();
        let allowed_ext = ["jpg", "png", "svg", "jpeg", "gif", "html", "css", "js", "ico"];
        
        let mut is_valid = true;

        if file.contains("%") {
            is_valid = false;
        } else {
            for itm in allowed_ext {
                if file_ext.to_lowercase() == itm {
                    is_valid = true;
                    break;
                }
    
                is_valid = false;
            }
        }

        if file_ext.to_lowercase() != "html" {
            file = format!("assets/{}", file);
        }

        self.file = Some(file);
        self.file_ext = Some(file_ext);
        self.file_valid = Some(is_valid);
    }
}

impl HttpObject {
    pub fn init() -> HttpObject {
        HttpObject {
            method: None,
            query_string: None,
            http_ver: None,
            http_headers: None,
            host: None,
            body: None,
            file: None,
            file_ext: None,
            file_valid: None,
        }
    }

    pub fn get_http_header(&self, key: &str) -> Option<String> {
        let mut found = None;
        let mut i = 0;

        loop {
            if i >= self.http_headers.as_ref().expect("Invalid HTTP Headers").len() {
                break;
            }
            
            if self.http_headers.as_ref().expect("Invalid HTTP Headers").get(i).unwrap().get(0).unwrap().to_lowercase() == key.to_lowercase() {
                found = Some(self.http_headers.clone().expect("Invalid HTTP Headers").get(i).unwrap().get(1).unwrap().trim().to_string());
                break;
            }

            i += 1;
        }

        found
    }

    pub fn get_query_string(&self, key: &str) -> Option<String> {
        let mut res = None;
        let cloned = self.query_string.clone().unwrap_or_default();

        for iter in cloned.split("&").collect::<Vec<_>>() {
            if iter.contains(&key) {
                res = Some(iter.trim().replace(key, "").trim().to_string());
                break;
            }
        }

        res
    }

    pub fn to_object(&mut self, value: Vec<&str>) {
        self.put_method(value.clone());
        self.put_query(value.clone());
        self.put_http_ver(value.clone());
        self.put_host(value.clone());
        self.put_body(value.clone());
        self.put_http_headers(value.clone());
        self.put_http_file(value.clone());
    }
}

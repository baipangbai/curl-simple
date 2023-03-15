use curl::easy::{Easy, List};
use curl::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::{stdout, Read, Write};

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct Infos {
        nodeId: String,
        typeIds: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct BatchParam {
        infos: Vec<Infos>,
    }

    #[test]
    fn it_works() {
        let mut infos = Vec::new();
        infos.push(Infos {
            nodeId: "056768560".to_string(),
            typeIds: "56, 57, 66, 70".to_string(),
        });
        let param = BatchParam { infos };

        let mut http = CurlSimpleHttp::new();
        http.bind("https://www.baidu.com".to_string())
            .add_header("header".to_string(), "baipang".to_string())
            .add_header("Content-Type".to_string(), "application/json".to_string())
            .with_header()
            .json_body(param)
            .post()
            .unwrap();
    }
}

pub struct CurlSimpleHttp {
    easy: Easy,
    body: String,
    header: HashMap<String, String>,
}

impl CurlSimpleHttp {
    pub fn new() -> CurlSimpleHttp {
        CurlSimpleHttp {
            easy: Easy::new(),
            body: "".to_string(),
            header: HashMap::new(),
        }
    }

    pub fn bind(&mut self, url: String) -> &mut Self {
        self.easy.url(&url).unwrap();
        self
    }

    pub fn add_header(&mut self, k: String, v: String) -> &mut Self {
        self.header.insert(k, v);
        self
    }

    pub fn with_header(&mut self) -> &mut Self {
        let mut list = List::new();
        for (k, v) in &self.header {
            let tmp = format!("{}:{}", k, v);
            list.append(&tmp).unwrap();
        }
        self.easy.http_headers(list).unwrap();
        self
    }

    pub fn json_body<'a, T: Serialize + Deserialize<'a>>(&mut self, t: T) -> &mut CurlSimpleHttp {
        match serde_json::to_string(&t) {
            Ok(res) => {
                self.body = res;
                self
            }
            Err(err) => panic!("json_body err {:?}", err),
        }
    }

    pub fn post(&mut self) -> Result<(), Error> {
        self.easy.post(true).unwrap();
        let mut param = self.body.as_bytes();
        self.easy.post_field_size(param.len() as u64).unwrap();

        // let mut data = Vec::new();
        {
            let mut transfer = self.easy.transfer();
            transfer
                .read_function(|buf| Ok(param.read(buf).unwrap_or(0)))
                .unwrap();

            transfer
                .write_function(|data| {
                    stdout().write_all(data).unwrap();
                    // dst.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();

            // transfer
            //     .write_function(|new_data| {
            //         data.write_all(new_data).unwrap();
            //         Ok(new_data.len())
            //     })
            //     .unwrap();

            transfer.perform().unwrap();
        }

        Ok(())
    }
}

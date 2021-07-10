use cursive::Cursive;
use cursive::views::Button;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::SelectView;
use cursive::views::TextView;
use spider::ascii_to_char;
use serde_json::Value;
use std::{fs::File,env,path::Path,io::prelude::*};
use serde_json::Result;
extern crate base64;

enum Tcp {
    Ss,
    V2,
}
fn ascii_to_string(code:Vec<u8>) -> String{
    let mut test:String = String::new();
    for cor in code.into_iter(){
        test.push(ascii_to_char(cor));
    }
    test
}
#[derive(Clone)]
pub struct MyButton {
    //pub urls : String,
    //pub name : String,
    //pub port :String,
    pub func :String,
    //pub company:String,
    pub urls: String,
    pub add: String,
    pub aid: String,
    pub host: String,
    pub id: String,
    pub net: String,
    pub path: String,
    pub port: String,
    pub ps: String,
    pub tls: String,
    pub typpe: String

}

impl MyButton{
    pub fn output(&self) -> Dialog {
        fn running_json(s: &mut Cursive,name: &MyButton){
            let mut json = String::new();
            let temp = name.port.clone();
            let length = temp.len();
            let port:String = (&temp[1..length-1]).to_string();
let output = format!(
"{{
    \"inbounds\":[{{
        \"port\":8889,
        \"listen\":\"127.0.0.1\",
        \"protocol\":\"http\",
        \"settings\":{{
            \"udp\": true
        }}
    }}],
    \"outbounds\":[{{
        \"protocol\":{},
        \"settings\":{{
            \"vnext\": [{{
                \"address\": {},
                \"port\":{},
                \"users\":[{{
                    \"id\":{}
                }}]
            }}]
        }}}},
        {{
            \"protocol\":\"freedom\",
            \"tag\": \"direct\",
            \"settings\":{{}}
    }}],
    \"routing\": {{
        \"domainStrategy\": \"IPOnDemand\",
        \"rules\":[{{
            \"type\":\"field\",
            \"ip\":[\"geoip:private\"],
            \"outboundTag\": \"direct\"
        }}]
    }}
}}", name.func,name.add,port,name.id
);
            json.push_str(output.as_str());
            let home = env::var("HOME").unwrap();
            let location = home+"/.config/tv2ray/running.json";
            let path2 = Path::new(location.as_str());
            //let display = path.display();
            //let path2 = Path::new("storage.json");
            let display2 = path2.display();
            let mut file2 = match File::create(&path2) {
                Err(why) => panic!("couldn't create {}: {}",
                                   display2,
                                   why.to_string()),
                Ok(file2) => file2,
            };

            // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
            match file2.write_all(json.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", display2,
                                                       why.to_string())
                },
                Ok(_) => {
                },
            }
            s.pop_layer();
        }
        let mut select = SelectView::<MyButton>::new()
            .on_submit(running_json);
        select.add_item("<start>", self.clone());
        //Dialog::text(format!("Name:{}\nUrl:{}\nport:{}\nfunction:{}\ncompany:{}", self.ps,self.urls,self.port,self.func,self.add))
        //        .title(format!("{}", self.add))
        //        //.button("Quit", Cursive::quit)
        //        .button("quit", |s|{
        //            s.pop_layer();
        //        })
        Dialog::around(
            LinearLayout::horizontal()
                .child(TextView::new(format!("Name:{}\nUrl:{}\nport:{}\nfunction:{}\ncompany:{}", self.ps,self.urls,self.port,self.func,self.add)))
                .child(LinearLayout::vertical()
                    .child(select)
                    .child(Button::new("quit", |s|{
                        s.pop_layer();
                    }))
                    )
            )
    }
    pub fn new (url:String) -> MyButton{
        let mut test : Tcp = Tcp::V2; 
        for pair in url.chars(){
            if pair=='s'{
                test =  Tcp::Ss;
                break;
            }
            if pair=='v'{
                test = Tcp::V2;
                break;

            }
        }
        match test {
            Tcp::Ss => {
                return MyButton{
                    urls  : url,
                    func  : "\"ss\"".to_string(),
                    add   : "\"unknown\"".to_string(),
                    aid   : "\"unknown\"".to_string(),
                    host  : "\"unknown\"".to_string(),
                    id    : "\"unknown\"".to_string(),
                    net   : "\"unknown\"".to_string(),
                    path  : "\"unknown\"".to_string(),
                    port  : "\"unknown\"".to_string(),
                    ps    : "\"unknown\"".to_string(),
                    tls   : "\"unknown\"".to_string(),
                    typpe : "\"unknown\"".to_string()

                }
            },
            Tcp::V2 => {
                let newurl=&url[8..];
                let json = ascii_to_string(base64::decode(newurl.to_string().as_bytes()).unwrap());
                let v : Result<Value> = serde_json::from_str(json.as_str());
                match v {
                    Ok(input)=>{
                        return MyButton{
                            //company : input["add"].to_string(),
                            urls : url,
                            func : "\"vmess\"".to_string(),
                            add : input["add"].to_string(),
                            aid : input["aid"].to_string(),
                            host : input["host"].to_string(),
                            id : input["id"].to_string(),
                            net : input["net"].to_string(),
                            path : input["path"].to_string(),
                            port : input["port"].to_string(),
                            ps : input["ps"].to_string(),
                            tls : input["tls"].to_string(),
                            typpe : input["type"].to_string()
                        }}
                    Err(_)=>{
                        return MyButton{
                            urls  : url,
                            func  : "\"vmess\"".to_string(),
                            add   : "\"unknown\"".to_string(),
                            aid   : "\"unknown\"".to_string(),
                            host  : "\"unknown\"".to_string(),
                            id    : "\"unknown\"".to_string(),
                            net   : "\"unknown\"".to_string(),
                            path  : "\"unknown\"".to_string(),
                            port  : "\"unknown\"".to_string(),
                            ps    : "\"unknown\"".to_string(),
                            tls   : "\"unknown\"".to_string(),
                            typpe : "\"unknown\"".to_string()

                        }

                    }
                }

                
            }
        }
    }
} 



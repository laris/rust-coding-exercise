// Hands-On_Microservices_with_Rust_@Denis_2019_Packt.pdf
// ch04 serde crate
use serde::{Serialize, Deserialize};
//use serde_derive;
use serde_json::Value;
use std::ops::Range;


fn main() {
//----
    #[derive(Serialize, Deserialize, Debug)]
    struct RngResponse {
        value: f64,
    }
    // init struct
    let resp_struct = RngResponse { value: 3.1415926_f64, };
    // convert to json str
    let resp_json_str: String = serde_json::to_string(&resp_struct).unwrap();
    println!("{resp_json_str}");
    // convert back
    let resp_struct_recovered: RngResponse = serde_json::from_str(&resp_json_str).unwrap();
    println!("{resp_struct_recovered:#?}");
//----
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "distribution", content = "parameters", rename_all = "lowercase")]
    enum RngRequest {
        Uniform {
            #[serde(flatten)]
            range: Range<i32>,
        },
        Normal {
            #[serde(rename="MEAN")]
            mean: f64,
            std_dev: f64,
        },
        Bernoulli {
            p: f64,
        }

    }
    let req_enum_uniform = RngRequest::Uniform{ range: 1..10 };
    let req_enum_uniform_json_str: String = serde_json::to_string(&req_enum_uniform).unwrap();
    println!("{req_enum_uniform_json_str}");
    let req_enum_uniform_recovered: RngRequest = serde_json::from_str(&req_enum_uniform_json_str).unwrap();
    println!("{req_enum_uniform_recovered:#?}");

    let req2 = RngRequest::Normal { mean: 3.157389218493_f64, std_dev: 1.4145789302, };
    let req2_json_str: String = serde_json::to_string(&req2).unwrap();
    println!("{req2_json_str}");
//----

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    // {"Request":{"id":0,"method":"get_user","params":[123]}}
    //            {"id":0,"method":"get_user","params":[123]}
    enum RpcRequest<Value> {
        Request { id: u32, method: String, params: Vec<Value> },
        Notification { id: u32, method: String, params: Vec<Value> },
    }
    let rpc_req = RpcRequest::<i32>::Request { id: 0, method: String::from("get_user"), params: vec![123] };
    //let rpc_note  = RpcRequest::Notification  { id: 0, method: String::from("get_user"), params: vec![123] };
    println!("{rpc_req:?}");
    let rpc_req_json_str: String = serde_json::to_string(&rpc_req).unwrap();
    println!("{rpc_req_json_str}");
//----
    #[derive(Deserialize)]
    struct Response {
        id: u32,
        result: serde_json::Value,
    }
    //let u: User = serde_json::from_value(&response)?;
}


extern crate rand;
extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;

// const URL: &'static str= "localhost:8082";


fn get_haiku() -> String {
    let selected_index = rand::thread_rng().gen_range(1,6);

    let selected_haiku = match selected_index {
        1 => "古池 \n蛙飛び込む \n水の音",
        2 => "蛸壺や \nはかなき夢を \n夏の月",
        3 => "年暮れぬ \n笠きて草鞋 \nはきながら",
        4 => "この道や \n行く人なしに \n秋のくれ",
        5 => "旅に病で \n夢は枯野を \nかけ廻る",
        6 => "田一枚 \n植て立去る \n柳かな",
        _ => "No lo sepio"
    };
    selected_haiku.to_string()
} 
fn main() {
    println!("HAIKU:\n{}", get_haiku());
}

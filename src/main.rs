/*
 * @Author: 四氿
 * @Date: 2021-11-06 14:02:38
 * @LastEditTime: 2021-12-12 15:52:39
 * @Description: 
 */
mod proxy;
use std::time::Duration;
use proxy::Proxy;

#[tokio::main]
async fn main() {
    print!("proxy serve");
    let proxy = Proxy::new();
    proxy.serve().await;
 }


 async fn test() {
     print!("test");
     std::thread::sleep(Duration::new(2, 1000));
 }
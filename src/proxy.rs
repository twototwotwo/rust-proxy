/*
 * 代理，负责建立代理
 * @author wsjiu
 * @date 2021/11/02
 */
use tokio::net::TcpStream;
use hyper::upgrade::Upgraded;
use hyper::Response;
use std::convert::Infallible;
use std::net::{SocketAddr};
use hyper::{Body, Client, Request, Method, http};
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};

type HttpClient = Client<hyper::client::HttpConnector>;

pub struct Proxy {
    ip : [u8; 4],
    port: u16
}

impl Proxy {
    pub fn new() -> Proxy {
        Proxy {
            ip : [127, 0, 0, 1],
            port : 8888
        }
    }
    
    pub async fn serve(&self) { 
        let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

        let addr = SocketAddr::from((self.ip, self.port));
        let svc =   make_service_fn(move |_| {
            let client = client.clone();
            async move { Ok::<_, Infallible>(service_fn(move |req| handle(client.clone(), req)))}
        });
        let server = Server::bind(&addr)
            .http1_preserve_header_case(true)
            .http1_title_case_headers(true)
            .serve(svc);

        if let Err(e) = server.await {
            println!("server error by {}", e);
        }
    }
}

async fn handle(client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if Method::CONNECT == req.method() {
        // Method::CONNECT indicates that req expect upgrade protocol
        if let Some(addr) = parse_host(req.uri()) {
            tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, addr).await {
                            println!("server io error by {}", e);
                        };
                    },
                    Err(e) => println!("connect upgrade error by {}", e),
                }
            });
        };
        println!("connect upgraded success");
        Ok(Response::new(Body::empty()))
    }else {
        let resp = client.request(req).await?;
        Ok(Response::new(Body::from("aa")))

    }
}

fn parse_host(uri : &http::Uri) -> Option<String> {
    uri.authority().and_then(|auth| Some(auth.to_string()))
}

async fn tunnel(mut upgraded : Upgraded, addr : String) -> std::io::Result<()> {
    let mut conn = TcpStream::connect(addr).await?;
    let (send, receive) = tokio::io::copy_bidirectional(&mut upgraded, &mut conn).await?;
    println!("client send {}, and revice {}", send, receive);
    Ok(())
}


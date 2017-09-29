extern crate hyper;
extern crate futures;
extern crate mime;

use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};
use hyper::header::{ContentType, Headers};
use std::process::Command;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::net::IpAddr;
use std::str;

struct SpaceText;

impl Service for SpaceText {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        let mut headers = Headers::new();
        headers.set(ContentType(mime::TEXT_HTML));
        let path = match req.path() {
            "/" => "/welcome",
            p => p,
        };

        match (req.method(), path) {
            (&Method::Get, _) => {
                let letters = path.to_uppercase().chars().skip(1).collect::<Vec<char>>();
                let mut cross: std::string::String = letters
                    .clone()
                    .iter()
                    .map(|c| char::to_string(&c))
                    .collect::<Vec<std::string::String>>()
                    .join(" ");
                let mut skip_first = false;
                for c in letters.iter() {
                    if !skip_first {
                        skip_first = true;
                        continue;
                    }

                    cross.push('\n');
                    cross.push(*c);
                }

                let mut case_flip = std::string::String::new();
                let mut idx = 0;
                for c in letters.iter() {
                    if idx % 2 == 0 {
                        case_flip.push(c.to_lowercase().next().expect("error to_lowercase"));
                    } else {
                        case_flip.push(c.to_uppercase().next().expect("error to_uppercase"));
                    }

                    idx += 1
                }

                let parts = vec![cross, case_flip];
                let body = format!("<pre>{}</pre>\n", parts.join("\n\n").as_str());
                response.set_body(body);
            }
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        };

        futures::future::ok(response)
    }
}

fn main() {
    let host_port = 8080;
    let hostname_cmd = Command::new("hostname").arg("-I").output();
    // Grab hostname from hostname command, default to 127.0.0.1
    let host_addr: Ipv4Addr = match hostname_cmd {
        Ok(res) => {
            let addr = str::from_utf8(res.stdout.as_slice())
                .map_err(|err| err.to_string())
                .and_then(|ip_str| {
                    ip_str.trim().parse::<Ipv4Addr>().map_err(
                        |err| err.to_string(),
                    )
                });

            match addr {
                Ok(addr) => addr,
                Err(_) => Ipv4Addr::new(127, 0, 0, 1),
            }
        }
        Err(_) => Ipv4Addr::new(127, 0, 0, 1),
    };
    let host_addr_and_port: SocketAddr = SocketAddr::new(IpAddr::V4(host_addr), host_port);

    println!("Server listening at {}", host_addr_and_port);
    let server = Http::new()
        .bind(&host_addr_and_port, || Ok(SpaceText))
        .unwrap();
    server.run().unwrap();
}

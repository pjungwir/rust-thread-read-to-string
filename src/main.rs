extern crate hyper;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use hyper::{Client};
use hyper::client::{Request, Response};

#[derive(Debug)]
enum MyAppError {
  Timeout,
  TcpError(hyper::error::Error),
  ReadError(std::io::Error),
}

fn send_request(url: &str) -> Result<Response, MyAppError> {
  let mut c = Client::new();
  let mut req = c.get(url);
  req.send().map_err(|e| MyAppError::TcpError(e))
}

fn send_request_with_timeout(url: &str) -> Result<Response, MyAppError> {
  let (tx, rx) = mpsc::channel();
  let url = url.to_owned();
  let t = thread::spawn(move || {
    match tx.send(send_request(&url)) {
      Ok(()) => {}, // everything good
      Err(_) => {}, // we have been released, no biggie
    }
  });
  match rx.recv_timeout(Duration::from_millis(5000)) {
    Ok(resp) => resp,
    Err(_) => Err(MyAppError::Timeout),
  }
}

fn get_url(url: &str, mut buf: &mut String) -> Result<u16, MyAppError> {
  let mut resp = send_request(url)?;
  resp.read_to_string(&mut buf).map_err(|e| MyAppError::ReadError(e))?;
  Ok(resp.status.to_u16())
}

fn get_url_with_timeout_1(url: &str, mut buf: &mut String) -> Result<u16, MyAppError> {
  let mut resp = send_request_with_timeout(url)?;
  resp.read_to_string(&mut buf).map_err(|e| MyAppError::ReadError(e))?;
  Ok(resp.status.to_u16())
}

fn get_url_with_timeout_2(url: &str, mut buf: &mut String) -> Result<u16, MyAppError> {
  let (tx, rx) = mpsc::channel();
  let url = url.to_owned();
  let t = thread::spawn(move || {
    match tx.send(get_url(&url, &mut buf)) {
      Ok(()) => {}, // everything good
      Err(_) => {}, // we have been released, no biggie
    }
  });
  match rx.recv_timeout(Duration::from_millis(5000)) {
    Ok(resp) => resp,
    Err(_) => Err(MyAppError::Timeout),
  }
}

fn main() {
  let mut buf = String::new();
  get_url("http://example.com/", &mut buf).unwrap();
  println!("get_url: {}", &buf[0..20]);

  let mut buf = String::new();
  get_url_with_timeout_1("http://example.com/", &mut buf).unwrap();
  println!("get_url_with_timeout_1: {}", &buf[0..20]);

  let mut buf = String::new();
  get_url_with_timeout_2("http://example.com/", &mut buf).unwrap();
  println!("get_url_with_timeout_2: {}", &buf[0..20]);
}


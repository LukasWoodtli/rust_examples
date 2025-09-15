use std::thread;
use std::time::Duration;
use crate::future::{Future, PollState};
use crate::http::Http;

mod http;
mod future;

enum State {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved
}
struct Coroutine {
    state: State,
}

impl Coroutine {
    fn new() -> Self {
        Self {state: State::Start}
    }
}

impl Future for Coroutine {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("Starting");
                    let fu = Box::new(Http::get("/600/HelloWorld1"));
                    self.state = State::Wait1(fu);
                }
                State::Wait1(ref mut fut) => match fut.poll() {
                    PollState::Ready(txt) => {
                        println!("Got response 1: {}", txt);
                        let fut2 = Box::new(Http::get("/400/HelloWorld2"));
                        self.state = State::Wait2(fut2);
                    }
                    PollState::NotReady => {
                        break PollState::NotReady;
                    }
                }
                State::Wait2(ref mut fut2) => match fut2.poll() {
                    PollState::Ready(txt2) => {
                        println!("Got response 2: {}", txt2);
                        self.state = State::Resolved;
                        break PollState::Ready(());
                    }
                    PollState::NotReady => {
                        break PollState::NotReady;
                    }
                }
                State::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}

fn async_main() -> impl Future<Output = ()> {
    Coroutine::new()
}

fn main() {
    let mut future = async_main();
    loop {
        match future.poll() {
            PollState::NotReady => println!("Schedule other tasks"),
            PollState::Ready(()) => break,
        }

        thread::sleep(Duration::from_millis(100));
    }
}

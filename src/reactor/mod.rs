use futures::{
    IntoFuture,
    Future,
    Async
};
use opencm::common::asm;

static mut ACTION: bool = true;

pub fn set_action_flag() {
    unsafe {ACTION = true};
}

pub fn run<I, E, F: IntoFuture<Item=I, Error=E>>(f: F) -> Result<I, E> {
    let mut future = f.into_future();

    loop {
        while unsafe {ACTION} {
            unsafe {ACTION = false};

            match future.poll() {
                Ok(Async::Ready(val)) => return Ok(val),
                Err(err) => return Err(err),
                _ => {}
            };
        }

        asm::wfi();
    }
}
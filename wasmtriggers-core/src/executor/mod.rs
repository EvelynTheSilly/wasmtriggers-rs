use std::{
    collections::VecDeque,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

pub struct Executor {
    tasks: Mutex<VecDeque<Pin<Box<dyn Future<Output = ()>>>>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: VecDeque::new().into(),
        }
    }
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.lock().unwrap().push_back(Box::pin(future));
    }
    pub fn poll_all(&self) {
        let mut tasks = self.tasks.lock().unwrap();
        let waker = make_waker();
        let mut i = 0;
        while i < tasks.len() {
            let mut cx = Context::from_waker(&waker);

            match tasks[i].as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    tasks.swap_remove_back(i);
                }
                Poll::Pending => {
                    i += 1;
                }
            }
        }
    }
}

fn make_waker() -> Waker {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) }
}

use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub fn wait_next_gametick() -> impl Future<Output = ()> {
    struct WaitNextGametick {
        has_waited: bool,
    }

    impl Future for WaitNextGametick {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
            if self.has_waited {
                Poll::Ready(())
            } else {
                self.has_waited = true;
                Poll::Pending
            }
        }
    }

    WaitNextGametick { has_waited: false }
}

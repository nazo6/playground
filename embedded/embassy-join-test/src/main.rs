use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// Atomic flags to track future readiness
static FUTURE1_READY: AtomicBool = AtomicBool::new(false);
static FUTURE2_READY: AtomicBool = AtomicBool::new(false);

// A simple future that completes after two polls
struct SlowFuture {
    count: u8,
}

impl Future for SlowFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            self.count += 1;
            FUTURE1_READY.store(true, Ordering::SeqCst); // Indicate it needs polling again
            cx.waker().wake_by_ref();
            log::info!("SlowFuture polled once");
            Poll::Pending
        } else {
            Poll::Ready("SlowFuture done")
        }
    }
}

// A simple future that completes immediately
struct FastFuture;

impl Future for FastFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("FastFuture done")
    }
}

// Custom waker function
fn waker_fn(ready_flag: &'static AtomicBool) -> Waker {
    fn noop_clone(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }

    fn wake(ptr: *const ()) {
        let flag = unsafe { &*(ptr as *const AtomicBool) };
        flag.store(true, Ordering::SeqCst);
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, wake, wake, wake);

    let raw_waker = RawWaker::new(ready_flag as *const _ as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

// Manually poll only the ready future
fn poll_futures<F1, F2>(future1: &mut F1, future2: &mut F2)
where
    F1: Future<Output = &'static str> + Unpin,
    F2: Future<Output = &'static str> + Unpin,
{
    let waker1 = waker_fn(&FUTURE1_READY);
    let waker2 = waker_fn(&FUTURE2_READY);
    let mut cx1 = Context::from_waker(&waker1);
    let mut cx2 = Context::from_waker(&waker2);

    loop {
        let mut done1 = false;
        let mut done2 = false;

        if FUTURE1_READY.load(Ordering::SeqCst) {
            if let Poll::Ready(output) = Pin::new(&mut *future1).poll(&mut cx1) {
                done1 = true;
                FUTURE1_READY.store(false, Ordering::SeqCst);
            }
        }

        if FUTURE2_READY.load(Ordering::SeqCst) {
            if let Poll::Ready(output) = Pin::new(&mut *future2).poll(&mut cx2) {
                done2 = true;
                FUTURE2_READY.store(false, Ordering::SeqCst);
            }
        }

        if done1 && done2 {
            break;
        }
    }
}

fn main() {
    let mut future1 = SlowFuture { count: 0 };
    let mut future2 = FastFuture;

    // Initially, only poll FastFuture
    FUTURE1_READY.store(true, Ordering::SeqCst);
    FUTURE2_READY.store(true, Ordering::SeqCst);

    poll_futures(&mut future1, &mut future2);

    loop {}
}

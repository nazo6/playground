use core::cell::Cell;
use core::future::Future;
use core::marker::Unpin;
use core::mem::MaybeUninit;
use core::pin::Pin;
use core::ptr;
use core::task::{Context, Poll};

pub struct ManualJoin2<F1, F2> {
    fut1: MaybeUninit<F1>,
    fut2: MaybeUninit<F2>,
    done: Cell<u8>, // 各 Future の完了フラグ
}

impl<F1, F2> ManualJoin2<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    pub fn new(fut1: F1, fut2: F2) -> Self {
        Self {
            fut1: MaybeUninit::new(fut1),
            fut2: MaybeUninit::new(fut2),
            done: Cell::new(0),
        }
    }
}

impl<F1, F2> Future for ManualJoin2<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut done = this.done.get();
        let mut fut1_out = None;
        let mut fut2_out = None;

        // Future1 のポーリング
        if done & 1 == 0 {
            let fut1 = unsafe { Pin::new_unchecked(&mut *this.fut1.as_mut_ptr()) };
            if let Poll::Ready(out) = fut1.poll(cx) {
                fut1_out = Some(out);
                done |= 1; // 完了フラグをセット
            }
        }

        // Future2 のポーリング
        if done & 2 == 0 {
            let fut2 = unsafe { Pin::new_unchecked(&mut *this.fut2.as_mut_ptr()) };
            if let Poll::Ready(out) = fut2.poll(cx) {
                fut2_out = Some(out);
                done |= 2; // 完了フラグをセット
            }
        }

        this.done.set(done);

        // 両方完了したら結果を返す
        if done == 3 {
            Poll::Ready((fut1_out.unwrap(), fut2_out.unwrap()))
        } else {
            Poll::Pending
        }
    }
}

pub fn join<F1, F2>(fut1: F1, fut2: F2) -> ManualJoin2<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    ManualJoin2::new(fut1, fut2)
}

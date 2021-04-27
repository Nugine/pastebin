use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nuclear::http::StatusCode;
use nuclear::{BoxFuture, Middleware, Next, Request, Response, Result};
use nuclear::{BoxedHandler, Handler, HandlerExt};

use tokio::sync::oneshot;
use tokio::time::interval;

pub struct TokenBucket {
    current_tokens: Arc<AtomicU64>,
    kill_tx: Option<oneshot::Sender<()>>,

    period: Duration,
    amount: u64,
    capacity: u64,

    on_limit: BoxedHandler<'static>,
}

pub async fn create503(_: Request) -> Response {
    let status = StatusCode::SERVICE_UNAVAILABLE;
    let body = "503 Service temporarily unavailable";
    Response::new(status, body.into())
}

impl TokenBucket {
    pub fn new(period: Duration, amount: u64, capacity: u64) -> Self {
        Self {
            current_tokens: Arc::new(AtomicU64::from(0)),
            kill_tx: None,
            period,
            amount,
            capacity,
            on_limit: create503.boxed(),
        }
    }

    async fn daemon(
        period: Duration,
        amount: u64,
        capacity: u64,
        ct: Arc<AtomicU64>,
        mut kill_rx: oneshot::Receiver<()>,
    ) {
        let mut i = interval(period);
        let ct = &*ct;
        let update = || {
            let mut prev = ct.load(Ordering::SeqCst);
            loop {
                let next = prev.saturating_add(amount).min(capacity);
                match ct.compare_exchange_weak(prev, next, Ordering::SeqCst, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(p) => prev = p,
                }
            }
        };

        loop {
            tokio::select! {
                _ = &mut kill_rx => {
                    break;
                }
                _ = i.tick() => {
                    update();
                }
            };
        }
    }

    fn consume(&self) -> Option<()> {
        let ct = &*self.current_tokens;
        let mut prev = ct.load(Ordering::SeqCst);
        loop {
            if prev == 0 {
                return None;
            }
            match ct.compare_exchange_weak(prev, prev - 1, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => break Some(()),
                Err(p) => prev = p,
            }
        }
    }

    pub fn spawn_daemon(&mut self) {
        if self.kill_tx.is_some() {
            tracing::warn!("this limiter has already a running daemon");
            return;
        }

        let (tx, rx) = oneshot::channel();
        self.kill_tx = Some(tx);
        tokio::spawn(Self::daemon(
            self.period,
            self.amount,
            self.capacity,
            Arc::clone(&self.current_tokens),
            rx,
        ));
    }
}

impl Drop for TokenBucket {
    fn drop(&mut self) {
        if let Some(tx) = self.kill_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl<'a> Middleware<'a> for TokenBucket {
    type Future = BoxFuture<'a, Result<Response>>;

    fn handle<'t, 'h>(&'t self, req: Request, next: Next<'h>) -> Self::Future
    where
        't: 'a,
        'h: 'a,
        Self: 'a,
    {
        match self.consume() {
            Some(()) => next.handle(req),
            None => self.on_limit.handle(req),
        }
    }
}

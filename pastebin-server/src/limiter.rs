use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nuclear::http::StatusCode;
use nuclear::{BoxFuture, Middleware, Next, Request, Response, Result};
use nuclear::{BoxedHandler, Handler, HandlerExt};

use tokio::task::JoinHandle;
use tokio::time::interval;

pub struct TokenBucket {
    current_tokens: Arc<AtomicU64>,
    daemon: Option<JoinHandle<()>>,

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
            daemon: None,
            period,
            amount,
            capacity,
            on_limit: create503.boxed(),
        }
    }

    async fn daemon(period: Duration, amount: u64, capacity: u64, ct: Arc<AtomicU64>) {
        let ct = &*ct;

        let mut int = interval(period);
        int.tick().await;

        loop {
            int.tick().await;

            let mut prev = ct.load(Ordering::SeqCst);
            loop {
                let next = prev.saturating_add(amount).min(capacity);
                match ct.compare_exchange_weak(prev, next, Ordering::SeqCst, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(p) => prev = p,
                }
            }
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
        if self.daemon.is_some() {
            tracing::warn!("this limiter has already a running daemon");
            return;
        }

        self.daemon = Some(tokio::spawn(Self::daemon(
            self.period,
            self.amount,
            self.capacity,
            Arc::clone(&self.current_tokens),
        )));
    }
}

impl Drop for TokenBucket {
    fn drop(&mut self) {
        if let Some(daemon) = self.daemon.take() {
            daemon.abort();
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

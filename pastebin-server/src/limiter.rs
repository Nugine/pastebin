use nuclear::core::{Endpoint, Middleware, Next, Request, Response, Result};
use nuclear::{core::async_trait, http::StatusCode};
use std::sync::atomic::{self, AtomicU64};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::interval;

pub struct TokenBucket {
    current_tokens: Arc<AtomicU64>,
    kill_tx: Option<oneshot::Sender<()>>,

    period: Duration,
    amount: u64,
    capacity: u64,

    on_limit: Box<dyn Endpoint>,
}

pub async fn create503(_: Request) -> Response {
    let mut res = Response::new("503 Service temporarily unavailable");
    res.set_status(StatusCode::SERVICE_UNAVAILABLE);
    res
}

impl TokenBucket {
    pub fn new(period: Duration, amount: u64, capacity: u64) -> Self {
        Self {
            current_tokens: Arc::new(AtomicU64::from(0)),
            kill_tx: None,
            period,
            amount,
            capacity,
            on_limit: Box::new(create503),
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
        let update = || loop {
            let prev = ct.load(atomic::Ordering::SeqCst);
            let next = prev.saturating_add(amount).min(capacity);
            let res = ct.compare_and_swap(prev, next, atomic::Ordering::SeqCst);
            if res == prev {
                break;
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

    pub fn consume(&self) -> Option<()> {
        let ct = &*self.current_tokens;
        let mut prev = ct.load(atomic::Ordering::SeqCst);
        loop {
            if prev == 0 {
                return None;
            }
            let res = ct.compare_and_swap(prev, prev - 1, atomic::Ordering::SeqCst);
            if res == prev {
                break Some(());
            }
            prev = ct.load(atomic::Ordering::SeqCst);
        }
    }

    pub fn spawn_daemon(&mut self) {
        if self.kill_tx.is_some() {
            log::warn!("spawn_daemon has already been called on this limiter")
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

#[async_trait]
impl Middleware for TokenBucket {
    async fn call(&'_ self, req: Request, next: Next<'_>) -> Result<Response> {
        match self.consume() {
            Some(()) => next.call(req).await,
            None => self.on_limit.call(req).await,
        }
    }
}

#![allow(dead_code, unused_imports)]
use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use std::sync::{Condvar, Mutex};
use std::{sync::atomic::AtomicBool, thread, time::Duration};

static ATOMIC_LOCKER: AtomicBool = AtomicBool::new(false);
static MUTEX_LOCKER: Mutex<bool> = Mutex::new(false);
static CONDVAR: Condvar = Condvar::new();

fn main() {
    #[cfg(feature = "atomic")]
    {
        thread::spawn(|| ping_atomic());
        thread::spawn(|| pong_atomic());
    }

    #[cfg(feature = "mutex")]
    {
        thread::spawn(|| ping_mutex());
        thread::spawn(|| pong_mutex());
    }

    #[cfg(feature = "no_sync")]
    {
        thread::spawn(|| ping());
        thread::spawn(|| pong());
    }

    thread::sleep(Duration::from_secs(10));
}

fn ping_atomic() {
    loop {
        let b = ATOMIC_LOCKER.load(SeqCst);
        if !b {
            println!("ping");
            ATOMIC_LOCKER.store(!b, SeqCst);
        }
    }
}

fn pong_atomic() {
    loop {
        let b = ATOMIC_LOCKER.load(SeqCst);
        if b {
            println!("pong");
            ATOMIC_LOCKER.store(!b, SeqCst);
        }
    }
}

fn ping_mutex() {
    #[cfg(not(feature = "condvar_enabled"))]
    loop {
        let mut b = MUTEX_LOCKER.lock().expect("PING UNLOCK ERR");
        if !*b {
            println!("ping");
            *b = !*b;
            CONDVAR.notify_one();
        }
    }
    #[cfg(feature = "condvar_enabled")]
    loop {
        let mut b = MUTEX_LOCKER.lock().expect("PING UNLOCK ERR");
        while *b {
            b = CONDVAR.wait(b).expect("PING WAIT ERR");
        }
        println!("ping");
        *b = true;
        CONDVAR.notify_one();
    }
}

fn pong_mutex() {
    #[cfg(not(feature = "condvar_enabled"))]
    loop {
        let mut b = MUTEX_LOCKER.lock().expect("PONG UNLOCK ERR");
        if *b {
            println!("pong");
            *b = !*b;
            CONDVAR.notify_one();
        }
    }
    #[cfg(feature = "condvar_enabled")]
    loop {
        let mut b = MUTEX_LOCKER.lock().expect("PONG UNLOCK ERR");
        while !*b {
            b = CONDVAR.wait(b).expect("PONG WAIT ERR");
        }
        println!("pong");
        *b = false;
        CONDVAR.notify_one();
    }
}

fn ping_condvar() {
    loop {
        let mut b = MUTEX_LOCKER.lock().expect("PING UNLOCK ERR");
        if !*b {
            println!("ping");
            *b = !*b;
        }
    }
}

fn pong_condvar() {
    loop {
        let mut b = MUTEX_LOCKER.lock().expect("PONG UNLOCK ERR");
        if *b {
            println!("pong");
            *b = !*b;
        }
    }
}

fn ping() {
    loop {
        println!("ping");
    }
}

fn pong() {
    loop {
        println!("pong");
    }
}

//# parking_lot = "*"
//# backoff = "*"
use parking_lot::{Mutex, ReentrantMutex};
use std::rc::Rc;
use std::sync::{Arc, Mutex as StdMutex};
use std::thread;
use std::time::Duration;
//demo1
fn rescure (
    data: Rc<Mutex<u32>>,
    remaining: usize,
    ) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => { println!("rem = 0, return 0"); 0 },
        rem => { println!("rem: {rem} and rem != 0, call rescure(data: {data:?}, remaining: {remaining})"); 
            rescure(Rc::clone(&data), rem -1)
        },
    }
}
//demo2
fn rescure2(
    data: Rc<ReentrantMutex<u32>>,
    remaining: usize,
    ) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => { println!("rem = 0, return 0"); 0 },
        rem => { 
            println!("rem: {rem} and rem != 0, call rescure(data: {data:?}, remaining: {remaining})"); 
            rescure2(Rc::clone(&data), rem -1)
        },
    }
}
fn main() {
    //demo1 no deadlock
    let data = Rc::new(Mutex::new(0_u32));
    let rem  = 0;
    let result = rescure(data, rem);
    println!("result: {result}");
    //demo2 deadlock
    let data = Rc::new(ReentrantMutex::new(0_u32));
    let rem  = 1;
    let result = rescure2(data, rem);
    println!("result: {result}");
    //demo3
    type ArcAccount = Arc<StdMutex<Account>>;
    #[derive(Debug)]
    struct Account {
        balance: i64,
    }
    fn transfer1(from: ArcAccount, to: ArcAccount, amount: i64) {
        let mut from = from.lock();
        let mut to = to.lock();
        from.unwrap().balance -= amount;
        println!("transfer1 from - amount({amount}) done");
        to.unwrap().balance += amount;
        println!("transfer1 to + amount({amount}) done");
    }
    let account = Account { balance: 1000, };
    let account1 = Arc::new(StdMutex::new(Account { balance: 1000, }));
    let account2 = Arc::new(StdMutex::new(Account { balance: 2000, }));
    let a1 = account1.clone();
    let a2 = account1.clone();
    let b1 = account2.clone();
    let b2 = account2.clone();

    println!("before transfer1 account1: {account1:?}");
    println!("before transfer1 account2: {account2:?}");
    let t1 = thread::spawn(move || {transfer1(a1, b1, 500);});
    let t2 = thread::spawn(move || {transfer1(b2, a2, 800);});
    println!("after transfer1 account1: {account1:?}");
    println!("after transfer1 account2: {account2:?}");

    thread::sleep(Duration::from_secs(1));

    fn transfer2(from: ArcAccount, to: ArcAccount, amount: i64) {
        loop {
            if let Ok(mut from) = from.try_lock() {
                if let Ok(mut to) = to.try_lock() {
                    from.balance -= amount;
                    println!("transfer2 from - amount({amount}) done");
                    to.balance += amount;
                    println!("transfer2 to + amount({amount}) done");
                    return;
                }
            }
            thread::sleep(Duration::from_millis(2));
        }
    }
    let a1 = account1.clone();
    let a2 = account1.clone();
    let b1 = account2.clone();
    let b2 = account2.clone();
    println!("before transfer2 account1: {account1:?}");
    println!("before transfer2 account2: {account2:?}");
    let t1 = thread::spawn(move || {transfer2(a1, b1, 500);});
    let t2 = thread::spawn(move || {transfer2(b2, a2, 800);});
    println!("after transfer2 account1: {account1:?}");
    println!("after transfer2 account2: {account2:?}");

    // demo4
    use backoff::ExponentialBackoff;
    fn transfer3(from: ArcAccount, to: ArcAccount, amount: i64) {
        let op = || {
            if let Ok(mut from) = from.try_lock() {
                if let Ok(mut to) = to.try_lock() {
                    from.balance -= amount;
                    println!("transfer3 from - amount({amount}) done");
                    to.balance += amount;
                    println!("transfer3 to + amount({amount}) done");
                    return Ok(());
                }
            }
            Err(0)?
        };
        let backoff = ExponentialBackoff::default();
        backoff::retry(backoff, op);
    }
    thread::sleep(Duration::from_secs(1));
    let a1 = account1.clone();
    let a2 = account1.clone();
    let b1 = account2.clone();
    let b2 = account2.clone();
    println!("before transfer3 account1: {account1:?}");
    println!("before transfer3 account2: {account2:?}");
    let t1 = thread::spawn(move || {transfer3(a1, b1, 500);});
    let t2 = thread::spawn(move || {transfer3(b2, a2, 800);});
    println!("after transfer3 account1: {account1:?}");
    println!("after transfer3 account2: {account2:?}");
}

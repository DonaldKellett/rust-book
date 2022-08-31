use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let m1 = Arc::new(Mutex::new(42));
    let m2 = Arc::new(Mutex::new(37));

    let m1_2 = Arc::clone(&m1);
    let m2_2 = Arc::clone(&m2);

    let handle1 = thread::spawn(move || {
        println!("Acquiring mutex m1 in child thread #1 ...");
        let _num1 = m1.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        println!("Acquiring mutex m2 in child thread #1 ...");
        let _num2 = m2.lock().unwrap();

        // This will never print - deadlock!
        println!("Acquired both mutexes from child thread #1");
    });

    let handle2 = thread::spawn(move || {
        println!("Acquiring mutex m2 in child thread #2 ...");
        let _num2 = m2_2.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        println!("Acquiring mutex m1 in child thread #2 ...");
        let _num1 = m1_2.lock().unwrap();

        // This will never print - deadlock!
        println!("Acquired both mutexes from child thread #2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // This will never print - deadlock!
    println!("Joined both child threads; exiting ...");
}

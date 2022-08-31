## deadlock

A Rust program that deliberately deadlocks by creating a cycle in the resource allocation graph (RAG)

### Usage

Run the project like you would any other:

```bash
$ cargo run
```

You should initially see the following output, possibly in a different order:

```
Acquiring mutex m1 in child thread #1 ...
Acquiring mutex m2 in child thread #2 ...
```

After approx. 1 second, you should see two more lines printed to the console, possibly in a different order:

```
Acquiring mutex m2 in child thread #1 ...
Acquiring mutex m1 in child thread #2 ...
```

Then ... the program just hangs, because we've created a deadlock and none of the child threads can make any further progress. Since the main thread waits for its children by calling `join`, the main thread hangs as well and cannot exit normally. Press `Ctrl-C` to kill the program.

### Explanation

If you inspect the code, you'll see that there are two mutexes: `m1` and `m2`. We use `Arc` (**a**tomic **r**eference **c**ounting) to clone the references to `m1` and `m2` so they can be owned by two separate threads while satisfying Rust's borrow checker. Then, we create two threads and pass ownership of both mutexes to both threads:

- Thread 1 acquires the lock to `m1`, waits 1 second, then attempts to acquire the lock to `m2`
- At the same time, thread 2 acquires the lock to `m2`, waits 1 second, then attempts to acquire the lock to `m1`

Since threads 1 and 2 acquire locks to `m1` and `m2` respectively, they wait on each other for `m2` and `m1` respectively after approximately 1 second. But since neither of them are willing to give up their existing acquired lock, they wait for each other forever.

One way to ensure deadlocks never occur is to enforce a total order on which mutexes will be acquired. For example, we could define `m1 < m2` such that no thread will ever attempt to acquire `m1` while holding a lock to `m2`. This way, it's impossible to create a cycle in our RAG and thus no deadlock could occur.

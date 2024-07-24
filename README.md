### Experiments with multithreaded and concurrent programming

### Comparison of locks (src/mutex.rs and src/bin/mutex):

| Lock           | Execution time |
| -------------- | -------------  |
| SpinlockTicket |     701 ms     |
| Mutex          |     250 ms     |
| Spinlock       |     108 ms     |

### Threadpool efficiency (src/threadpool.rs and src/bin/threadpool.rs):

| Count threads  | Execution time |
| -------------- | -------------  |
| 1              |     17712 ms   |
| 6              |     3590 ms    |

### Examples lazy static (src/bin.lazy.rs)

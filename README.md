### Примеры конкурентного программирования на Rust

- Блокировки (locks) на атомарных переменных:

src/mutex.rs - реализация spinlock и spinlockticket (спинлок c билетами) на атомарных переменных.

src/bin/mutex - тестирование быстродействия данных блокировок с мютексом. Алгоритм заключается в подсчете счетчика при попеременном переключении между 2 потоками,
в тест также добавлена реализация mutex на атомарных переменных (mutex on atomic)

| Lock            | Execution time |
| --------------- | -------------  |
| SpinlockTicket  |     701 ms     |
| Mutex           |     250 ms     |
| Mutex on atomic |     250 ms     |
| Spinlock        |     108 ms     |

- Пул потоков (Threadpool):

src/threadpool.rs - реализация пула потоков

src/bin/threadpool.rs - тестирования производительности пула потоков.

| Count threads  | Execution time |
| -------------- | -------------  |
| 1              |     17712 ms   |
| 6              |     3590 ms    |

- Примеры отложенной инициализации (lazy static):

src/bin.lazy.rs - непосредственно примеры, с использование Once, OnceLock и с применением атомарных переменных. Также тест производительности

| Used primitive         | Execution time |
| ---------------------- | -------------  |
| Once                   |     76 ms      |
| AtomicU64              |     76 ms      |
| OnceLock (reference)   |     843 ms     |
| AtomicPtr (reference)  |     891 ms     |

- Потокобезопасный генератор ID на атомарной переменной (Generator ID):

src/bin/generatorid.rs - непосредственно пример генератора

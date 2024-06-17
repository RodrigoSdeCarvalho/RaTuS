#[cfg(test)]
mod tests {
    use std::thread::JoinHandle;
    use std::{thread, time};
    
    use ts_core::mutex_store::MutexStore;
    use ts_core::query_tuple::QueryTuple;
    use ts_core::result::Result;
    use ts_core::store::Store;
    use ts_core::tuple::Tuple;
    use ts_core::vec_store::VecStore;

    use system::{Logger, set_process_name};

    #[test]
    fn test_threads() {
        set_process_name("TS Threads");

        Logger::info("Starting", true);
        let mutex_store = MutexStore::<VecStore>::default();
    
        let mut writer_tuple_space = mutex_store.clone();
        let writer_1_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            Logger::info("Writer 1", true);
            let writer_sleep = time::Duration::from_millis(100);
    
            for i in 0..100 {
                let tuple = Tuple::builder().integer(1).integer(i).build();
                Logger::info(&format!("Writer 1: Wrote: {:?}", tuple), true);
                writer_tuple_space.write(&tuple)?;
                thread::sleep(writer_sleep);
            }
            Logger::info("Writer 1: Wrote 100 tuples", true);

            Ok(())
        });
    
        thread::sleep(time::Duration::from_millis(200));
    
        let reader_mutex_store = mutex_store.clone();
        let reader_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            Logger::info("Spawning Reader", true);
            let mut num_tuples = 0;
            let query_tuple = QueryTuple::builder().any().any().build();
            let reader_sleep = time::Duration::from_millis(500);
    
            while let Some(tuple) = reader_mutex_store.read(&query_tuple)? {
                Logger::info(&format!("Reader: Read: {:?}", tuple), true);
                num_tuples += 1;
                thread::sleep(reader_sleep);
            }
    
            Logger::info(&format!("Reader: Tuple space empty! I read {} tuples.", num_tuples), true);
            Ok(())
        });
    
        let mut getter_mutex_store = mutex_store.clone();
        let getter_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            Logger::info("Spawning getter", true);
            let mut num_tuples = 0;
            let query_tuple = QueryTuple::builder().any_integer().any_integer().build();
            let getter_sleep = time::Duration::from_millis(110);
    
            while let Some(tuple) = getter_mutex_store.get(&query_tuple)? {
                Logger::info(&format!("getter: Took: {:?}", tuple), true);
                num_tuples += 1;
                thread::sleep(getter_sleep);
            }
    
            Logger::info(&format!("getter: Tuple space empty! I took {} tuples.", num_tuples), true);
            Ok(())
        });
    
        let mut writer_mutex_store = mutex_store.clone();
        let writer_2_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            Logger::info("Writer 2", true);
            let writer_sleep = time::Duration::from_millis(100);
    
            for i in 0..100 {
                let tuple = Tuple::builder().integer(2).integer(i).build();
                Logger::info(&format!("Writer 2: Wrote: {:?}", tuple), true);
                writer_mutex_store.write(&tuple)?;
                thread::sleep(writer_sleep);
            }
            Logger::info("Writer 2: Wrote 100 tuples", true);
    
            Ok(())
        });
    
        if let Err(_) = writer_1_thread.join() {
            panic!("Writer 1 panic")
        };
        if let Err(_) = writer_2_thread.join() {
            panic!("Writer 2 panic")
        };
        if let Err(_) = getter_thread.join() {
            panic!("getter panic")
        };
        if let Err(_) = reader_thread.join() {
            panic!("Reader panic")
        };
    
        Logger::info("Finished", true);
    }
}

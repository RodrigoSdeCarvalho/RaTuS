#[cfg(test)]
mod tests {
    use std::thread::JoinHandle;
    use std::{thread, time};

    use ts_core::mutex_store::MutexStore;
    use ts_core::query_tuple::QueryTuple;
    use ts_core::result::Result;
    use ts_core::store::Store;
    use ts_core::tuple::Tuple;
    use ts_core::types::Types;
    use ts_core::vec_store::VecStore;

    use system::{Logger, set_process_name};

    #[test]
    fn test_calculate() {
        set_process_name("TS Calculate");

        let mut writer_mutex_store = MutexStore::<VecStore>::default();
        let mut adder_mutex_store = writer_mutex_store.clone();
        let mut print_mutex_store = writer_mutex_store.clone();
    
        let writer_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            let sleep = time::Duration::from_millis(100);
            for i in 0..100 {
                let tuple = Tuple::builder().integer(i).integer(i).build();
                writer_mutex_store.write(&tuple)?;
                thread::sleep(sleep);
            }
            Ok(())
        });
    
        thread::sleep(time::Duration::from_millis(200));
    
        let adder_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            let adder_query_tuple = QueryTuple::builder().any_integer().any_integer().build();
            let sleep = time::Duration::from_millis(110);
            while let Ok(Some(tuple)) = adder_mutex_store.get(&adder_query_tuple) {
                if let (Types::Integer(num_1), Types::Integer(num_2)) = (&tuple[0], &tuple[1]) {
                    let sum_tuple = Tuple::builder().integer(num_1 + num_2).build();
                    adder_mutex_store.write(&sum_tuple)?;
                }
                thread::sleep(sleep);
            }
            Ok(())
        });
    
        let printer_thread: JoinHandle<Result<()>> = thread::spawn(move || {
            let printer_query_tuple = QueryTuple::builder().any_integer().build();
            let sleep = time::Duration::from_millis(120);
            while let Ok(Some(tuple)) = print_mutex_store.get(&printer_query_tuple) {
                if let Types::Integer(num) = &tuple[0] {
                    Logger::info(&format!("Printer: {}", num), true);
                }
                thread::sleep(sleep);
            }
            Ok(())
        });
    
        if let Err(_) = writer_thread.join() {
            panic!("Writer panic")
        };
        if let Err(_) = adder_thread.join() {
            panic!("Adder panic")
        };
        if let Err(_) = printer_thread.join() {
            panic!("Printer panic")
        };
    }
}

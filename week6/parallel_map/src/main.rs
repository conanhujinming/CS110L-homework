use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let len = input_vec.len();
    
    let mut output_vec: Vec<U> = (0..len).map(|_| U::default()).collect();

    if len == 0 {
        return output_vec;
    }

    let (sender, receiver) = crossbeam_channel::unbounded::<(usize, U)>();

    let mut input_iter = input_vec.into_iter().enumerate();
    
    let chunk_size = (len + num_threads - 1) / num_threads;
    
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let chunk: Vec<(usize, T)> = input_iter.by_ref().take(chunk_size).collect();
        
        if chunk.is_empty() {
            break;
        }

        let sender_clone = sender.clone();
        
        handles.push(thread::spawn(move || {
            for (index, item) in chunk {
                let result = f(item);
                sender_clone.send((index, result)).unwrap();
            }
        }));
    }

    drop(sender);

    for _ in 0..len {
        let (idx, val) = receiver.recv().unwrap();
        output_vec[idx] = val;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    output_vec
}

fn parallel_reduce<T, F>(input_vec: Vec<T>, num_threads: usize, f: F) -> T
where
    F: Fn(T, T) -> T + Send + Copy + 'static,
    T: Send + 'static + Default,
{
    let len = input_vec.len();

    if len == 0 {
        return T::default();
    }
    
    if num_threads == 0 || num_threads == 1 {
        let mut iter = input_vec.into_iter();
        let first = iter.next().unwrap();
        return iter.fold(first, f);
    }

    let (sender, receiver) = crossbeam_channel::unbounded::<(usize, T)>();

    let chunk_size = (len + num_threads - 1) / num_threads;
    let mut handles = Vec::new();
    let mut input_iter = input_vec.into_iter();

    for i in 0..num_threads {
        let chunk: Vec<T> = input_iter.by_ref().take(chunk_size).collect();
        
        if chunk.is_empty() {
            break;
        }

        let sender_clone = sender.clone();
        
        handles.push(thread::spawn(move || {
            let mut iter = chunk.into_iter();
            
            let first = iter.next().unwrap();
            
            let local_result = iter.fold(first, |acc, x| f(acc, x));
            sender_clone.send((i, local_result)).unwrap();
        }));
    }

    drop(sender);

    let mut partial_results: Vec<(usize, T)> = receiver.iter().collect();

    for handle in handles {
        handle.join().unwrap();
    }
    
   
    partial_results.sort_by_key(|k| k.0);

    let mut result_iter = partial_results.into_iter().map(|(_, val)| val);

    if let Some(first) = result_iter.next() {
        result_iter.fold(first, f)
    } else {
        T::default()
    }
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}

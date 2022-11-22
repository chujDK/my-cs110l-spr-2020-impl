use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), U::default);
    let (input_sender, input_receiver) = crossbeam_channel::unbounded();
    let (output_sender, output_receiver) = crossbeam_channel::unbounded();
    let mut threads = vec![];
    for _ in 0..num_threads {
        let receiver = input_receiver.clone();
        let sender = output_sender.clone();
        threads.push(thread::spawn(move || {
            while let Ok((index, input)) = receiver.recv() {
                let output = f(input);
                sender.send((index, output)).unwrap();
            }
        }))
    }

    input_vec
        .into_iter()
        .enumerate()
        .for_each(|(index, input)| input_sender.send((index, input)).unwrap());
    drop(input_sender);

    while let Some(thread) = threads.pop() {
        thread.join().expect("some thread panic!");
    }
    drop(output_sender);

    while let Ok((index, output)) = output_receiver.recv() {
        output_vec[index] = output;
    }

    output_vec
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

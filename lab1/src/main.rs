mod priority_queue;

use priority_queue::PriorityQueue;

fn main() {
    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    println!("Is emppty: {}", queue.is_empty());

    queue.push(1);
    queue.push(2);
    queue.push(4);
    queue.push(16);
    queue.push(32);

    println!("Is emppty: {}, length is {}", queue.is_empty(), queue.len());

    match queue.pop() {
        Some(res) => println!(
            "After pop (which returned {res}), length is {}",
            queue.len()
        ),
        None => println!("Queue is empty"),
    };

    match queue.peek() {
        Some(res) => println!("Curently max element is {res}"),
        None => println!("Queue is empty"),
    }
}

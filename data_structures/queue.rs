pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        let mut new_tail = Box::new(Node {
            value,
            next: None,
        });
        let raw_tail: *mut _ = &mut *new_tail;
        if let Some(tail) = self.tail.take() {
            unsafe {
                (*tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = Some(raw_tail);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.next {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            head.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.dequeue().is_none());
}

/// From pythonesque


use list::Node;

mod list {
    use std::mem;

    #[derive(Show)]
    pub struct Node<T> {
        pub data: T,
        prev: Option<Box<Node<T>>>,
        next: Option<Box<Node<T>>>
    }

    impl<T> Node<T> {
        pub fn new(data: T) -> Node<T> {
            Node { data: data, prev: None, next: None }
        }

        pub fn insert_after(&mut self, mut new: Box<Node<T>>) -> Option<Box<Node<T>>> {
            mem::swap(&mut new.next, &mut self.next);
            mem::replace(&mut self.next, Some(new))
        }

        pub fn insert_before(&mut self, mut new: Box<Node<T>>) -> Option<Box<Node<T>>> {
            mem::swap(&mut new.prev, &mut self.prev);
            mem::replace(&mut self.prev, Some(new))
        }

        pub fn remove_after(&mut self) -> Option<Box<Node<T>>> {
            match self.next.take() {
                Some(mut next) => {
                    mem::swap(&mut self.next, &mut next.next);
                    Some(next)
                },
                None => None,
            }
        }

        pub fn remove_before(&mut self) -> Option<Box<Node<T>>> {
            match self.prev.take() {
                Some(mut prev) => {
                    mem::swap(&mut self.prev, &mut prev.prev);
                    Some(prev)
                },
                None => None,
            }
        }

        pub fn next(&mut self) -> Option<&mut T> {
            let Node { ref mut data, ref mut prev, ref mut next } = *self;
            match *next {
                Some(ref mut next) => {
                    mem::swap(&mut next.data, data);
                    {
                        let mut next = &mut **next;
                        mem::swap(&mut next.next, &mut next.prev);
                    }
                    mem::swap(&mut next.prev, prev);
                },
                None => return None,
            }
            mem::swap(prev, next);
            Some(data)
        }

        pub fn prev(&mut self) -> Option<&mut T> {
            let Node { ref mut data, ref mut prev, ref mut next } = *self;
            match *prev {
                Some(ref mut prev) => {
                    mem::swap(&mut prev.data, data);
                    {
                        let mut prev = &mut **prev;
                        mem::swap(&mut prev.next, &mut prev.prev);
                    }
                    mem::swap(&mut prev.next, next);
                },
                None => return None
            }
            mem::swap(prev, next);
            Some(data)
        }
    }
}

fn main() {
    let mut list = Node::new(0i8);
    list.insert_before(box Node::new(-1));
    list.insert_after(box Node::new(1));
    while let Some(_) = list.next() {
        println!("{}", list);
    }
    list.insert_before(box Node::new(2));
    while let Some(_) = list.prev() {
        println!("{}", list.remove_before());
        println!("{}", list);
    }
    println!("{}", list.remove_after());
}

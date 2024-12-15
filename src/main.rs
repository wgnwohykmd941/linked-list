//------------------------------simplified version-----------------------------
// enum List <T> {
//     Node (T, Box<List<T>>),
//     Nil
// }
// []-> stack ()-> heap
//[node1,ptr] -> (node2 ,ptr) -> (node3, nil)
// fn main() {
//     let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
//     println!("{:?}", list);
// }
//enum {A(i8),B(i16),C(i32)} => struct {tag:u8, data:i32(the max)}
//tag => 0,1,2 ,0 => A, 1=>B, 2=>C
//--------------------------------improved version1---------------------------
// #[derive(Debug)]
// struct ListNode<T> {
//     data: T,
//     nextnode: Link,
// }
// #[derive(Debug)]
// enum Link {
//     Nil,
//     Node(Box<ListNode<i32>>),
// }
// #[derive(Debug)]
// struct  MyList {
//     head: Link，
// }
// ----------------------improved version2(头插链表)-----------------------------
//[ptr] -> (node2 ,ptr) -> (node3, null)\
#[derive(Debug)]
struct  Node {
    data: i32,
    next: Option<Box<Node>>,
}
#[derive(Debug)]
struct  MyList {
    head: Option<Box<Node>>,
}

impl MyList {
    fn new() -> Self {
        MyList { head:None}
    }
    
    fn push(&mut self,elem: i32) {
        let node = Box::new(
            Node {
                data: elem,
                next: self.head.take(),
            }
        );
        self.head = Some(node);
    }//头插
    fn pop(&mut self) {
        match self.head.take() {
            None => panic!("empty list"),
            Some(Node) => self.head = Node.next,
        }
    }
    fn split() {
        todo!()
    }
    fn merge() {
        todo!()
    }
}

fn main() {
    let mut  list = MyList::new();
    println!("{:?}", list);
    list.push(1);
    println!("{:?}", list);
    list.push(2);
    println!("{:?}", list);
    list.pop();
    println!("{:?}", list);
}

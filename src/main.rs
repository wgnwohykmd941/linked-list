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
// struct  Node {
//     data: i32,
//     next: Option<Box<Node>>,
// }
// struct  MyList {
//     head: Option<Box<Node>>,
// }
// -------------------------------improved version2-----------------------------
//[ptr] -> (node2 ,ptr) -> (node3, null)\

#[derive(Debug)]
struct ListNode<T> {
    ndata: T,
    nextnode: Link,
}
#[derive(Debug)]
enum Link {
    Nil,
    Node(Box<ListNode<i32>>),
}
#[derive(Debug)]
struct  MyList {
    list: Link,
}

fn main() {
    todo!()
}

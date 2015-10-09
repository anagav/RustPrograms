extern crate std;

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T>{
	fn new(data:T) -> Self {
		 Node { data:data, left:None, right:None }
	}
}


// fn in_order_traversal<T>(head: &Node<i32>) {
// 	match *head {
//
//         Node::<i32>{data,ref left, ref right} => {
//                 match *left {
//                     Some(ref value) => in_order_traversal::<T>(value),
//                     None => {}
//                 }
//                 format!("{:?}", data);
//                 match *right {
//                     Some(ref value) => in_order_traversal::<T>(value),
//                     None => {}
//                 }
//             }
//
// 	}
// }


fn max_depth_tree<T: std::fmt::Display>(node: &Option<Box<Node<T>>>) -> i32 {
    match *node {
        None => {
            return 0;
        },
        Some(ref node) => {
            let left_height = max_depth_tree::<T>(&node.left);
            let right_height = max_depth_tree::<T>(&node.right);
            return if left_height > right_height { left_height + 1 } else { right_height + 1 }
        }
    }
}


pub fn run(){
    let mut root: Node<i32> = Node::new(3);
    root.left = Some(Box::new(Node::new(4)));
    root.left.as_mut().unwrap().left = Some(Box::new(Node::new(6)));
    root.left.as_mut().unwrap().right = Some(Box::new(Node::new(7)));
    root.right = Some(Box::new(Node::new(5)));
    println!("{}{}","depth of binary tree:\t",max_depth_tree::<i32>(&Some(Box::new(root))));
	//println!("{:?}", root);
}

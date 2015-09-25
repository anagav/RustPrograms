#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T>{
	fn new(data:T) -> Node<T> {
		 Node { data:data, left:None, right:None }
	}
}


fn in_order_traversal<T>(head: &Node<i32>) {
	match *head {

        Node::<i32>{data,ref left, ref right} => {
                match *left {
                    Some(ref value) => in_order_traversal::<T>(value),
                    None => {}
                }
                println!("{:?}", data);
                match *right {
                    Some(ref value) => in_order_traversal::<T>(value),
                    None => {}
                }
            }

	}
}



pub fn run(){
    let mut root: Node<i32> = Node::new(3);
    root.left = Some(Box::new(Node::new(4)));
    root.left.as_mut().unwrap().left = Some(Box::new(Node::new(6)));
    root.left.as_mut().unwrap().right = Some(Box::new(Node::new(7)));
    root.right = Some(Box::new(Node::new(5)));
    in_order_traversal::<i32>(&root);
	//println!("{:?}", root);
}

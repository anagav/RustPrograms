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


fn in_order_traversal<T>(head: &Node<T>) {
	match *head {

        Node{data,left,right} => {
                println!("{:?}", data);
                in_order_traversal::<T>(left.as_ref::<'r>().unwrap());
                in_order_traversal::<T>(right.as_mut::<'r>().unwrap());
            }

	}
}



pub fn run(){
	let mut root: Node<i32> = Node::new(3);
	root.left = Some(Box::new(Node::new(4)));
	root.left.as_mut().unwrap().left = Some(Box::new(Node::new(6)));
	root.left.as_mut().unwrap().right = Some(Box::new(Node::new(7)));
	root.right = Some(Box::new(Node::new(5)));

	println!("{:?}", root);
}

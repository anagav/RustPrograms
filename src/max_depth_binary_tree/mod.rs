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




pub fn run(){
	let root: Node<i32> = Node::new(3);

	println!("{:?}", root);
}
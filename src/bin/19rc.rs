use std::rc::Rc;

//Rc<T>内部的数据不可变
enum GraphNode {
    Const(i32, Rc<GraphNode>),
    Nil,
}

impl GraphNode {
    fn node(v: i32, node: Rc<GraphNode>) -> Rc<Self> {
        return Rc::new(GraphNode::Const(v, node));
    }

    fn end(v: i32) -> Rc<Self> {
        return Rc::new(GraphNode::Const(v, Rc::new(GraphNode::Nil)));
    }
}

fn main() {
    /*
    1
      ↘
        5 -> 10 -> nil
      ↗
    2
    */
    let common = GraphNode::node(5, GraphNode::end(10));
    println!("Rc::new会初始化引用计数,{}", Rc::strong_count(&common));
    let line1 = GraphNode::node(1, Rc::clone(&common));
    {
        let line2 = GraphNode::node(2, Rc::clone(&common));
        println!("Rc::clone会增加引用计数{}", Rc::strong_count(&common));
    }
    println!("引用被清理后,会减少引用计数{}", Rc::strong_count(&common));
}
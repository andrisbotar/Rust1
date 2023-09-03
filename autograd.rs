use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    val: f64,
    grad: f64,
    adjacency : Vec<u64>, //maybe option for efficiency, for params which have no inputs?
    function : Function
}

impl Node {
    fn new(val : f64, adjacency : Vec<u64>, function : Function)-> Self{
        Node{
            val,
            grad: 0.0,
            adjacency,
            function,
        }
    }
    fn new_disconnected(val : f64)-> Self{
        Node{
            val,
            grad: 0.0,
            adjacency: Vec::new(),
            function: Function::Noop,
        }
    }
    fn new_empty()-> Self{
        Self::new_disconnected(0.0)
    }
}

#[derive(Clone)]
pub enum Function {
    Add,
    Multiply,
    Noop
}

pub fn forward(mut nodes : HashMap<u64, Node>, index : &u64){
    let node: &Node = nodes.get(index).unwrap();
    let val = match node.function {
        Function::Add => {
            node.adjacency.clone().into_iter().map(
                |n| nodes.get(&n).unwrap().val
            ).sum()
        },
        Function::Multiply => {
            node.adjacency.clone().into_iter().map(
                |n| nodes.get(&n).unwrap().val
            ).product()
        }
        Function::Noop =>
            node.val,
    };
    nodes.get_mut(index).unwrap().val = val;
}

pub fn backward(mut nodes : HashMap<u64, Node>, index : &u64){
    let temp = nodes.clone();
    let node: &Node = temp.get(index).unwrap();
    for neighbour in &node.adjacency {
        let grad = match node.function {
            Function::Add => {
                nodes.get_mut(&neighbour).unwrap().grad = node.grad;
                //nodes.insert(*index, Node::new_empty()).unwrap();
            },
            Function::Multiply => {
                //nodes.insert();
            },
            Function::Noop => {},
        };
    }
}

pub fn autograd_test() {
    let mut nodes : HashMap<u64, Node> = HashMap::new();




    /*let add : Add = Add::new(1,2,4);
    let mult : Multiply = Multiply::new(3,4,5);

    add.forward(&mut values);
    mult.forward(&mut values);

    mult.backward(&mut values,&mut grads);
    add.backward(&mut values,&mut grads);

    //do toposort

    //println!("{:#?}",values);
    println!("{:?}",values.iter());
    println!("{:?}",grads);*/
}
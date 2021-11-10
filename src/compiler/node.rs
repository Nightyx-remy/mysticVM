pub enum Node {
    Value(ValueNode),
    BinOP(Box<Node>, Operator, Box<Node>),
    VariableDefinition(String, Box<Node>),
    VariableCall(String),
}

impl Node {

    pub fn get_weight(&self) -> usize {
        return match self {
            Node::Value(_) => 0,
            Node::BinOP(_, _, _) => 1,
            Node::VariableDefinition(_, _) => 2,
            Node::VariableCall(_) => 0,
        }
    }

}

pub enum ValueNode {
    U8(u8),
}

pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE
}
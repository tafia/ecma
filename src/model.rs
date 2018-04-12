pub struct Position {
    pub row: u32,
    pub col: u32,
}

pub struct SourceLocation<Idx> {
    pub src: String,
    pub span: ::std::ops::Range<Idx>,
}

pub enum Node {
    Program {
        body: Vec<Node>,
    },
    Function {
        id: Option<Identifier>,
        params: Vec<Pattern>,
        defaults: Vec<Expression>,
        rest: Option<Identifier>,
        body: BlockStatement,
    },
    Statement(Statement),
}

pub enum Statement {
    Empty,
    Block(BlockStatement),
    Expression {
        expression: Expression,
    },
    If {
        test: Expression,
        consequence: Expression,
        alternate: Option<Expression>,
    },
    Labeled {
        label: Identifier,
        body: Box<Statement>,
    },
    Break {
        label: Option<Identifier>,
    },
    Continue {
        label: Option<Identifier>,
    },
    With {
        object: Expression,
        body: Box<Statement>,
    },
    Switch {
        discriminant: Expression,
        cases: Vec<SwitchCase>,
        lexical: bool,
    },
    Return {
        argument: Option<Expression>,
    },
    Throw {
        argument: Option<Expression>,
    },
    Try {
        block: BlockStatement,
        handler: Option<CatchClause>,
        finalizer: Option<BlockStatement>,
    },
    While {
        test: Expression,
        body: Box<Statement>,
    },
    DoWhile {
        body: Box<Statement>,
        test: Expression,
    },
    For {
        init: Option<VariableDeclOrExpression>,
        test: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
    },
    ForIn {
        left: Box<VariableDeclOrExpression>,
        right: Expression,
        body: Box<Statement>,
    },
    ForOf {
        left: Box<VariableDeclOrExpression>,
        right: Expression,
        body: Box<Statement>,
    },
    Debugger,
    Declaration(Declaration),
}

pub enum VariableDeclOrExpression {
    VariableDecl(VariableDeclarator),
    Expression(Expression),
}

pub struct BlockStatement {
    pub body: Vec<Statement>,
}

pub enum Declaration {
    Function {
        id: Identifier,
        params: Vec<Pattern>,
        defaults: Vec<Expression>,
        rest: Option<Identifier>,
        body: BlockStatement,
    },
    Variable {
        declaration: Vec<VariableDeclarator>,
        kind: VariableKind,
    },
}

pub enum VariableKind {
    Var,
    Let,
    Const,
}

pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
}

pub enum Expression {
    This,
    Array {
        elements: Vec<Option<Expression>>,
    },
    Object {
        properties: Vec<Property>,
    },
    Function {
        id: Option<Identifier>,
        params: Vec<Pattern>,
        defaults: Vec<Expression>,
        rest: Option<Identifier>,
        body: BlockStatement,
    },
    Arrow {
        params: Vec<Pattern>,
        defaults: Vec<Expression>,
        rest: Option<Identifier>,
        body: BlockOrExpression,
    },
    Sequence {
        expressions: Vec<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        prefix: bool,
        argument: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assignment {
        operator: AssignmentOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Update {
        operator: UpdateOperator,
        argument: Box<Expression>,
        prefix: bool,
    },
    Logical {
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Conditional {
        test: Box<Expression>,
        alternate: Box<Expression>,
        consequent: Box<Expression>,
    },
    New {
        callee: Box<Expression>,
        arguments: Vec<Option<Expression>>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Option<Expression>>,
    },
    Member {
        object: Box<Expression>,
        property: IdentifierOrExpression,
        computed: bool,
    },
    Yield {
        argument: Option<Box<Expression>>,
    },
    Conprehension {
        body: Box<Expression>,
        blocks: Vec<ComprehensionBlock>,
        filter: Option<Box<Expression>>,
    },
    Generator {
        body: Box<Expression>,
        blocks: Vec<ComprehensionBlock>,
        filter: Option<Box<Expression>>,
    },
    Let {
        head: Vec<LetHead>,
    },
}

pub enum BlockOrExpression {
    Block(BlockStatement),
    Expression(Box<Expression>),
}

pub enum IdentifierOrExpression {
    Identifier(Identifier),
    Expression(Box<Expression>),
}

pub struct LetHead {
    pub id: PatternOrExpression,
    pub body: Expression,
}

pub enum PatternOrExpression {
    Pattern(Pattern),
    Expression(Expression),
}

pub enum Pattern {
    Object { properties: Vec<PropertyPattern> },
    Property(Box<PropertyPattern>),
    Array { elements: Vec<Option<Pattern>> },
}

pub struct PropertyPattern {
    pub key: LiteralOrIdentifier,
    pub value: Pattern,
}

pub enum LiteralOrIdentifier {
    Literal(Literal),
    Identifier(Identifier),
}

pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

pub struct CatchClause {
    pub param: Pattern,
    pub body: BlockStatement,
}

pub struct ComprehensionBlock {
    pub left: Pattern,
    pub right: Expression,
    pub each: bool,
}

pub struct Identifier {
    pub name: String,
}

pub enum Literal {
    String(String),
    Boolean(bool),
    Null,
    Number(isize),
    RegExp(String),
}

pub struct Property {
    pub key: LiteralOrIdentifier,
    pub value: Expression,
    pub kind: PropertyKind,
}

pub enum PropertyKind {
    Init,
    Get,
    Set,
}

pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
    TypeOf,
    Void,
    Delete,
    InstanceOf,
}

pub enum BinaryOperator {
    Equal,
    NotEqual,
    EqualType,
    NotEqualType,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    ShiftLeft,
    ShiftRight,
    ShiftRightZero,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Or,
    Exp,
    In,
    InstanceOf,
}

pub enum LogicalOperator {
    And,
    Or,
}

pub enum AssignmentOperator {
    Set,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    ShL,
    ShR,
    ShRZero,
    Or,
    And,
    Xor,
}

pub enum UpdateOperator {
    Add,
    Sub,
}

//! TRU Language AST (Abstract Syntax Tree)
//! All node types produced by the parser.

use crate::tru_lang::lexer::Span;

/// A complete .tru source file
#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

// ─── Statements ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(LetStmt),
    Fn(FnDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Impl(ImplBlock),
    Trait(TraitDecl),
    Return(Option<Expr>, Span),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Break(Span),
    Continue(Span),
    Expr(Expr),
    Block(Vec<Stmt>, Span),
    Use(UsePath, Span),
    Mod(String, Vec<Stmt>, Span),
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    pub mutable: bool,
    pub ty: Option<TypeExpr>,
    pub value: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Option<TypeExpr>,
    pub body: Vec<Stmt>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: TypeExpr,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<StructField>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: TypeExpr,
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<TypeExpr>, // tuple variants
}

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub ty: String,
    pub trait_name: Option<String>,
    pub methods: Vec<FnDecl>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub name: String,
    pub methods: Vec<FnSignature>,
    pub is_pub: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnSignature {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Option<TypeExpr>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub then_block: Vec<Stmt>,
    pub else_block: Option<Vec<Stmt>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub var: String,
    pub iter: Expr,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UsePath {
    pub segments: Vec<String>,
    pub alias: Option<String>,
}

// ─── Expressions ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64, Span),
    Float(f64, Span),
    Str(String, Span),
    Bool(bool, Span),
    Nil(Span),

    Ident(String, Span),
    Array(Vec<Expr>, Span),
    Tuple(Vec<Expr>, Span),
    Map(Vec<(Expr, Expr)>, Span),

    Binary(Box<Expr>, BinOp, Box<Expr>, Span),
    Unary(UnOp, Box<Expr>, Span),
    Assign(Box<Expr>, AssignOp, Box<Expr>, Span),

    Call(Box<Expr>, Vec<Expr>, Span),
    Index(Box<Expr>, Box<Expr>, Span),
    Field(Box<Expr>, String, Span),
    MethodCall(Box<Expr>, String, Vec<Expr>, Span),

    StructInit(String, Vec<(String, Expr)>, Span),
    EnumVariant(String, String, Vec<Expr>, Span),

    If(Box<IfExpr>),
    Match(Box<MatchExpr>),
    Block(Vec<Stmt>, Option<Box<Expr>>, Span),
    Closure(Vec<Param>, Option<TypeExpr>, Box<Expr>, Span),
    Range(Box<Expr>, Box<Expr>, bool, Span), // start..end or start..=end
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub cond: Expr,
    pub then_val: Expr,
    pub else_val: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchExpr {
    pub subject: Expr,
    pub arms: Vec<MatchArm>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

// ─── Patterns ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
    Tuple(Vec<Pattern>),
    Struct(String, Vec<(String, Pattern)>),
    EnumVariant(String, String, Vec<Pattern>),
    Or(Vec<Pattern>),
}

// ─── Types ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Simple(String),
    Generic(String, Vec<TypeExpr>),
    Ref(Box<TypeExpr>, bool), // &T / &mut T
    Slice(Box<TypeExpr>),
    Array(Box<TypeExpr>, usize),
    Tuple(Vec<TypeExpr>),
    Fn(Vec<TypeExpr>, Box<TypeExpr>),
    Option(Box<TypeExpr>),
    Result(Box<TypeExpr>, Box<TypeExpr>),
    Never,     // !
    Unit,      // ()
    Infer,     // _
}

// ─── Operators ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Neg,   // -
    Not,   // !
    Ref,   // &
    Deref, // *
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Assign,        // =
    AddAssign,     // +=
    SubAssign,     // -=
    MulAssign,     // *=
    DivAssign,     // /=
}

impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Expr::Int(_, s) | Expr::Float(_, s) | Expr::Str(_, s)
            | Expr::Bool(_, s) | Expr::Nil(s) | Expr::Ident(_, s)
            | Expr::Array(_, s) | Expr::Tuple(_, s) | Expr::Map(_, s)
            | Expr::Binary(_, _, _, s) | Expr::Unary(_, _, s)
            | Expr::Assign(_, _, _, s) | Expr::Call(_, _, s)
            | Expr::Index(_, _, s) | Expr::Field(_, _, s)
            | Expr::MethodCall(_, _, _, s) | Expr::StructInit(_, _, s)
            | Expr::EnumVariant(_, _, _, s) | Expr::Block(_, _, s)
            | Expr::Closure(_, _, _, s) | Expr::Range(_, _, _, s) => s,
            Expr::If(b) => &b.span,
            Expr::Match(b) => &b.span,
        }
    }
}

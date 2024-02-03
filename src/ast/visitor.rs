use super::{Ast, BinOp, Expr, ExprKind, Item, ItemKind, Literal, Stmt, StmtKind, UnOp};

pub trait Walkable {
    fn walk<V: Visitor>(&self, visitor: &mut V);
}

pub trait Visitor: Sized {
    fn visit_ast(&mut self, ast: &Ast) {
        for item in &ast.items {
            item.walk(self);
        }
    }

    fn visit_item(&mut self, item: &Item) {
        item.walk(self);
    }

    fn visit_item_kind(&mut self, kind: &ItemKind) {
        kind.walk(self);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        stmt.walk(self);
    }

    fn visit_stmt_kind(&mut self, stmt: &StmtKind) {
        stmt.walk(self);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        expr.walk(self);
    }

    fn visit_expr_kind(&mut self, expr: &ExprKind) {
        expr.walk(self);
    }

    fn visit_binary(&mut self, _op: &BinOp, lhs: &Expr, rhs: &Expr) {
        lhs.walk(self);
        rhs.walk(self);
    }

    fn visit_unary(&mut self, _op: &UnOp, expr: &Expr) {
        expr.walk(self);
    }

    fn visit_literal(&mut self, lit: &Literal) {
        lit.walk(self);
    }
}

impl Walkable for Item {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_item_kind(&self.kind);
    }
}

impl Walkable for ItemKind {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self {
            ItemKind::Statement(stmt) => visitor.visit_stmt(stmt),
        }
    }
}

impl Walkable for Stmt {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_stmt_kind(&self.kind);
    }
}

impl Walkable for StmtKind {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self {
            StmtKind::If(cond, res) => {
                visitor.visit_expr(cond);
                for item in res {
                    visitor.visit_item(item);
                }
            }
            StmtKind::While(cond, res) => {
                visitor.visit_expr(cond);
                for item in res {
                    visitor.visit_item(item);
                }
            }
            StmtKind::Let(.., expr) => {
                //visitor.visit_expr_kind(ident); // TODO: FIX Ident DEclaration
                visitor.visit_expr(expr)
            },
            StmtKind::Error => {}
        }
    }
}

impl Walkable for Expr {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_expr_kind(&self.kind);
    }
}

impl Walkable for ExprKind {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self {
            ExprKind::Literal(value) => visitor.visit_literal(value),
            ExprKind::Binary(op, lhs, rhs) => visitor.visit_binary(op, &lhs.ptr, &rhs.ptr),
            ExprKind::Unary(op, expr) => visitor.visit_unary(op, &expr.ptr),
            ExprKind::Ident(_) => {}
            ExprKind::Error => {}
        }
    }
}

impl Walkable for Literal {
    fn walk<V: Visitor>(&self, _visitor: &mut V) {
        match &self {
            Literal::String(_string) => {},
            Literal::Integer(_int) => {},    
        }
    }
}

use core::panic;
use std::{collections::HashMap, io};

use crate::{lexer::Token, parser::Expr};

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

pub struct Interpreter<'a> {
    exprs: &'a [Expr],
    variables: HashMap<String, Value>,
}

impl<'a> Interpreter<'a> {
    pub fn new(exprs: &'a [Expr]) -> Self {
        Interpreter { 
            exprs,
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) {
        for expr in self.exprs {
            self.eval_stmt(expr);
        }
    }

    fn eval_stmt(&mut self, expr: &Expr) {
        match expr {
            Expr::Print(ref expr) => self.print(expr),
            Expr::Input(ref prompt, ref out) => self.input(prompt, out),
            Expr::VarDec(ref id, ref expr) => self.var_dec(id, expr),
            Expr::Assign(ref id, ref expr) => self.assign_expr(id, expr),
            Expr::Bin(ref l, ref op, ref r) => { self.bin_expr(l, op, r); },
            Expr::Rel(ref l, ref op, ref r) => { self.rel_expr(l, op, r); },
            Expr::If(ref cond, ref cons) => self.if_stmt(cond, cons),
            Expr::For { ref variable, ref start, ref end, ref step, ref body } => {
                self.for_loop(variable, start, end, step.as_deref(), body)
            },
            _ => { self.eval_expr(expr); },
        }
    }
    
    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Num(n) => Value::Integer(*n),
            Expr::Str(s) => Value::String(s.to_string()),
            Expr::Bin(l, op, r) => self.bin_expr(l, op, r),
            Expr::Rel(l, op, r) => self.rel_expr(l, op, r),
            Expr::Call(func, args) => self.call_stmt(func, args),
            Expr::Identifier(id) => self.variables.get(id).cloned().unwrap_or_else(|| panic!("Undefined variable: {}", id)),
            _ => panic!("Unknown expression in interpreter"),
        }
    }

    fn rel_expr(&mut self, left: &Box<Expr>, op: &Token, right: &Box<Expr>) -> Value {
        let lval = self.eval_expr(left);
        let rval = self.eval_expr(right);

        match (lval, rval) {
            (Value::Integer(lval), Value::Integer(rval)) => {
                match op {
                    Token::RelOp(op_str) => match op_str.as_str() {
                        ">" => Value::Bool(lval > rval),
                        "<" => Value::Bool(lval < rval),
                        "<=" => Value::Bool(lval <= rval),
                        ">=" => Value::Bool(lval >= rval),
                        "=" => Value::Bool(lval == rval),
                        "<>" => Value::Bool(lval != rval),
                        _ => panic!(""),
                    },
                    _ => panic!("Invalid expression for relational comparison"),
                }
            },
            _ => panic!("Invalid type for relational comparison"),
        }
    }

    fn bin_expr(&mut self, left: &Box<Expr>, op: &Token, right: &Box<Expr>) -> Value {
        let lval = self.eval_expr(left);
        let rval = self.eval_expr(right);

        match (lval, rval) {
            (Value::Integer(lval), Value::Integer(rval)) => {
                match op {
                    Token::BinOp(op_str) => match op_str.as_str() {
                        "+" => Value::Integer(lval + rval),
                        "-" => Value::Integer(lval - rval),
                        "*" => Value::Integer(lval * rval),
                        "/" => {
                            if rval == 0 {
                                panic!("Division by zero")
                            } else {
                                Value::Integer(lval / rval)
                            }
                        },
                        "%" => Value::Integer(lval % rval),
                        _ => panic!("Unknown operator in binary expression")
                    },
                    _ => panic!(""),
                }
            },
            _ => panic!("Invalid types for arithmetic operation"),
        }
    }

    fn for_loop(&mut self, variable: &String, start: &Box<Expr>, end: &Box<Expr>, step: Option<&Expr>, body: &[Expr]) {
        let mut start_val = if let Value::Integer(v) = self.eval_expr(start) { v } else { panic!("For loop start must be an integer"); };
        let end_val = if let Value::Integer(v) = self.eval_expr(end) { v } else { panic!("For loop end must be an integer"); };
        let step_val = step.map_or(1, |step_expr| {
            if let Value::Integer(v) = self.eval_expr(step_expr) {
                v
            } else {
                panic!("For loop step must be an integer")
            }
        });

        self.variables.insert(variable.clone(), Value::Integer(start_val));
        while (step_val > 0 && start_val <= end_val) || (step_val < 0 && start_val >= end_val) {
            for stmt in body {
                self.eval_stmt(stmt);
            }

            start_val += step_val;
            self.variables.insert(variable.clone(), Value::Integer(start_val));
        }
    }

    fn if_stmt(&mut self, cond: &Box<Expr>, cons: &Box<Expr>) {
        let condition = self.eval_expr(cond);

        match condition {
            Value::Bool(true) => {
                self.eval_stmt(cons);
            },
            Value::Bool(false) => {},
            _ => panic!("If statement must evaluate to a boolean")
        }
        
    }

    fn assign_expr(&mut self, id: &String, expr: &Box<Expr>) {
        if !self.variables.contains_key(id) {
            panic!("Cannot assign undefined variable");
        }

        let val = self.eval_expr(expr);
        self.variables.insert(id.to_string(), val);
    }

    fn var_dec(&mut self, id: &String, expr: &Box<Expr>) {
        let val = match expr.as_ref() {
            Expr::Call(func, args) => self.call_stmt(func, args),
            _ => self.eval_expr(expr),
        };
        self.variables.insert(id.to_string(), val);
    }
    
    fn print(&mut self, expr: &Box<Expr>) {
        let result = self.eval_expr(expr);
        match result {
            Value::Integer(n) => println!("{}", n),
            Value::String(s) => println!("{}", s),
            Value::Bool(b) => println!("{}", b),
            Value::Float(f) => println!("{}", f),
            _ => panic!("Invalid type for print")
        }
    }

    fn call_stmt(&mut self, func_name: &String, args: &Vec<Expr>) -> Value {
        match func_name.as_str() {
            "SIN" => {
                if args.len() != 1 {
                    panic!("SIN function takes exactly one argument");
                }
    
                let arg = &args[0];
                let val = self.eval_expr(arg);
    
                match val {
                    Value::Integer(n) => {
                        let radians = (n as f64).to_radians();
                        let result = f64::sin(radians);
                        return Value::Float(result)
                    }
                    _ => panic!("SIN function expects an integer argument"),
                }
            }
            _ => panic!("Undefined function: {}", func_name),
        }
    }
    

    fn input(&mut self, prompt: &String, out: &Box<Expr>) {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if let Expr::Identifier(id) = &**out {
            if let Ok(num) = input.parse::<i64>() {
                self.variables.insert(id.clone(), Value::Integer(num));
            } else {
                self.variables.insert(id.clone(), Value::String(input.to_string()));
            }
        } else {
            panic!("Invalid expression for input");
        }
    }
}
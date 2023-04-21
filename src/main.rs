use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        ArithExpr(expr) => IntValue(eval_arith_expr(expr)),
        BoolExpr(expr)  => BoolValue(eval_bool_expr(expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr {
        BinArithExpr { left, right, op }    => 
        {
            
            match op {
                AddOp       => eval_arith_expr(*left) + eval_arith_expr(*right),
                SubOp       => eval_arith_expr(*left) - eval_arith_expr(*right),
                MulOp       => eval_arith_expr(*left) * eval_arith_expr(*right),
                IntDivOp    => eval_arith_expr(*left) / eval_arith_expr(*right),
            }
        },
        IntLit(num)                         => num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr {
        ArithCmpExpr {left, right, op}  => 
        {
            match op {
                LtOp        => eval_arith_expr(*left) < eval_arith_expr(*right),
                LteOp       => eval_arith_expr(*left) <= eval_arith_expr(*right),
                GtOp        => eval_arith_expr(*left) > eval_arith_expr(*right),
                GteOp       => eval_arith_expr(*left) >= eval_arith_expr(*right),
                ArithEqOp   => eval_arith_expr(*left) == eval_arith_expr(*right),
                ArithNeqOp  => eval_arith_expr(*left) != eval_arith_expr(*right),
            }
        },
        BinBoolExpr {left, right, op}   => 
        {
            match op {
                AndOp       => eval_bool_expr(*left) && eval_bool_expr(*right),
                OrOp        => eval_bool_expr(*left) || eval_bool_expr(*right),
                BoolEqOp    => eval_bool_expr(*left) == eval_bool_expr(*right),
                BoolNeqOp   => eval_bool_expr(*left) != eval_bool_expr(*right),
            }
        },
        NotExpr(expr)                   => !eval_bool_expr(*expr),
        BoolLit(bool)                   => bool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ArithExpr_IntLit() {
        let expr = ArithExpr(IntLit(0));
        let answer = IntValue(0);

        assert_eq!(eval(expr), answer); 
    }

    #[test]
    fn test_ArithExpr_Add() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: AddOp});
        let answer = IntValue(10);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_ArithExpr_Sub() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: SubOp});
        let answer = IntValue(0);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_ArithExpr_Mul() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), right: Box::new(IntLit(5)), op: MulOp});
        let answer = IntValue(5);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_ArithExpr_Div() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: IntDivOp});
        let answer = IntValue(5);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BoolLit() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_BoolExpr_ArithCmpExpr_LtT() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), right: Box::new(IntLit(5)), op: LtOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }  

    #[test]
    fn test_BoolExpr_ArithCmpExpr_LtF() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(5)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: LtOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }   

    #[test]
    fn test_BoolExpr_ArithCmpExpr_LteT() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), right: Box::new(IntLit(5)), op: LteOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }  
    
    #[test]
    fn test_BoolExpr_ArithCmpExpr_LteT2() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), right: Box::new(IntLit(1)), op: LteOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }  

    #[test]
    fn test_BoolExpr_ArithCmpExpr_LteF() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(5)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: LteOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_ArithCmpExpr_GtF() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), right: Box::new(IntLit(5)), op: GtOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }  

    #[test]
    fn test_BoolExpr_ArithCmpExpr_GtT() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(5)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: GtOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_ArithCmpExpr_GteF() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), right: Box::new(IntLit(5)), op: GteOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }  
    
    #[test]
    fn test_BoolExpr_ArithCmpExpr_GteT() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), right: Box::new(IntLit(1)), op: GteOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }  

    #[test]
    fn test_BoolExpr_ArithCmpExpr_GteT2() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(5)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: GteOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_ArithCmpExpr_EqT() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(1)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: ArithEqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_ArithCmpExpr_EqF() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(2)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: ArithEqOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_ArithCmpExpr_NeqF() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(1)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: ArithNeqOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_ArithCmpExpr_NeqT() {
        let expr = BoolExpr(ArithCmpExpr {left: Box::new(IntLit(2)), right: Box::new(BinArithExpr { left: Box::new(IntLit(5)), right: Box::new(IntLit(5)), op: IntDivOp}), op: ArithNeqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_AndT() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: AndOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_AndF() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_OrT() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp}), right: Box::new(BoolLit(true)), op: OrOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_OrF() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp}), right: Box::new(BoolLit(false)), op: OrOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_EqT() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp}), right: Box::new(BoolLit(false)), op: BoolEqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_EqF() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp}), right: Box::new(BoolLit(true)), op: BoolEqOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_NeqF() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp}), right: Box::new(BoolLit(false)), op: BoolNeqOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_NeqT() {
        let expr = BoolExpr(BinBoolExpr {left: Box::new(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp}), right: Box::new(BoolLit(true)), op: BoolNeqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_BoolExpr_BinBoolExpr_Not() {
        let expr = BoolExpr(NotExpr(Box::new(BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: AndOp})));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}

fn main() {}
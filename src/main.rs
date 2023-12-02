fn main() {}

type Scaler = isize;
#[derive(Debug, Clone, PartialEq)]
enum Base {
    Scaler(Scaler),
    X(X),
    Exp(Exp),
    Pow(Pow),
    Log(Log),
    Sin(Sin),
    Cos(Cos),
    Tan(Tan),
}

impl Base {
    fn diff(&self) -> Expression {
        match self {
            Self::Scaler(_) => Expression::Base(Base::Scaler(0)),
            Self::X(x) => x.diff(),
            Self::Exp(exp) => exp.diff(),
            Self::Pow(pow) => pow.diff(),
            Self::Log(log) => log.diff(),
            Self::Sin(sin) => sin.diff(),
            Self::Cos(cos) => cos.diff(),
            Self::Tan(tan) => tan.diff(),
        }
    }
}

type Add = Vec<Expression>;
type Mul = Vec<Expression>;

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Base(Base),
    Add(Add),
    Mul(Mul),
}

impl Expression {
    fn diff(&self) -> Expression {
        match self {
            Self::Base(base) => base.diff(),
            Self::Add(add) => Self::Add(add.iter().map(|e| e.diff()).collect()),
            Self::Mul(mul) => {
                let mut expressions = vec![];
                for i in 0..mul.len() {
                    let mut row = vec![];
                    for (j, expression) in mul.iter().enumerate() {
                        if i == j {
                            row.push(expression.diff())
                        } else {
                            row.push(expression.clone())
                        }
                    }
                    expressions.push(Self::Mul(row));
                }
                Self::Add(expressions)
            }
        }
    }
}

trait DiffBase {
    fn diff(&self) -> Expression;
}

#[derive(Debug, Clone, PartialEq)]
struct X {}

impl DiffBase for X {
    fn diff(&self) -> Expression {
        Expression::Base(Base::Scaler(1))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Exp {
    arg: Box<Expression>,
}

impl DiffBase for Exp {
    fn diff(&self) -> Expression {
        Expression::Mul(vec![
            Expression::Base(Base::Exp(self.clone())),
            self.arg.diff(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pow {
    exp: isize,
    arg: Box<Expression>,
}

impl DiffBase for Pow {
    fn diff(&self) -> Expression {
        Expression::Mul(vec![
            Expression::Base(Base::Scaler(self.exp)),
            Expression::Base(Base::Pow(Pow {
                exp: self.exp - 1,
                arg: self.arg.clone(),
            })),
            self.arg.diff(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Log {
    arg: Box<Expression>,
}

impl DiffBase for Log {
    fn diff(&self) -> Expression {
        Expression::Mul(vec![
            Expression::Base(Base::Pow(Pow {
                exp: -1,
                arg: self.arg.clone(),
            })),
            self.arg.diff(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Sin {
    arg: Box<Expression>,
}

impl DiffBase for Sin {
    fn diff(&self) -> Expression {
        Expression::Mul(vec![
            Expression::Base(Base::Cos(Cos {
                arg: self.arg.clone(),
            })),
            self.arg.diff(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Cos {
    arg: Box<Expression>,
}

impl DiffBase for Cos {
    fn diff(&self) -> Expression {
        Expression::Mul(vec![
            Expression::Base(Base::Scaler(-1)),
            Expression::Base(Base::Sin(Sin {
                arg: self.arg.clone(),
            })),
            self.arg.diff(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Tan {
    arg: Box<Expression>,
}

impl DiffBase for Tan {
    fn diff(&self) -> Expression {
        Expression::Mul(vec![
            Expression::Base(Base::Pow(Pow {
                exp: -2,
                arg: Box::new(Expression::Base(Base::Cos(Cos {
                    arg: self.arg.clone(),
                }))),
            })),
            self.arg.diff(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::{Base, Cos, Exp, Expression, Log, Pow, Sin, Tan, X};

    #[test]
    fn test_diff() {
        let cases = [
            (
                Expression::Base(Base::Scaler(0)),
                Expression::Base(Base::Scaler(0)),
            ),
            (
                Expression::Base(Base::Scaler(1)),
                Expression::Base(Base::Scaler(0)),
            ),
            (
                Expression::Base(Base::X(X {})),
                Expression::Base(Base::Scaler(1)),
            ),
            (
                Expression::Base(Base::Exp(Exp {
                    arg: Box::new(Expression::Base(Base::X(X {}))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Exp(Exp {
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    })),
                    Expression::Base(Base::Scaler(1)),
                ]),
            ),
            (
                Expression::Base(Base::Pow(Pow {
                    exp: 1,
                    arg: Box::new(Expression::Base(Base::Scaler(1))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Scaler(1)),
                    Expression::Base(Base::Pow(Pow {
                        exp: 0,
                        arg: Box::new(Expression::Base(Base::Scaler(1))),
                    })),
                    Expression::Base(Base::Scaler(0)),
                ]),
            ),
            (
                Expression::Base(Base::Pow(Pow {
                    exp: 1,
                    arg: Box::new(Expression::Base(Base::X(X {}))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Scaler(1)),
                    Expression::Base(Base::Pow(Pow {
                        exp: 0,
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    })),
                    Expression::Base(Base::Scaler(1)),
                ]),
            ),
            (
                Expression::Base(Base::Log(Log {
                    arg: Box::new(Expression::Base(Base::X(X {}))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Pow(Pow {
                        exp: -1,
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    })),
                    Expression::Base(Base::Scaler(1)),
                ]),
            ),
            (
                Expression::Base(Base::Sin(Sin {
                    arg: Box::new(Expression::Base(Base::X(X {}))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Cos(Cos {
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    })),
                    Expression::Base(Base::Scaler(1)),
                ]),
            ),
            (
                Expression::Base(Base::Tan(Tan {
                    arg: Box::new(Expression::Base(Base::X(X {}))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Pow(Pow {
                        exp: -2,
                        arg: Box::new(Expression::Base(Base::Cos(Cos {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        }))),
                    })),
                    Expression::Base(Base::Scaler(1)),
                ]),
            ),
            (
                Expression::Base(Base::Exp(Exp {
                    arg: Box::new(Expression::Base(Base::Pow(Pow {
                        exp: 2,
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    }))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Exp(Exp {
                        arg: Box::new(Expression::Base(Base::Pow(Pow {
                            exp: 2,
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        }))),
                    })),
                    Expression::Mul(vec![
                        Expression::Base(Base::Scaler(2)),
                        Expression::Base(Base::Pow(Pow {
                            exp: 1,
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                        Expression::Base(Base::Scaler(1)),
                    ]),
                ]),
            ),
            (
                Expression::Base(Base::Exp(Exp {
                    arg: Box::new(Expression::Base(Base::Log(Log {
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    }))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Exp(Exp {
                        arg: Box::new(Expression::Base(Base::Log(Log {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        }))),
                    })),
                    Expression::Mul(vec![
                        Expression::Base(Base::Pow(Pow {
                            exp: -1,
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                        Expression::Base(Base::Scaler(1)),
                    ]),
                ]),
            ),
            (
                Expression::Base(Base::Sin(Sin {
                    arg: Box::new(Expression::Base(Base::Cos(Cos {
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    }))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Cos(Cos {
                        arg: Box::new(Expression::Base(Base::Cos(Cos {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        }))),
                    })),
                    Expression::Mul(vec![
                        Expression::Base(Base::Scaler(-1)),
                        Expression::Base(Base::Sin(Sin {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                        Expression::Base(Base::Scaler(1)),
                    ]),
                ]),
            ),
            (
                Expression::Base(Base::Tan(Tan {
                    arg: Box::new(Expression::Base(Base::Log(Log {
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    }))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Pow(Pow {
                        exp: -2,
                        arg: Box::new(Expression::Base(Base::Cos(Cos {
                            arg: Box::new(Expression::Base(Base::Log(Log {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            }))),
                        }))),
                    })),
                    Expression::Mul(vec![
                        Expression::Base(Base::Pow(Pow {
                            exp: -1,
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                        Expression::Base(Base::Scaler(1)),
                    ]),
                ]),
            ),
            (
                Expression::Add(vec![Expression::Base(Base::Scaler(1))]),
                Expression::Add(vec![Expression::Base(Base::Scaler(0))]),
            ),
            (
                Expression::Mul(vec![Expression::Base(Base::Scaler(1))]),
                Expression::Add(vec![Expression::Mul(vec![Expression::Base(Base::Scaler(
                    0,
                ))])]),
            ),
            (
                Expression::Add(vec![
                    Expression::Base(Base::X(X {})),
                    Expression::Base(Base::Scaler(1)),
                ]),
                Expression::Add(vec![
                    Expression::Base(Base::Scaler(1)),
                    Expression::Base(Base::Scaler(0)),
                ]),
            ),
            (
                Expression::Mul(vec![
                    Expression::Base(Base::X(X {})),
                    Expression::Base(Base::Exp(Exp {
                        arg: Box::new(Expression::Base(Base::X(X {}))),
                    })),
                ]),
                Expression::Add(vec![
                    Expression::Mul(vec![
                        Expression::Base(Base::Scaler(1)),
                        Expression::Base(Base::Exp(Exp {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                    ]),
                    Expression::Mul(vec![
                        Expression::Base(Base::X(X {})),
                        Expression::Mul(vec![
                            Expression::Base(Base::Exp(Exp {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                            Expression::Base(Base::Scaler(1)),
                        ]),
                    ]),
                ]),
            ),
            (
                Expression::Mul(vec![
                    Expression::Add(vec![
                        Expression::Base(Base::X(X {})),
                        Expression::Base(Base::Exp(Exp {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                    ]),
                    Expression::Add(vec![
                        Expression::Base(Base::X(X {})),
                        Expression::Base(Base::Exp(Exp {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                    ]),
                ]),
                Expression::Add(vec![
                    Expression::Mul(vec![
                        Expression::Add(vec![
                            Expression::Base(Base::Scaler(1)),
                            Expression::Mul(vec![
                                Expression::Base(Base::Exp(Exp {
                                    arg: Box::new(Expression::Base(Base::X(X {}))),
                                })),
                                Expression::Base(Base::Scaler(1)),
                            ]),
                        ]),
                        Expression::Add(vec![
                            Expression::Base(Base::X(X {})),
                            Expression::Base(Base::Exp(Exp {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                        ]),
                    ]),
                    Expression::Mul(vec![
                        Expression::Add(vec![
                            Expression::Base(Base::X(X {})),
                            Expression::Base(Base::Exp(Exp {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                        ]),
                        Expression::Add(vec![
                            Expression::Base(Base::Scaler(1)),
                            Expression::Mul(vec![
                                Expression::Base(Base::Exp(Exp {
                                    arg: Box::new(Expression::Base(Base::X(X {}))),
                                })),
                                Expression::Base(Base::Scaler(1)),
                            ]),
                        ]),
                    ]),
                ]),
            ),
            (
                Expression::Add(vec![
                    Expression::Mul(vec![
                        Expression::Base(Base::Sin(Sin {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                        Expression::Base(Base::Cos(Cos {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                        Expression::Base(Base::Tan(Tan {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                    ]),
                    Expression::Mul(vec![
                        Expression::Base(Base::Exp(Exp {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                        Expression::Base(Base::Log(Log {
                            arg: Box::new(Expression::Base(Base::X(X {}))),
                        })),
                    ]),
                ]),
                Expression::Add(vec![
                    Expression::Add(vec![
                        Expression::Mul(vec![
                            Expression::Mul(vec![
                                Expression::Base(Base::Cos(Cos {
                                    arg: Box::new(Expression::Base(Base::X(X {}))),
                                })),
                                Expression::Base(Base::Scaler(1)),
                            ]),
                            Expression::Base(Base::Cos(Cos {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                            Expression::Base(Base::Tan(Tan {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                        ]),
                        Expression::Mul(vec![
                            Expression::Base(Base::Sin(Sin {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                            Expression::Mul(vec![
                                Expression::Base(Base::Scaler(-1)),
                                Expression::Base(Base::Sin(Sin {
                                    arg: Box::new(Expression::Base(Base::X(X {}))),
                                })),
                                Expression::Base(Base::Scaler(1)),
                            ]),
                            Expression::Base(Base::Tan(Tan {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                        ]),
                        Expression::Mul(vec![
                            Expression::Base(Base::Sin(Sin {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                            Expression::Base(Base::Cos(Cos {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                            Expression::Mul(vec![
                                Expression::Base(Base::Pow(Pow {
                                    exp: -2,
                                    arg: Box::new(Expression::Base(Base::Cos(Cos {
                                        arg: Box::new(Expression::Base(Base::X(X {}))),
                                    }))),
                                })),
                                Expression::Base(Base::Scaler(1)),
                            ]),
                        ]),
                    ]),
                    Expression::Add(vec![
                        Expression::Mul(vec![
                            Expression::Mul(vec![
                                Expression::Base(Base::Exp(Exp {
                                    arg: Box::new(Expression::Base(Base::X(X {}))),
                                })),
                                Expression::Base(Base::Scaler(1)),
                            ]),
                            Expression::Base(Base::Log(Log {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                        ]),
                        Expression::Mul(vec![
                            Expression::Base(Base::Exp(Exp {
                                arg: Box::new(Expression::Base(Base::X(X {}))),
                            })),
                            Expression::Mul(vec![
                                Expression::Base(Base::Pow(Pow {
                                    exp: -1,
                                    arg: Box::new(Expression::Base(Base::X(X {}))),
                                })),
                                Expression::Base(Base::Scaler(1)),
                            ]),
                        ]),
                    ]),
                ]),
            ),
            (
                Expression::Base(Base::Exp(Exp {
                    arg: Box::new(Expression::Add(vec![
                        Expression::Base(Base::X(X {})),
                        Expression::Base(Base::Scaler(1)),
                    ])),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Exp(Exp {
                        arg: Box::new(Expression::Add(vec![
                            Expression::Base(Base::X(X {})),
                            Expression::Base(Base::Scaler(1)),
                        ])),
                    })),
                    Expression::Add(vec![
                        Expression::Base(Base::Scaler(1)),
                        Expression::Base(Base::Scaler(0)),
                    ]),
                ]),
            ),
            (
                Expression::Base(Base::Sin(Sin {
                    arg: Box::new(Expression::Mul(vec![
                        Expression::Base(Base::Scaler(2)),
                        Expression::Base(Base::X(X {})),
                    ])),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Cos(Cos {
                        arg: Box::new(Expression::Mul(vec![
                            Expression::Base(Base::Scaler(2)),
                            Expression::Base(Base::X(X {})),
                        ])),
                    })),
                    Expression::Add(vec![
                        Expression::Mul(vec![
                            Expression::Base(Base::Scaler(0)),
                            Expression::Base(Base::X(X {})),
                        ]),
                        Expression::Mul(vec![
                            Expression::Base(Base::Scaler(2)),
                            Expression::Base(Base::Scaler(1)),
                        ]),
                    ]),
                ]),
            ),
            (
                Expression::Base(Base::Log(Log {
                    arg: Box::new(Expression::Base(Base::Scaler(0))),
                })),
                Expression::Mul(vec![
                    Expression::Base(Base::Pow(Pow {
                        exp: -1,
                        arg: Box::new(Expression::Base(Base::Scaler(0))),
                    })),
                    Expression::Base(Base::Scaler(0)),
                ]),
            ),
        ];

        for case in cases {
            assert_eq!(case.0.diff(), case.1);
        }
    }
}

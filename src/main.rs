fn main() {}

type Scaler = isize;

#[derive(Debug, Clone, PartialEq)]
enum Base {
    SCALER(Scaler),
    X(X),
    EXP(Exp),
    POW(Pow),
    LOG(Log),
    SIN(Sin),
    COS(Cos),
    TAN(Tan),
}

impl Base {
    fn diff(&self) -> Monomial {
        match self {
            Self::SCALER(_) => vec![Self::SCALER(0)],
            Self::X(x) => x.diff(),
            Self::EXP(exp) => exp.diff(),
            Self::POW(pow) => pow.diff(),
            Self::LOG(log) => log.diff(),
            Self::SIN(sin) => sin.diff(),
            Self::COS(cos) => cos.diff(),
            Self::TAN(tan) => tan.diff(),
        }
    }
}

trait DiffBase {
    fn diff(&self) -> Monomial;
}

#[derive(Debug, Clone, PartialEq)]
struct X {}

impl DiffBase for X {
    fn diff(&self) -> Monomial {
        vec![Base::SCALER(1)]
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Exp {
    arg: Box<Base>,
}

impl DiffBase for Exp {
    fn diff(&self) -> Monomial {
        let mut mono = vec![Base::EXP(self.clone())];
        mono.extend(self.arg.diff());
        mono
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pow {
    exp: isize,
    arg: Box<Base>,
}

impl DiffBase for Pow {
    fn diff(&self) -> Monomial {
        let mut mono = if self.exp == 1 {
            vec![]
        } else {
            vec![
                Base::SCALER(self.exp),
                Base::POW(Pow {
                    exp: self.exp - 1,
                    arg: self.arg.clone(),
                }),
            ]
        };
        mono.extend(self.arg.diff());
        mono
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Log {
    arg: Box<Base>,
}

impl DiffBase for Log {
    fn diff(&self) -> Monomial {
        let mut mono = vec![Base::POW(Pow {
            exp: -1,
            arg: self.arg.clone(),
        })];
        mono.extend(self.arg.diff());
        mono
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Sin {
    arg: Box<Base>,
}

impl DiffBase for Sin {
    fn diff(&self) -> Monomial {
        let mut mono = vec![Base::COS(Cos {
            arg: self.arg.clone(),
        })];
        mono.extend(self.arg.diff());
        mono
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Cos {
    arg: Box<Base>,
}

impl DiffBase for Cos {
    fn diff(&self) -> Monomial {
        let mut mono = vec![
            Base::SCALER(-1),
            Base::SIN(Sin {
                arg: self.arg.clone(),
            }),
        ];
        mono.extend(self.arg.diff());
        mono
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Tan {
    arg: Box<Base>,
}

impl DiffBase for Tan {
    fn diff(&self) -> Monomial {
        let mut mono = vec![Base::POW(Pow {
            exp: -2,
            arg: Box::new(Base::COS(Cos {
                arg: self.arg.clone(),
            })),
        })];
        mono.extend(self.arg.diff());
        mono
    }
}

type Monomial = Vec<Base>;
// type Polynomial = Vec<Monomial>;

#[cfg(test)]
mod tests {
    use crate::{Base, Cos, Exp, Log, Pow, Sin, Tan, X};

    #[test]
    fn test_diff() {
        let cases = [
            (Base::SCALER(0), vec![Base::SCALER(0)]),
            (Base::SCALER(1), vec![Base::SCALER(0)]),
            (Base::X(X {}), vec![Base::SCALER(1)]),
            (
                Base::EXP(Exp {
                    arg: Box::new(Base::X(X {})),
                }),
                vec![
                    Base::EXP(Exp {
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::POW(Pow {
                    exp: 1,
                    arg: Box::new(Base::SCALER(1)),
                }),
                vec![Base::SCALER(0)],
            ),
            (
                Base::POW(Pow {
                    exp: 1,
                    arg: Box::new(Base::X(X {})),
                }),
                vec![Base::SCALER(1)],
            ),
            (
                Base::POW(Pow {
                    exp: 2,
                    arg: Box::new(Base::X(X {})),
                }),
                vec![
                    Base::SCALER(2),
                    Base::POW(Pow {
                        exp: 1,
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::LOG(Log {
                    arg: Box::new(Base::X(X {})),
                }),
                vec![
                    Base::POW(Pow {
                        exp: -1,
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::SIN(Sin {
                    arg: Box::new(Base::X(X {})),
                }),
                vec![
                    Base::COS(Cos {
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::TAN(Tan {
                    arg: Box::new(Base::X(X {})),
                }),
                vec![
                    Base::POW(Pow {
                        exp: -2,
                        arg: Box::new(Base::COS(Cos {
                            arg: Box::new(Base::X(X {})),
                        })),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::EXP(Exp {
                    arg: Box::new(Base::POW(Pow {
                        exp: 2,
                        arg: Box::new(Base::X(X {})),
                    })),
                }),
                vec![
                    Base::EXP(Exp {
                        arg: Box::new(Base::POW(Pow {
                            exp: 2,
                            arg: Box::new(Base::X(X {})),
                        })),
                    }),
                    Base::SCALER(2),
                    Base::POW(Pow {
                        exp: 1,
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::EXP(Exp {
                    arg: Box::new(Base::LOG(Log {
                        arg: Box::new(Base::X(X {})),
                    })),
                }),
                vec![
                    Base::EXP(Exp {
                        arg: Box::new(Base::LOG(Log {
                            arg: Box::new(Base::X(X {})),
                        })),
                    }),
                    Base::POW(Pow {
                        exp: -1,
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::SIN(Sin {
                    arg: Box::new(Base::COS(Cos {
                        arg: Box::new(Base::X(X {})),
                    })),
                }),
                vec![
                    Base::COS(Cos {
                        arg: Box::new(Base::COS(Cos {
                            arg: Box::new(Base::X(X {})),
                        })),
                    }),
                    Base::SCALER(-1),
                    Base::SIN(Sin {
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
            (
                Base::TAN(Tan {
                    arg: Box::new(Base::LOG(Log {
                        arg: Box::new(Base::X(X {})),
                    })),
                }),
                vec![
                    Base::POW(Pow {
                        exp: -2,
                        arg: Box::new(Base::COS(Cos {
                            arg: Box::new(Base::LOG(Log {
                                arg: Box::new(Base::X(X {})),
                            })),
                        })),
                    }),
                    Base::POW(Pow {
                        exp: -1,
                        arg: Box::new(Base::X(X {})),
                    }),
                    Base::SCALER(1),
                ],
            ),
        ];

        for case in cases {
            assert_eq!(case.0.diff(), case.1);
        }
    }
}

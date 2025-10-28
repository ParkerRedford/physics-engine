use Expr;

enum TrigFunction {
    sin,
    csc,
    cos,
    sec,
    tan,
    cot
}

enum InverseTrigFunction {
    arcsin,
    arccsc,
    arccos,
    arcsec,
    arctan,
    arccot
}

struct TrigInput {
    func: TrigFunction,
    degrees: f64
}

struct InverseTrigInput {
    func: InverseTrigFunction,
    ratio: f64
}

struct TrigResult {
    value: f64
}

impl TrigInput {
    fn accuracy(self, accuracy: i8) -> TrigResult {
        let value = match self.func {
            TrigFunction::sin => {},
            TrigFunction::csc => {},
            TrigFunction::cos => {},
            TrigFunction::sec => {},
            TrigFunction::tan => {},
            TrigFunction::cot => {}
        };

        TrigResult { value }
    }
}

impl InverseTrigInput {
    fn accuracy(self, accuracy: i8) -> TrigResult {
        let value = match self.func {
            InvsereTrigFunction::arcsin => {},
            InvsereTrigFunction::arccsc => {},
            InvsereTrigFunction::arccos => {},
            InvsereTrigFunction::arcsec => {},
            InvsereTrigFunction::arctan => {},
            InvsereTrigFunction::arccot => {}
        };

        TrigResult { value }
    }
}

enum Logarithmic {
    Normal,
    Natural
}

struct LogarithmicFunction {
    func: Logarithmic,
    base: Expr,
    exponent: Expr
}

struct LogarithmicResult {
    value: f64
}

impl Logarithmic {
    fn accuracy(self, accuracy: i8) -> LogarithmicResult {
        let mut value = match self.func {
            Logarithmic::Normal => {},
            Logarithmic::Natural => {}
        };

        LogarithmicResult { value }
    }
}

struct Exponent {
    input: f64
}

struct ExponentResult {
    output: f64
}

impl Exponent {
    fn accuracy(self, accuracy: i8) -> ExponentResult {
        let value = 0;

        ExponentResult { value }
    }
}


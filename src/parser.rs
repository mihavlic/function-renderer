use std::collections::VecDeque;

pub enum BinaryOperation {
    Sub,
    Add,
    Div,
    Mul,
    Exp,
    Greater,
    Lower,
    GreaterEq,
    LowerEq,
    Eq,
}

impl BinaryOperation {
    pub fn try_from_token(token: &Token) -> Option<(Self, u8)> {
        let (precedence, token) = match token {
            Token::Sub => (4, BinaryOperation::Sub),
            Token::Add => (4, BinaryOperation::Add),
            Token::Div => (3, BinaryOperation::Div),
            Token::Mul => (2, BinaryOperation::Mul),
            Token::Exp => (1, BinaryOperation::Exp),
            Token::Greater => (5, BinaryOperation::Greater),
            Token::Lower => (5, BinaryOperation::Lower),
            Token::GreaterEq => (5, BinaryOperation::GreaterEq),
            Token::LowerEq => (5, BinaryOperation::LowerEq),
            Token::Eq => (5, BinaryOperation::Eq),
            _ => return None,
        };

        Some((token, precedence))
    }
    pub fn name(&self) -> &'static str {
        match self {
            BinaryOperation::Sub => "sub",
            BinaryOperation::Add => "add",
            BinaryOperation::Div => "div",
            BinaryOperation::Mul => "mul",
            BinaryOperation::Exp => "exp",
            BinaryOperation::Greater => "greater",
            BinaryOperation::Lower => "lower",
            BinaryOperation::GreaterEq => "greatereq",
            BinaryOperation::LowerEq => "lowereq",
            BinaryOperation::Eq => "eq",
        }
    }
}

pub enum UnaryOperation {
    Neg,
    Log,
    Log2,
    Ln,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Abs,
    CoTan,
    ArcSin,
    ArcCos,
    ArcTan,
    ArcCotan,
    Length,
}

impl UnaryOperation {
    pub fn try_function_str(str: &str) -> Option<Self> {
        match str {
            "arccos" => Some(Self::ArcCos),
            "arccotan" => Some(Self::ArcCotan),
            "arcsin" => Some(Self::ArcSin),
            "arctan" => Some(Self::ArcTan),
            "cos" => Some(Self::Cos),
            "cotan" => Some(Self::CoTan),
            "ln" => Some(Self::Ln),
            "log" => Some(Self::Log),
            "log2" => Some(Self::Log2),
            "sin" => Some(Self::Sin),
            "sqrt" => Some(Self::Sqrt),
            "tan" => Some(Self::Tan),
            "abs" => Some(Self::Abs),
            "length" => Some(Self::Length),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            UnaryOperation::Neg => "neg",
            UnaryOperation::Log => "log",
            UnaryOperation::Log2 => "log2",
            UnaryOperation::Ln => "ln",
            UnaryOperation::Sqrt => "sqrt",
            UnaryOperation::Sin => "sin",
            UnaryOperation::Cos => "cos",
            UnaryOperation::Tan => "tan",
            UnaryOperation::Abs => "abs",
            UnaryOperation::CoTan => "cotan",
            UnaryOperation::ArcSin => "arcsin",
            UnaryOperation::ArcCos => "arccos",
            UnaryOperation::ArcTan => "arctan",
            UnaryOperation::ArcCotan => "arccotan",
            UnaryOperation::Length => "length",
        }
    }
}

#[derive(Debug)]
pub enum BuiltingVariable {
    X,
    Y,
    Z,
    T,
}

impl BuiltingVariable {
    pub fn try_from_str(str: &str) -> Option<Self> {
        match str {
            "X" | "x" => Some(Self::X),
            "Y" | "y" => Some(Self::Y),
            "Z" | "z" => Some(Self::Z),
            "t" => Some(Self::T),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Constant {
    E,
    Pi,
}

impl Constant {
    pub fn try_from_str(str: &str) -> Option<Self> {
        match str {
            "e" => Some(Self::E),
            "pi" => Some(Self::Pi),
            _ => None,
        }
    }
}

pub enum Expression {
    Binary {
        op: BinaryOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOperation,
        child: Box<Expression>,
    },
    Builtin(BuiltingVariable),
    Constant(Constant),
    Variable(String),
    Number(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Sub,
    Add,
    Mul,
    Div,
    Abs,
    Exp,
    Num,
    Greater,
    Lower,
    GreaterEq,
    LowerEq,
    Eq,
    LParen,
    RParen,
    Comma,
    Ident(&'a str),
    Number(f32),
    End,
}

// "inspired" by https://github.com/gfx-rs/naga/blob/48e79388b506535d668df4f6c7be4e681812ab81/src/front/wgsl/lexer.rs#L8
pub fn next_token(mut str: &str) -> (Token<'_>, &str) {
    let mut chars = str.chars();
    loop {
        str = chars.as_str();
        let cur = match chars.next() {
            Some(c) => c,
            None => return (Token::End, ""),
        };

        macro_rules! two_char_token {
            ($($char:pat => $token:expr),+ ; $single:expr) => {
                match chars.clone().next() {
                    $(
                        Some($char) => {
                            chars.next();
                            $token
                        },
                    )+
                        _  => $single
                }
            };
        }

        let tok = match cur {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '*' => Token::Mul,
            '/' => Token::Div,
            '-' => Token::Sub,
            '+' => Token::Add,
            '=' => Token::Eq,
            '^' => Token::Exp,
            '|' => Token::Abs,
            '>' => two_char_token!(
                '=' => Token::GreaterEq
                ; Token::Greater
            ),
            '<' => two_char_token!(
                '=' => Token::LowerEq
                ; Token::Lower
            ),
            '0'..='9' => {
                let (number, next) = consume_number(str);
                return (Token::Number(number), next);
            }
            _ if is_blankspace(cur) => continue,
            _ if is_identifier(cur) => {
                let (ident, rest) = consume_any(str, is_identifier);
                return (Token::Ident(ident), rest);
            }
            _ => panic!("Unexpected character '{cur}'"),
        };

        return (tok, chars.as_str());
    }
}

pub fn consume_any(input: &str, what: impl Fn(char) -> bool) -> (&str, &str) {
    let pos = input.find(|c| !what(c)).unwrap_or(input.len());
    input.split_at(pos)
}

pub fn consume_number(str: &str) -> (f32, &str) {
    #[derive(PartialEq, Eq)]
    pub enum State {
        Start,
        Integer,
        Fractional,
    }

    let mut state = State::Start;
    let mut chars = str.char_indices();

    let mut dot_index = None;
    let mut end_index = None;

    while let Some((i, c)) = chars.next() {
        match c {
            '0'..='9' => {}
            '.' => {
                if state == State::Integer {
                    dot_index = Some(i);
                    state = State::Fractional;
                } else {
                    panic!("Unexpected . when parsing number")
                }
            }
            _ => {
                end_index = Some(i);
                break;
            }
        }
        if state == State::Start {
            state = State::Integer;
        }
    }

    let end = end_index.unwrap_or(str.len());
    let number = &str[..end];
    let value = if dot_index.is_some() {
        number.parse::<f32>().unwrap()
    } else {
        number.parse::<u64>().unwrap() as f32
    };

    (value, &str[end..])
}

/// Returns whether or not a char is a blankspace (Unicode Pattern_White_Space)
pub fn is_blankspace(c: char) -> bool {
    c.is_whitespace()
}

/// Returns whether or not a char is a word part (Unicode XID_Continue)
pub fn is_identifier(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

pub fn operator_precedence(token: Token) -> Option<u8> {
    Some(match token {
        Token::Sub => 4,
        Token::Add => 4,
        Token::Div => 3,
        Token::Mul => 2,
        Token::Exp => 1,
        Token::Greater | Token::Lower | Token::GreaterEq | Token::LowerEq | Token::Eq => 5,
        _ => return None,
    })
}

pub struct Parser<'a> {
    tokens: VecDeque<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(mut str: &'a str) -> Self {
        let mut tokens = VecDeque::new();
        loop {
            let (tok, rest) = next_token(str);
            str = rest;
            if let Token::End = tok {
                break;
            }
            tokens.push_back(tok);
        }
        Self { tokens }
    }
    pub fn peek(&self) -> Token {
        self.tokens.front().cloned().unwrap_or(Token::End)
    }
    pub fn next(&mut self) -> Token {
        self.tokens.pop_front().unwrap_or(Token::End)
    }
    pub fn ensure_token(&mut self, tok: &Token) {
        let d = std::mem::discriminant::<Token>(tok);
        let next = self.next();
        let dn = std::mem::discriminant::<Token>(&next);

        if d != dn {
            panic!("Expected token {:?}, found {:?}", tok, next);
        }
    }
}

pub fn parse_monoop(parser: &mut Parser) -> Box<Expression> {
    let expr = match parser.next() {
        Token::Sub => {
            let child = parse_monoop(parser);
            Expression::Unary {
                op: UnaryOperation::Neg,
                child,
            }
        }
        Token::Number(num) => {
            let number = Box::new(Expression::Number(num));
            match parser.peek() {
                Token::Num | Token::LParen | Token::Ident(_) => {
                    let right = parse_expr(parser, 1);
                    Expression::Binary {
                        op: BinaryOperation::Mul,
                        left: number,
                        right,
                    }
                }
                _ => {
                    return number;
                }
            }
        }
        Token::LParen => {
            let node = parse_expr(parser, 8);
            parser.ensure_token(&Token::RParen);
            return node;
        }
        Token::Abs => {
            let child = parse_expr(parser, 8);
            parser.ensure_token(&Token::Abs);
            Expression::Unary {
                op: UnaryOperation::Abs,
                child,
            }
        }
        Token::Ident(ident) => {
            if let Some(op) = UnaryOperation::try_function_str(ident) {
                let child = parse_expr(parser, 2);
                Expression::Unary { op, child }
            } else if let Some(builtin) = BuiltingVariable::try_from_str(ident) {
                Expression::Builtin(builtin)
            } else if let Some(constant) = Constant::try_from_str(ident) {
                Expression::Constant(constant)
            } else {
                panic!("Unknown identifier '{ident}' when parsing monoop")
            }
        }
        other => {
            panic!("Unexpected token '{:?}' in monoop", other)
        }
    };
    Box::new(expr)
}

pub fn parse_expr(parser: &mut Parser, precedence: u8) -> Box<Expression> {
    let mut left = parse_monoop(parser);

    loop {
        let op = parser.peek();

        if let Token::End = op {
            break;
        }

        let bop = BinaryOperation::try_from_token(&op);

        if let Some((op, op_precedence)) = bop {
            if op_precedence <= precedence {
                parser.next();

                left = Box::new(Expression::Binary {
                    op,
                    left,
                    right: parse_expr(parser, op_precedence - 1),
                });

                continue;
            }
        }

        break;
    }

    return left;
}

pub fn debug_ast(node: &Expression) {
    match node {
        Expression::Number(number) => {
            eprint!("{number}");
        }
        Expression::Unary { op, child } => {
            eprint!("({} ", op.name());
            debug_ast(child);
            eprint!(")");
        }
        Expression::Binary { op, left, right } => {
            eprint!("({} ", op.name());
            debug_ast(left);
            eprint!(" ");
            debug_ast(right);
            eprint!(")");
        }
        Expression::Builtin(builtin) => {
            eprint!("{:?}", builtin);
        }
        Expression::Constant(constant) => {
            eprint!("{:?}", constant);
        }
        Expression::Variable(var) => {
            eprint!("${var}");
        }
    }
}

const LANE_WIDTH: usize = 8;

#[repr(align(16))]
#[derive(Default)]
pub struct Lane([f32; LANE_WIDTH]);

impl Lane {
    pub fn splat(value: f32) -> Self {
        Self([value; LANE_WIDTH])
    }
    pub fn zeroed() -> Self {
        unsafe { Self(std::mem::zeroed()) }
    }
    pub fn raw(&self) -> &[f32; LANE_WIDTH] {
        &self.0
    }
}

macro_rules! lane_map {
    ($in1:ident, $in2:ident, $out:ident, |$a:ident, $b:ident| $code:expr) => {
        unsafe {
            for i in 0..LANE_WIDTH {
                let $a = *$in1.0.get_unchecked(i);
                let $b = *$in2.0.get_unchecked(i);
                *$out.0.get_unchecked_mut(i) = $code;
            }
        }
    };
}

macro_rules! lane_map_single {
    ($in1:ident, $out:ident, |$a:ident| $code:expr) => {
        for i in 0..LANE_WIDTH {
            unsafe {
                let $a = *$in1.0.get_unchecked(i);
                *$out.0.get_unchecked_mut(i) = $code;
            }
        }
    };
}

macro_rules! lane_map_none {
    ($out:ident, || $code:expr) => {
        unsafe {
            for i in 0..LANE_WIDTH {
                *$out.0.get_unchecked_mut(i) = $code;
            }
        }
    };
}

macro_rules! unary_op_cases {
    ($in1:ident, $out:ident, |$a:ident| $op:ident { $($variant:ident => $code:expr,)+ }) => {
        match $op {
            $(
                UnaryOperation::$variant => lane_map_single!($in1, $out, |$a| $code),
            )+
        }
    };
}

macro_rules! binary_op_cases {
    ($in1:ident, $in2:ident, $out:ident, |$a:ident, $b:ident| $op:ident { $($variant:ident => $code:expr,)+ }) => {
        match $op {
            $(
                BinaryOperation::$variant => lane_map!($in1, $in2, $out, |$a, $b| $code),
            )+
        }
    };
}

macro_rules! none_cases {
    ($out:ident, || $op:ident { $($variant:ident => $code:expr,)+ }) => {
        match $op {
            $(
                BinaryOperation::$variant => lane_map_none!($out, || $code),
            )+
        }
    };
}

pub fn array_eval(node: &Expression, x: &Lane, y: &Lane, z: &Lane, t: f32, out: &mut Lane) {
    let mut tmp = Lane::zeroed();

    match node {
        Expression::Number(value) => lane_map_none!(out, || *value),
        Expression::Unary { op, child } => {
            array_eval(child, x, y, z, t, out);
            unary_op_cases!(
                out, out, |a| op {
                    Neg => -a,
                    Log => a.log10(),
                    Log2 => a.log2(),
                    Ln => a.ln(),
                    Sqrt => a.sqrt(),
                    Sin => a.sin(),
                    Cos => a.cos(),
                    Tan => a.tan(),
                    Abs => a.abs(),
                    CoTan => a.tan().recip(),
                    ArcSin => a.asin(),
                    ArcCos => a.acos(),
                    ArcTan => a.atan(),
                    ArcCotan => std::f32::consts::FRAC_2_PI - a.atan(),
                    Length => panic!(),
                }
            );
        }
        Expression::Binary { op, left, right } => {
            array_eval(left, x, y, z, t, &mut tmp);
            array_eval(right, x, y, z, t, out);
            binary_op_cases!(tmp, out, out, |a, b| op {
                Sub => a - b,
                Add => a + b,
                Div => a / b,
                Mul => a * b,
                Exp => a.powf(b),
                Greater => (a > b) as u32 as f32,
                Lower => (a < b) as u32 as f32,
                GreaterEq => (a >= b) as u32 as f32,
                LowerEq => (a <= b) as u32 as f32,
                Eq => (a == b) as u32 as f32,
            });
        }
        Expression::Builtin(builtin) => match builtin {
            BuiltingVariable::X => lane_map_single!(x, out, |a| a),
            BuiltingVariable::Y => lane_map_single!(y, out, |a| a),
            BuiltingVariable::Z => lane_map_single!(z, out, |a| a),
            BuiltingVariable::T => lane_map_none!(out, || t),
        },
        Expression::Constant(constant) => match constant {
            Constant::E => lane_map_none!(out, || std::f32::consts::E),
            Constant::Pi => lane_map_none!(out, || std::f32::consts::PI),
        },
        Expression::Variable(value) => {
            todo!()
        }
    }
}

// int main(int argc, char *argv[]) {
//     if (argc < 2 || strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0) {
//         puts("No equation provided");
//         puts("Usage: [binary] [equation] ?[o/no] - force or disable outline algorithm");
//         return 0;
//     }

//     Node* root;
//     ArrayList tokens;
//     {
//         bool debug = getenv("PARSE_DEBUG") != NULL;
//         const char* what = argv[1];

//         if (debug) {
//             printf("%s\n", what);
//         }

//         list_init(&tokens, 32*sizeof(Token));

//         char buf[32];
//         LexerState state = {
//             .state = LStart,
//             .ident_buf = buf,
//             .ident_buf_cursor = buf,
//             .ident_buf_end = buf + 32,
//         };

//         // the end pointer is never dereferenced and the lexer also checks for null chars so in this case we can have it be 0
//         lex(&what, 0, &tokens, &state, true);

//         if (tokens.cursor == tokens.allocation) {
//             eat_shit_and_die("No tokens lexed");
//         }

//         if (debug) {
//             for (Token* tok = (Token*)tokens.allocation; tok != (Token*)tokens.cursor; tok++) {
//                 switch (tok->kind) {
//                 case Num:
//                     printf("%f ", tok->data.number);
//                     break;
//                 case Ident:
//                     printf("$%s ", tok->data.identifier);
//                     break;
//                 default:
//                     printf("%s ", token_name(tok->kind));
//                     break;
//                 }
//             }
//             printf("\n");
//         }

//         Token* start = (Token*)tokens.allocation;
//         root = parse_expr(&start, (Token*)tokens.cursor, 8);

//         if (debug) {
//             printf("\n");
//             traverse_ast(root);
//             printf("\n");
//         }
//     }

//     bool saw_x, saw_y = false;
//     for (Token* tok = (Token*)tokens.allocation; tok != (Token*)tokens.cursor; tok++) {
//         if (tok->kind == X) {
//             saw_x = true;
//         } else if (tok->kind == Y) {
//             saw_y = true;
//         }
//     }

//     // the expression only has one x as a variable
//     // this is likely user error, replace with an equation with y, this is what desmos does
//     // TODO reconsider this, or do this with y as well
//     if (saw_x && !saw_y) {
//         Node* y_node = NEW(Node);
//         y_node->kind = BuiltinConstant;
//         y_node->data.constant = Y;

//         Node* new = NEW(Node);
//         new->kind = BinaryOp;
//         new->data.b_op.op = Eq;
//         new->data.b_op.n1 = y_node;
//         new->data.b_op.n2 = root;

//         root = new;
//     }

//     // -1 disabled, 0 unconfigured, 1 enabled
//     int outline = 0;
//     if (argc >= 3) {
//         if (strcmp(argv[2], "o") == 0) {
//             outline = 1;
//         } else if (strcmp(argv[2], "no") == 0) {
//             outline = -1;
//         }
//     }

//     // heuristic, we probably want this for equation as it gives much better quality
//     // transform equation into a subtraction so that we don't lose the gradient of the function
//     if (outline != -1 && root->kind == BinaryOp && root->data.b_op.op == Eq) {
//         outline = 1;
//         root->data.b_op.op = Sub;
//     }

//     initscr();
//     noecho(); // dont print typed chars
//     cbreak(); // dont handle clear shortcuts and buffering
//     keypad(stdscr, TRUE); // handle arrow keys properly
//     nodelay(stdscr, TRUE); // don't block on wgetch
//     curs_set(0); // hide cursor
//     use_tioctl(true); // get the correct terminal size

//     // how many value units a single character is
//     float zoom = 1.0;
//     float x_shift = 0.0;
//     float y_shift = 0.0;

//     int w = 0;
//     int h = 0;

//     // for the outlines we need to evaluate a row outside the screen
//     // so if outline is enabled these are the dimensions of the buffer but not the screen
//     int w_ = w;
//     int h_ = h;
//     float* buffer = 0;
//     chtype* ch_buffer = 0;

//     bool dirty = false;

//     while (1) {
//         int ch = wgetch(stdscr);
//         switch (ch) {
//         case KEY_UP:
//             y_shift += 1.0*zoom;
//             dirty = true;
//             break;
//         case KEY_DOWN:
//             y_shift -= 1.0*zoom;
//             dirty = true;
//             break;
//         case KEY_RIGHT:
//             x_shift += 1.0*zoom;
//             dirty = true;
//             break;
//         case KEY_LEFT:
//             x_shift -= 1.0*zoom;
//             dirty = true;
//             break;
//         case 'w':
//             zoom *= 0.9;
//             dirty = true;
//             break;
//         case 'z':
//             zoom /= 0.9;
//             dirty = true;
//             break;
//         case 'r':
//             zoom = 1.0;
//             x_shift = 0.0;
//             y_shift = 0.0;
//             dirty = true;
//             break;
//         case 'q':
//             goto end;
//         default:
//             break;
//         }

//         // cap the zoom so that it doesn't become negative
//         zoom = fmax(zoom, 0.0001);

//         if (w != COLS || h != LINES) {
//             dirty = true;
//             w = COLS;
//             h = LINES;

//             if (outline) {
//                 w_ = w + 1;
//                 h_ = h + 1;
//             } else {
//                 w_ = w;
//                 h_ = h;
//             }

//             // round up to a multiple of LANE_WIDTH
//             int size = ((w_*h_ + LANE_WIDTH - 1) / LANE_WIDTH) * LANE_WIDTH;

//             if (buffer) {
//                 free(buffer);
//                 free(ch_buffer);
//             }

//             buffer = aligned_alloc(LANE_ALIGN, size*sizeof(float));
//             ch_buffer = malloc(size*sizeof(chtype));
//         }

//         if (dirty && w != 0 && h != 0) {
//             // not needed since we overwrite everything anyway
//             // clear();

//             _Alignas(LANE_ALIGN) float x_arr[LANE_WIDTH];
//             _Alignas(LANE_ALIGN) float y_arr[LANE_WIDTH];

//             // since terminal cells aren't square, we squish the y direction to roughly compensate
//             const float y_mul = 2.0;

//             float* out = buffer;
//             float xf0 = x_shift - w*zoom*0.5;
//             float xf;
//             float yf = y_shift + h*zoom*0.5*y_mul;
//             float d = zoom;

//             int i = 0;
//             for (int y = 0; y < h_; y++) {
//                 xf = xf0;
//                 for (int x = 0; x < w_; x++) {
//                     x_arr[i] = xf;
//                     y_arr[i] = yf;
//                     xf += d;
//                     i++;

//                     if (i == LANE_WIDTH) {
//                         array_eval(root, x_arr, y_arr, out);
//                         out += LANE_WIDTH;
//                         i = 0;
//                     }
//                 }
//                 yf -= d*y_mul;
//             }
//             // finish the rest, out was padded enough so that we don't overrun it
//             if (i != 0) {
//                 array_eval(root, x_arr, y_arr, out);
//             }

// #define CELL_CHECK(x, y) \
// if (((x >= 0) && (x < w_) && (y >= 0) && (y < h_))) { \
//     buffer[(x)+(y)*w_] > 0 ? positive++ : negative++; \
// }

//             if (outline == 1) {
//                 // stolen from https://www.youtube.com/watch?v=EvvWOaLgKVU
//                 for (int y = 0; y < h_; y++) {
//                     for (int x = 0; x < w_; x++) {
//                         int positive = 0;
//                         int negative = 0;
//                         CELL_CHECK(x,y)
//                         CELL_CHECK(x+1,y)
//                         CELL_CHECK(x,y+1)
//                         CELL_CHECK(x+1,y+1)

//                         // ch_buffer[x+y*w_] = (positive > 0 && positive < 3 && negative > 0) ? 'X' : ' ';
//                         ch_buffer[x+y*w_] = (positive > 0 && negative > 0) ? 'X' : ' ';
//                     }
//                 }
//             } else {
//                 for (int j = 0; j < w_*h; j++) {
//                     ch_buffer[j] = buffer[j] > 0.0 ? 'X' : ' ';
//                 }
//             }

//             chtype* ch_out = ch_buffer;
//             for (int k = 0; k < h; k++) {
//                 mvaddchnstr(k, 0, ch_out, w);
//                 ch_out += w_;
//             }

//             dirty = false;
//             refresh();
//         }

//         // sleep ~16 ms to get 60 fps, this assumes frame drawing is instant
//         usleep(16666);
//     }

//     end:
//     endwin();
//     list_free(&tokens);
//     free_ast(root);
//     return 0;
// }

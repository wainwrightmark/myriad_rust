use serde::{Serialize, Deserialize};


#[derive(PartialEq, Debug, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
pub enum Letter{
    Number{value: u32},
    Operator{operation: Operation},
    Blank

}


impl Letter {
    pub fn try_create(rune: char) -> Option<Letter> {
        let d = rune.to_digit(10).map(|x|Letter::Number{value: x});
        if d != None {return d;}
        
        match rune{
            '-' => Some(Letter::Operator{operation: Operation::Minus}) ,
            '+' => Some(Letter::Operator{operation: Operation::Plus}),
            '*' => Some(Letter::Operator{operation: Operation::Times}),
            '/' => Some(Letter::Operator{operation: Operation::Divide}),
            '_' => Some(Letter::Blank),
            _ => None
        }
    }

    pub fn legal_letters() -> impl Iterator<Item =Letter>{
        let nums = (1..9).map(|x|Letter::Number{value: x});
        let ops = [Letter::Operator{operation: Operation::Plus}, Letter::Operator{operation: Operation::Minus}, Letter::Operator{operation: Operation::Times}, Letter::Operator{operation: Operation::Divide}];
        //let others = [Letter::Blank];

        //.chain(others.into_iter());
        nums.chain(ops.into_iter())
    }

    pub fn word_text(&self)-> String{
        match self{
            Letter::Number {value} => value.to_string(),
            Letter::Operator {operation} => operation.to_string(),
            Letter::Blank => "_".to_string()
        }        
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = &self.word_text();
        write!(f, "{}", r)
    }
}

#[derive(PartialEq, Debug, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
pub enum Operation{
    Plus,
    Times,
    Minus,
    Divide
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self{
            Operation::Plus=> "+",
            Operation::Times=> "*",
            Operation::Minus=> "-",
            Operation::Divide=> "/"
        };
        write!(f, "{}", r)
    }
}
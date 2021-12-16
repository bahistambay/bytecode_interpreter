use serde::{Serialize, Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ByteCode {
    pub op: Op,
    pub value: Option<String>,
}

impl std::str::FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LOAD_VAL" => Ok(Op::LoadVal),
            "WRITE_VAR" => Ok(Op::WriteVar),
            "READ_VAR" => Ok(Op::ReadVar),
            "ADD" => Ok(Op::Add),
            "SUBTRACT" => Ok(Op::Subtract),
            "MULTIPLY" => Ok(Op::Multiply),
            "DIVIDE" => Ok(Op::Divide),
            "RETURN_VALUE" => Ok(Op::ReturnValue),
            "GOTO" => Ok(Op::Goto),
            "IF_CMP_EQ" => Ok(Op::IfCmpEq),
            "IF_CMP_GE" => Ok(Op::IfCmpGe),
            "IF_CMP_LE" => Ok(Op::IfCmpLe),
            "IF_CMP_GT" => Ok(Op::IfCmpGt),
            "IF_CMP_LT" => Ok(Op::IfCmpLt),
            "IF_CMP_NE" => Ok(Op::IfCmpNe),
            _ => Err(format!("'{}' is not a valid value for Op", s)),
        }
    }
}

impl<'de> Deserialize<'de> for ByteCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let mut parts = s.splitn(2, " ").fuse();

        Ok(ByteCode {
            op: <Op>::from_str(&parts.next().unwrap()).unwrap(),
            value: parts.next().map(str::to_string),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    LoadVal,
    WriteVar,
    ReadVar,
    ReturnValue,
    Goto,
    IfCmpEq,
    IfCmpGe,
    IfCmpGt,
    IfCmpLe,
    IfCmpLt,
    IfCmpNe,
}

#[derive(Clone, Debug, Serialize)]
pub struct Value {
    pub variable: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum ProgramError {
    StackParseError,
}

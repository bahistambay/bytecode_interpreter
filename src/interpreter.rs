use crate::op::{ByteCode, Op, ProgramError, Value};

macro_rules! do_op {
    ($stack:expr, $op:tt) => {
	// pop two last variables in the stack
        if let Some(a) = $stack.pop() {
            if let Some(b) = $stack.pop() {
		// push the result of the operation to stack
                $stack.push(Value {
                    variable: None,
                    value: Some((b.value.unwrap().parse::<i32>().unwrap() $op a.value.unwrap().parse::<i32>().unwrap()).to_string()),
                });
                Ok(Value { variable: None, value: None})
            } else {
                Err(ProgramError::StackParseError)
            }
        } else {
            Err(ProgramError::StackParseError)
        }
    };
}

macro_rules! do_jmp {
    ($stack:expr, $bytecode:expr, $i:ident, $op:tt) => {
	// pop two last variables in the stack
        if let Some(a) = $stack.pop() {
            if let Some(b) = $stack.pop() {
		// push the result of the operation to stack
                if b.value.unwrap().parse::<i32>().unwrap() $op a.value.unwrap().parse::<i32>().unwrap() {
                    $i = $bytecode.value.as_ref().unwrap().parse::<usize>().unwrap();
                }
                Ok(Value {
                    variable: None,
                    value: None,
                })
            } else {
                Err(ProgramError::StackParseError)
            }
        } else {
            Err(ProgramError::StackParseError)
        }
    };
}

pub fn interpret(stack: &mut Vec<Value>, bytecode_list: &mut Vec<ByteCode>) -> Result<Value, ProgramError> {
    let mut return_value = Ok(Value {
        variable: None,
        value: None,
    });
    let mut i = 0;
    while i < bytecode_list.len() {
        let bytecode = &bytecode_list[i];
        i += 1;

        return_value = match bytecode.op {
            Op::LoadVal => {
                stack.push(Value {
                    variable: None,
                    value: bytecode.value.clone(),
                });
                Ok(Value {
                    variable: None,
                    value: None,
                })
            }
            Op::WriteVar => {
                let loaded_value = stack.pop();
                let pos_value = stack.iter().position(|x| {
                    x.variable == Some(bytecode.value.as_deref().unwrap().to_string())
                });

                if let Some(v) = loaded_value {
                    if let Some(pv) = pos_value {
                        stack[pv] = Value {
                            variable: Some(bytecode.value.clone().unwrap()),
                            value: v.value,
                        };
                    } else {
                        stack.push(Value {
                            variable: Some(bytecode.value.clone().unwrap()),
                            value: v.value,
                        });
                    }
                    Ok(Value {
                        variable: None,
                        value: None,
                    })
                } else {
                    Err(ProgramError::StackParseError)
                }
            }
            Op::ReadVar => {
                let read_value = stack
                    .iter()
                    .find(|&x| x.variable == Some(bytecode.value.as_deref().unwrap().to_string()));

                if let Some(v) = read_value {
                    let var = v.clone();
                    stack.push(Value {
                        variable: var.variable,
                        value: var.value,
                    });
                    Ok(Value {
                        variable: None,
                        value: None,
                    })
                } else {
                    Err(ProgramError::StackParseError)
                }
            }
            Op::ReturnValue => {
                let retval = stack.pop();
                Ok(retval.unwrap())
            }
            Op::Multiply => do_op!(stack, *),
            Op::Divide => do_op!(stack, /),
            Op::Add => do_op!(stack, +),
            Op::Subtract => do_op!(stack, -),
            Op::Goto => {
                i = bytecode.value.as_ref().unwrap().parse::<usize>().unwrap();
                Ok(Value {
                    variable: None,
                    value: None,
                })
            }
            Op::IfCmpEq => do_jmp!(stack, bytecode, i, ==),
            Op::IfCmpGe => do_jmp!(stack, bytecode, i, >=),
            Op::IfCmpGt => do_jmp!(stack, bytecode, i, >),
            Op::IfCmpNe => do_jmp!(stack, bytecode, i, !=),
            Op::IfCmpLe => do_jmp!(stack, bytecode, i, <=),
            Op::IfCmpLt => do_jmp!(stack, bytecode, i, <),
        };
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] //load the value 1 and return an empty Value
    fn test_interpret_load_val() {
        let bytecode = ByteCode {
            op: Op::LoadVal,
            value: Some("1".to_string()),
        };
        let mut bytecode_list = Vec::new();
        bytecode_list.push(bytecode);
        let mut stack = Vec::new();
        assert_eq!(
            serde_json::to_string(&interpret(&mut stack, &mut bytecode_list).unwrap()).unwrap(),
            serde_json::to_string(&Value {
                variable: None,
                value: None
            })
            .unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "\"'DOES_NOT_EXIST' is not a valid value for Op\"")]
    fn test_interpret_bytecode_error() {
        let s: &str = "DOES_NOT_EXIST";
        let form_s = format!("\"{}\"", s);
        let mut bytecode_list = Vec::new();
        //deserialize form_s to ByteCode type
        bytecode_list.push(serde_json::from_str::<ByteCode>(&form_s).unwrap());
        let mut stack = Vec::new();
        assert!(interpret(&mut stack, &mut bytecode_list).is_err());
    }

    #[test]
    fn test_interpret() {
                        /*
                         * LOAD_VAL 1
         x = 1           * WRITE_VAR ‘x’
         y = 2           * LOAD_VAL 2
         return (x+1)*y  * WRITE_VAR ‘y’
                         * READ_VAR ‘x’
                         * LOAD_VAL 1
                         * ADD
                         * READ_VAR ‘y’
                         * MULTIPLY
                         * RETURN_VALUE
                         * */
        let mut stack = Vec::new();
        let mut bytecode_list = Vec::new();
        bytecode_list.push(ByteCode { op: Op::LoadVal, value: Some("1".to_string()) });
        bytecode_list.push(ByteCode { op: Op::WriteVar, value: Some("‘x’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::LoadVal, value: Some("2".to_string()) });
        bytecode_list.push(ByteCode { op: Op::WriteVar, value: Some("‘y’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::ReadVar, value: Some("‘x’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::LoadVal, value: Some("1".to_string()) });
        bytecode_list.push(ByteCode { op: Op::Add, value: None });
        bytecode_list.push(ByteCode { op: Op::ReadVar, value: Some("‘y’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::Multiply, value: None });
        bytecode_list.push(ByteCode { op: Op::ReturnValue, value: None });

        assert_eq!(
            serde_json::to_string(&interpret(&mut stack, &mut bytecode_list).unwrap()).unwrap(),
            serde_json::to_string(&Value {
                variable: None,
                value: Some("4".to_string())
            })
            .unwrap()
        );
    }

    #[test]
    fn test_interpret_loop() {
                        /*
                         * LOAD_VAL 1
         i = 1           * WRITE_VAR ‘i’
                         * READ_VAR ‘i’
         while (i < 5) { * LOAD_VAL 5
             i = i + 1;  * IF_CMP_GE 10
         }               * READ_VAR ‘i’
                         * LOAD_VAL 1
         return i;       * ADD
                         * WRITE_VAR ‘i’
                         * GOTO 2
                         * RETURN_VALUE
                         * */
        let mut stack = Vec::new();
        let mut bytecode_list = Vec::new();
        bytecode_list.push(ByteCode { op: Op::LoadVal, value: Some("1".to_string()) });
        bytecode_list.push(ByteCode { op: Op::WriteVar, value: Some("‘i’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::ReadVar, value: Some("‘i’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::LoadVal, value: Some("5".to_string()) });
        bytecode_list.push(ByteCode { op: Op::IfCmpGe, value: Some("10".to_string()) });
        bytecode_list.push(ByteCode { op: Op::ReadVar, value: Some("‘i’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::LoadVal, value: Some("1".to_string()) });
        bytecode_list.push(ByteCode { op: Op::Add, value: None });
        bytecode_list.push(ByteCode { op: Op::WriteVar, value: Some("‘i’".to_string()) });
        bytecode_list.push(ByteCode { op: Op::Goto, value: Some("2".to_string()) });
        bytecode_list.push(ByteCode { op: Op::ReturnValue, value: None });

        assert_eq!(
            serde_json::to_string(&interpret(&mut stack, &mut bytecode_list).unwrap()).unwrap(),
            serde_json::to_string(&Value {
                variable: Some("‘i’".to_string()),
                value: Some("5".to_string())
            })
            .unwrap()
        );
    }
}

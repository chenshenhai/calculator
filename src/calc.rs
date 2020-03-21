// Copyright 2020 the chenshenhai/rust_note authors. All rights reserved. MIT license.

// This code has been ported almost directly from https://github.com/chenshenhai/rust_note/blob/master/demo/calc/src/calc.rs
// Copyright 2020 The Authors. All rights reserved. MIT license.
// https://github.com/chenshenhai/rust_note/blob/master/LICENSE


pub fn calc_expression(expr: &String) -> String {
    let rpn = parse_to_rpn(expr);
    let result = eval_rpn(rpn);
    return result;
}

pub fn eval_rpn(rpn: Vec<String>) -> String {
    let mut rpn = rpn;
    let mut rs: f64 = 0.0;
    let mut idx = 0;
    let mut result = "Error expression!".to_string();
    loop {
        if rpn.len() == 1 {
            result = rs.to_string();
            break;
        }
        if rpn.len() < 3 {
            break;
        }
        if idx + 2 >= rpn.len() {
            break;
        }
        let str_cur = rpn[idx].clone();
        let str_next = rpn[idx + 1].clone();
        let str_next_next = rpn[idx + 2].clone();
        if is_number_str(&str_cur) && is_number_str(&str_next) && is_operator_str(&str_next_next) {
            let num1 = str_cur.parse::<f64>().unwrap();
            let num2 = str_next.parse::<f64>().unwrap();
            if get_operator_char(&str_next_next) == '+' {
                rs = num1 + num2;
            } else if get_operator_char(&str_next_next) == '-' {
                rs = num1 - num2;
            } else if get_operator_char(&str_next_next) == '*' {
                rs = num1 * num2;
            } else if get_operator_char(&str_next_next) == '/' {
                rs = num1 / num2;
            }
            
            let mut temp_rpn = vec![];
            for i in 0..rpn.len() as usize {
                if i == idx {
                    temp_rpn.push(rs.to_string());
                } else if !(i == idx || i == idx + 1 || i == idx + 2) {
                    temp_rpn.push(rpn[i].clone());
                }
            }
            rpn.clear();
            rpn = temp_rpn.clone();
            temp_rpn.clear();
            idx = 0;
            continue;
        } else {
            idx += 1;   
        }

    }
   
    return result;
}

pub fn is_number_str(stri: &String) -> bool {
    let code_vec = stri.as_bytes().to_vec();
    let mut result = true;
    let mut point_count = 0;
    for i in 0..code_vec.len() as usize {
        if point_count > 1 {
            break;
        }
        if i == 0 && code_vec[i] == 45 {
            continue;
        } else if i != 0 && code_vec[i] == 46 {
            point_count += 1;
            continue;
        }
        if !(code_vec[i] >= 48 && code_vec[i] <= 57) {
            result = false;
            break; 
        }
    }
    return result;
}

fn is_operator_str(stri: &String) -> bool {
    let opt_char = get_operator_char(stri);
    let mut result = false;
    if opt_char == '+' || opt_char == '-' || opt_char == '*' || opt_char == '/' {
        result = true;
    }
    return result;
}


fn get_operator_char (stri: &String) -> char {
    let code_vec = stri.as_bytes().to_vec();
    let mut result = ' ';
    if code_vec.len() == 1 {
        if code_vec[0] == 43 {
            result = '+';
        } else if code_vec[0] == 45 {
            result = '-';
        } else if code_vec[0] == 42 {
            result = '*';
        } else if code_vec[0] == 47 {
            result = '/';
        }
    }
    return result;
}

// // 3+(2-5)*6 => 325-6*+
// 11+(22-33)*44+(55-66)+77/88 = -484
// Parse expression to Reverse-Polish-notation
pub fn parse_to_rpn(expr: &String) -> Vec<String> {
    let char_arr:Vec<char> = expr.chars().collect();
    // let mut prev_char: char = '^';
    let mut prev_str: String = "".to_string();
    // Reverse Polish notation
    let mut rpn_stack:Vec<String> = vec![];
    let mut operate_stack:Vec<String> = vec![];
    for i in 0..char_arr.len() as usize {
        let mut next_char = '$';
        if i < char_arr.len() - 1 {
            next_char = char_arr[i + 1];
        }
        // if i > 0 {
        //     prev_char = char_arr[i - 1];
        // }

        if char_arr[i].is_digit(10) && !next_char.is_digit(10) {
            prev_str.push_str(&char_arr[i].to_string());
            rpn_stack.push(prev_str.clone());
            if operate_stack.len() > 0 {
                let last_opt = get_operator_char(&operate_stack[operate_stack.len() - 1]);
                if last_opt == '*' || last_opt == '/' {
                    rpn_stack.push(operate_stack.pop().unwrap().to_string());
                }
            }
            prev_str.clear();
            continue;
        }

        if char_arr[i].is_digit(10) {
            prev_str.push_str(&char_arr[i].to_string());
            continue;
        }

       
        if char_arr[i] == '+' || char_arr[i] == '-' || char_arr[i] == '*' || char_arr[i] == '/' {
            operate_stack.push(char_arr[i].to_string());
            continue;
        }
        

        if char_arr[i] == ')' {
            rpn_stack.push(operate_stack.pop().unwrap().to_string());
        }
    }

    loop {
        let opt = operate_stack.pop();
        if opt.is_none() {
            break;
        }
        rpn_stack.push(opt.unwrap().to_string());
    }
    return rpn_stack;
}

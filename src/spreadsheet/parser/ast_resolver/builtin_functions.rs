use crate::common_types::{ComputeError, Value};

pub fn get_func(name: &str) -> Option<fn(Vec<Value>) -> Result<Value, ComputeError>> {
    match name {
        "sum" => Some(self::sum),
        "product" => Some(self::product),
        "max" => Some(self::max),
        "min" => Some(self::min),
        "average" => Some(self::average),
        "count" => Some(self::count),
        "length" => Some(self::length),
        "if" => Some(self::if_func),
        "round" => Some(self::round),
        "rand" => Some(self::rand_func),
        "pow" => Some(self::power),
        _ => None,
    }
}

pub fn sum(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut sum = 0.0;
    for arg in args {
        if let Value::Number(num) = arg {
            sum += num;
        } else {
            return Err(ComputeError::InvalidArgument("sum expects only numeric values".to_string()));
        }
    }
    Ok(Value::Number(sum))
}

pub fn product(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut prod = 1.0;
    for arg in args {
        if let Value::Number(num) = arg {
            prod *= num;
        } else {
            return Err(ComputeError::InvalidArgument("product expects only numeric values".to_string()));
        }
    }
    Ok(Value::Number(prod))
}

pub fn max(args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.is_empty() {
        return Err(ComputeError::InvalidArgument("max expects at least one numeric value".to_string()));
    }

    let mut max = f64::MIN;
    for arg in args {
        if let Value::Number(num) = arg {
            max = f64::max(max, num);
        } else {
            return Err(ComputeError::InvalidArgument("max expects only numeric values".to_string()));
        }
    }
    Ok(Value::Number(max))
}

pub fn min(args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.is_empty() {
        return Err(ComputeError::InvalidArgument("min expects at least one numeric value".to_string()));
    }

    let mut min = f64::MAX;
    for arg in args {
        if let Value::Number(num) = arg {
            min = f64::min(min, num);
        } else {
            return Err(ComputeError::InvalidArgument("min expects only numeric values".to_string()));
        }
    }
    Ok(Value::Number(min))
}

pub fn average(args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.is_empty() {
        return Err(ComputeError::InvalidArgument("average expects at least one numeric value".to_string()));
    }

    let mut sum = 0.0;
    let len = args.len() as f64;
    for arg in args {
        if let Value::Number(num) = arg {
            sum += num;
        } else {
            return Err(ComputeError::InvalidArgument("average expects only numeric values".to_string()));
        }
    }
    Ok(Value::Number(sum / len))
}

pub fn count(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut count = 0.0;
    for arg in args {
        if let Value::Number(_) = arg {
            count += 1.0;
        } else {
            return Err(ComputeError::InvalidArgument("count expects only numeric values".to_string()));
        }
    }
    Ok(Value::Number(count))
}

pub fn length(args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.len() != 1 {
        return Err(ComputeError::InvalidArgument("length expects exactly one argument".to_string()));
    }

    match &args[0] {
        Value::Text(t) => Ok(Value::Number(t.len() as f64)),
        _ => Err(ComputeError::InvalidArgument("length expects a string argument".to_string())),
    }
}

pub fn if_func(mut args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.len() != 3 {
        return Err(ComputeError::InvalidArgument("if expects exactly three arguments".to_string()));
    }

    match args[0] {
        Value::Bool(b) => {
            if b {
                Ok(args.remove(1))
            } else {
                Ok(args.remove(2))
            }
        }
        _ => Err(ComputeError::InvalidArgument("if expects a boolean as the first argument".to_string())),
    }
}

pub fn round(args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.len() != 1 {
        return Err(ComputeError::InvalidArgument("round expects exactly one numeric argument".to_string()));
    }

    match args[0] {
        Value::Number(num) => Ok(Value::Number(num.round())),
        _ => Err(ComputeError::InvalidArgument("round expects a numeric argument".to_string())),
    }
}

pub fn rand_func(args: Vec<Value>) -> Result<Value, ComputeError> {
    if !args.is_empty() {
        return Err(ComputeError::InvalidArgument("rand expects no arguments".to_string()));
    }

    Ok(Value::Number(rand::Rng::gen(&mut rand::thread_rng())))
}

pub fn power(mut args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.len() != 2 {
        return Err(ComputeError::InvalidArgument("pow expects exactly two numeric arguments".to_string()));
    }

    let num2 = args.pop().unwrap();
    let num1 = args.pop().unwrap();

    match (num1, num2) {
        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1.powf(n2))),
        _ => Err(ComputeError::InvalidArgument("pow expects both arguments to be numeric".to_string())),
    }
}

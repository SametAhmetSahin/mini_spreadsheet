use crate::common_types::{ComputeError, Value, AST};

pub fn get_func(name: &str) -> Option<fn(Vec<Value>) -> Result<Value, ComputeError>> {
    match name {
        "sum" => Some(self::sum),
        "product" => Some(self::product),
        "max" => Some(self::max),
        "min" => Some(self::min),
        "average" => Some(self::average),
        "count" => Some(self::count),
        "length" => Some(self::length),
        _ => None,
    }
}

pub fn sum(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut sum = 0.0;
    for arg in args {
        if let Value::Number(num) = arg {
            sum += num
        } else {
            return Err(ComputeError::TypeError);
        }
    }
    Ok(Value::Number(sum))
}

pub fn product(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut prod = 1.0;
    for arg in args {
        if let Value::Number(num) = arg {
            prod *= num
        } else {
            return Err(ComputeError::TypeError);
        }
    }
    Ok(Value::Number(prod))
}

pub fn max(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut max = f64::MIN;
    for arg in args {
        if let Value::Number(num) = arg {
            max = f64::max(max, num)
        } else {
            return Err(ComputeError::TypeError);
        }
    }
    Ok(Value::Number(max))
}

pub fn min(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut min = f64::MAX;
    for arg in args {
        if let Value::Number(num) = arg {
            min = f64::min(min, num)
        } else {
            return Err(ComputeError::TypeError);
        }
    }
    Ok(Value::Number(min))
}

pub fn average(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut sum = 0.0;
    let len = args.len() as f64;
    for arg in args {
        if let Value::Number(num) = arg {
            sum += num
        } else {
            return Err(ComputeError::TypeError);
        }
    }
    let avg = sum / len;

    Ok(Value::Number(avg))
}

/// Count the number of numeric entries
pub fn count(args: Vec<Value>) -> Result<Value, ComputeError> {
    let mut count = 0.0;
    for arg in args {
        if let Value::Number(num) = arg {
            count += 1.0;
        } else {
            return Err(ComputeError::TypeError);
        }
    }

    Ok(Value::Number(count))
}

pub fn length(args: Vec<Value>) -> Result<Value, ComputeError> {
    if args.len() != 1 {
        Err(ComputeError::TypeError)
    } else {
        match &args[0] {
            Value::Text(t) => Ok(Value::Number(t.len() as f64)),
            Value::Number(_) => Err(ComputeError::TypeError),
            Value::Bool(_) => Err(ComputeError::TypeError),
        }
    }
}

// TODO logical functions and logical expressions

use std::io;

fn main() {
    println!("Calculate something!");

    loop {
        let (mut variable1, mut variable2) = (String::new(), String::new());

        println!("Please input your first number");

        io::stdin().read_line(&mut variable1)
            .expect("Failed to read line");
        
        println!("Please input your second number");

        io::stdin().read_line(&mut variable2)
            .expect("Failed to read line");
        
        let (variable1, variable2): (f64, f64) = 
            match (variable1.trim().parse(), variable2.trim().parse()){
                (Ok(num1),Ok(num2)) => (num1,num2),
                (Err(_1), Err(_2))  => continue,
                (Ok(num1), Err(_2)) => continue,
                (Err(_1), Ok(num2)) => continue,
        };

        println!("What opperation you want to use? +,-,/ or *?");

        let mut operation = String::new();

        io::stdin().read_line(&mut operation)
            .expect("Failed to read line");
        
        let operation: String = match operation.trim().parse(){
            Ok(char) => char,
            Err(_) => continue,
        };

        calculate(&operation, &variable1, &variable2);
    }
}

fn calculate(operation: &String, variable1: &f64, variable2: &f64){
    match operation.as_ref() {
            "plus" | "+"   => println!("result is: {}",plus(&variable1, &variable2)),
            "minus" | "-"  => println!("result is: {}",minus(&variable1, &variable2)),
            "times" | "*"  => println!("result is: {}",times(&variable1, &variable2)),
            "devide" | "/" => println!("result is: {}",devide(&variable1, &variable2)),
            _ => println!("Please choose plus, minus, devide, times"),
    }
}

fn plus(x: &f64, y: &f64) -> f64{
    x + y
}

fn minus(x: &f64, y: &f64) -> f64{
    x - y
}

fn devide(x: &f64, y: &f64) -> f64{
    x / y
}

fn times(x: &f64, y: &f64) -> f64{
    x * y
}
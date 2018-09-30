use std::io;

fn main() {
    println!("Calculate something!");
    
    loop {
        println!("Please input your first number");

        let mut variable1 = String::new();

        io::stdin().read_line(&mut variable1)
            .expect("Failed to read line");

        let variable1: f64 = match variable1.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Please input your second number");

        let mut variable2 = String::new();

        io::stdin().read_line(&mut variable2)
            .expect("Failed to read line");
            
        let variable2: f64 = match variable2.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("What opperation you want to use? +,-,/ or *?");

        let mut operation = String::new();

        io::stdin().read_line(&mut operation)
            .expect("Failed to read line");
        
        let operation: String = match operation.trim().parse(){
            Ok(char) => char,
            Err(_) => continue,
        };

        if operation == "plus"{
            let result = plus(&variable1, &variable2);
            println!("The result is {}",result);
        } else if operation == "minus"{
            let result = minus(&variable1, &variable2);
            println!("The result is {}",result);
        } else if operation == "devide"{
            let result = devide(&variable1, &variable2);
            println!("The result is {}",result);
        } else if operation == "times"{
            let result = times(&variable1, &variable2);
            println!("The result is {}",result);
        } else{
            println!("Please choose plus, minus, devide, times");
        }
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
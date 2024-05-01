use std::io;

fn main() {
    println!("Todo list!");
    let mut tasks = Vec::new();
    loop {
        println!("Please select one operation");
        println!("1.Add\n2.Remove\n3.Show\n4.Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: Failed to read input from stdin");

        let choice: i32 = input.trim().parse().expect("Error: Failed to read input from stdin");

       

        match choice {
            1 => {
                println!("Enter a task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Error: Failed to read input from stdin");
                let trimmed_task = task.trim().to_string();
                tasks.push(trimmed_task);
            }
            2 => {
                println!("Enter task to remove:");
                let mut task_for_removing = String::new();
                io::stdin().read_line(&mut task_for_removing).expect("Error: Failed to read input from stdin");
                let trimmed_removed_task = task_for_removing.trim().to_string();
                let mut found = false;
                tasks.retain(|task| {
                    if task.trim() == trimmed_removed_task {
                        found = true;
                        false
                    } else {
                        true
                    }
                });

                if found {
                    println!("Removed task: {}", trimmed_removed_task);
                } else {
                    println!("Task not found");
                }

            }
            3 => {
                if tasks.is_empty() {
                    println!("Todo list is empty");
                    break;
                }
                println!("These are your tasks:{:?}", tasks);
            }
            4 => {
                println!("Thank u for using this app , Pleasee Visit again!");
                break;
            }
            _ => println!("Please select any options"),
        }
    }
}

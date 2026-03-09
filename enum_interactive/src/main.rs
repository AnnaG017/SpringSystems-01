use std::io;
use std::process::Command;
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}
// Executes the chosen file using the system commands
fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");
        }
        FileOperation::Display(path) => {
            Command::new("cat")
                .arg(path)
                .status()
                .expect("Failed to execute cat");
        }
        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");
        }
        FileOperation::Remove(path) => {
            Command::new("rm")
                .arg(path)
                .status()
                .expect("Failed to remove file");
        }
        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}


fn main(){
    loop{
        //prints menu
        println!("File Operation Menu:\n");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        println!("Enter your choice (0-5): ");
        
        //reads the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        match input.trim() {
            //options that user can input
            "0" => {
                println!("Goodbye!");
                break; // Exit the loop 
            }
            "1" => {
                println!("You chose: List file \n");
                println!("Enter file path:");
                let mut path = String::new();

                io::stdin().read_line(&mut path).expect("Failed to read path");
                let operation = FileOperation::List(path.trim().to_string());
                println!("Created List operation");
                perform_operation(operation);

            }
            "2" => {
                println!("You chose: Display files content\n");
                println!("Enter directory path:");
                let mut path = String::new();

                io::stdin().read_line(&mut path).expect("Failed to read path");
                let operation = FileOperation::Display(path.trim().to_string());
                println!("Created Display operation");
                perform_operation(operation);
            }
            "3" => {
                println!("You chose: Create a new file");
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read file path");

                println!("Enter file content:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).expect("Failed to read content");

                let operation = FileOperation::Create(
                    path.trim().to_string(),
                    content.trim().to_string()
                );
                perform_operation(operation);
                println!("Created Create operation")
            }

            "4" =>{
                println!("You chose: Remove a file\n");
                println!("Enter file path:");
                let mut path = String::new();
                
                io::stdin().read_line(&mut path).expect("Failed to read file path");
                let operation = FileOperation::Remove(path.trim().to_string());
                println!("Created Remove operation");
                perform_operation(operation);

            }
            "5" =>{
                println!("You chose: Print working directory");
                let operation = FileOperation::Pwd;
                println!("Created Pwd operation\n");
                perform_operation(operation);
            }
            _ => println!("Invalid \n"),
        }
    }
}

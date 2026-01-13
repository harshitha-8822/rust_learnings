mod models;
mod utils;


use models::Task;

// add task, list task, mark done, exit

fn find_task_mut(tasks: &mut Vec<Task>, id: u32) -> Option<&mut Task>{
    tasks.iter_mut().find(|t| t.id==id)
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop{
        println!("\n===== Task Manager CLI =====");
        println!("1) Add Task");
        println!("2) List Tasks");
        println!("3) Mark Done");
        println!("4) Exit");

        let choice = utils::read_u32("Enter choice: ");

        match choice{
            1 => {
                let title = utils::read_line("Enter task title: ");
                let id = (tasks.len()+1) as u32;
                // tasks.len() return usize  
                let task = Task::new(id, title);
                tasks.push(task);

                println!("task added");
            }

            2=> {
                if tasks.is_empty(){
                    println!("No tasks found. ");
                }
                else{
                    println!("\n---Task List---");
                    for task in &tasks{
                        task.display();
                    }
                }
            }

            3 => {
                let id = utils::read_u32("Enter task id to mark done: ");
                match find_task_mut(&mut tasks, id){
                    Some(task)=>{
                        task.mark_done();
                        println!("task marked as done!!!!!");
                    }
                    None=>{
                        println!("Task not found");
                    }
                }
            }

            4 => {
                println!("Exitinggggggggg");
                break;
            }

            _ => println!("Invalid choice. Try 1-4"),
        }


    }
}

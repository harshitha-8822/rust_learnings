#[derive(Debug)]
pub enum Status{
    Todo, 
    Done,
}

pub struct Task{
    pub id: u32,
    pub title: String,
    pub status: Status,
}

impl Task{

    pub fn new(id: u32, title: String)-> Self{
        Self{id, title, status: Status::Todo}
    }
    pub fn mark_done(&mut self){
        self.status = Status::Done;
    }

    pub fn display(&self){
        // task - todo/Done
        let status_text = match self.status{
            Status::Todo => "Todo",
            Status::Done => "Done",
        };
        println!(" [{}] {} - {}", self.id, self.title, status_text);
    }
}


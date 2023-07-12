pub struct Queue<T> {
    commands: Vec<T>,
    marker: usize, //points to the first element in the queue
}

impl<T:Copy> Queue<T>
    {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            marker: 0,
        }
    }

    pub fn push(&mut self, command: T) {
        self.commands.push(command);
    }

    pub fn pop(&mut self) -> T {
        let old_marker = self.marker;
        self.marker += 1;
        self.commands[old_marker]
    }

    /*
    pub fn get_number_of(&self, command: input_parser::InputCommand) -> usize {
        let mut total_num: usize = 0;
        for element in self.commands.iter() {
            if *element == command {
                total_num += 1;
            }
        }
        total_num
    }
    */

    pub fn size(&self) -> usize {
        self.commands.len()
    }
}

use crate::data_structures;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputCommand{
    Right,
    Left,
    Up,
    Down,
    Wait,
    Quit,
}

fn read_user_input() -> String {
    let mut line = String::new();
    //(NOTE) byte_size: number of characters read + 2 (one is for entry and the other idk)
    let _byte_size = std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn check_for_starting_command_letters(c: &char) -> bool {
    c == &'r' || c == &'w' || c == &'d' || c == &'u' || c == &'l' || c == &'q'
}

//(NOTE): bugs in the parser
//when the end of a command is wrong (ex. 10rightlkasdf) it still passes
//if there is only a number (ex. 10right25) the program crashes because
//of the unwrap checking the next index that is null

//both of these errors are quite easy to fix via match statement and option
//but I'm too lazy right now to fix it
pub fn get_parsed_user_input_map() -> data_structures::Queue{
        //(NOTE) commands available in map mode
        let mut current_number = 1;
        let mut queue = data_structures::Queue::new();
        let mut i = 0;

        let command: String = read_user_input().chars().filter(|c| !c.is_whitespace()).collect();

        while i < command.chars().count() {
            //println!("i equals {}", i);
            let c = command.chars().nth(i).unwrap();

            if c.is_numeric() {
                current_number = c as u32 - '0' as u32;

                //CHECKING IF IT'S A MULTIPLE DIGIT NUMBER
                let mut end_of_number = false;
                let mut j = 1;

                while !end_of_number {
                    let c = command.chars().nth(i + j).unwrap();
                    match c.is_numeric() {
                        true => {
                            let rightmost_number = c as u32 - '0' as u32;
                            current_number = current_number * 10 + rightmost_number;
                            j += 1;
                        },
                        false => {
                            end_of_number = true;
                            i += j - 1;
                        }
                    }
                }
            }
            //IT'S NOT A NUMBER SO WE ARE CHECKING IF THE COMMAND IS RIGHT
            else if check_for_starting_command_letters(&c) {
                //right
                if c == 'r'
                    && command.chars().nth(i + 1).unwrap() == 'i'
                    && command.chars().nth(i + 2).unwrap() == 'g'
                    && command.chars().nth(i + 3).unwrap() == 'h'
                    && command.chars().nth(i + 4).unwrap() == 't'
                {
                    for _ in 0..current_number {
                        queue.push(InputCommand::Right);
                    }
                    current_number = 1;
                    i += 5;
                    continue;
                }
                //left
                else if c == 'l'
                    && command.chars().nth(i + 1).unwrap() == 'e'
                    && command.chars().nth(i + 2).unwrap() == 'f'
                    && command.chars().nth(i + 3).unwrap() == 't'
                {
                    for _ in 0..current_number {
                        queue.push(InputCommand::Left);
                    }
                    current_number = 1;
                    i += 4;
                    continue;
                }
                //up
                else if c == 'u' && command.chars().nth(i + 1).unwrap() == 'p' {
                    for _ in 0..current_number {
                        queue.push(InputCommand::Up);
                    }
                    current_number = 1;
                    i += 2;
                    continue;
                }
                //down
                else if c == 'd'
                    && command.chars().nth(i + 1).unwrap() == 'o'
                    && command.chars().nth(i + 2).unwrap() == 'w'
                    && command.chars().nth(i + 3).unwrap() == 'n'
                {
                    for _ in 0..current_number {
                        queue.push(InputCommand::Down);
                    }
                    current_number = 1;
                    i += 4;
                    continue;
                }
                //wait
                else if c == 'w'
                    && command.chars().nth(i + 1).unwrap() == 'a'
                    && command.chars().nth(i + 2).unwrap() == 'i'
                    && command.chars().nth(i + 3).unwrap() == 't'
                {
                    for _ in 0..current_number {
                        queue.push(InputCommand::Wait);
                    }
                    current_number = 1;
                    i += 4;
                    continue;
                }
                //quit
                else if c == 'q'
                    && command.chars().nth(i + 1).unwrap() == 'u'
                    && command.chars().nth(i + 2).unwrap() == 'i'
                    && command.chars().nth(i + 3).unwrap() == 't'
                {
                    queue.push(InputCommand::Quit);
                    current_number = 1;
                    i += 4;
                    continue;
                }            
            }
            i += 1;
        }
        //Debugging
        //println!("current number {}, right amount {} left amount {} up amount {} down amount {} wait amount {} quit amount {}", current_number, queue.get_number_of(InputCommand::Right), queue.get_number_of(InputCommand::Left), queue.get_number_of(InputCommand::Up), queue.get_number_of(InputCommand::Down), queue.get_number_of(InputCommand::Wait), queue.get_number_of(InputCommand::Quit));
    queue
}

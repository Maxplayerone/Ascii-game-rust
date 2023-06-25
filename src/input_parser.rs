use crate::data_structures;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputCommand {
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

enum MessageError {
    OnlyNumber,
    IncorrectCommand,
    CommandTypedIncorrectly,
}

fn show_helpful_message(error: MessageError) {
    match error {
        MessageError::OnlyNumber => {
            println!("-------------------------------");
            println!("CommandError: number only given");
            println!("\nAfter typing the number \nyou should type the command \nyou want to do \n(ex. 10 right)\n(moving right ten units)");
            println!("-------------------------------");
        }
        MessageError::IncorrectCommand => {
            println!("-------------------------------");
            println!("CommandError: incorrect command");
            println!("\ncommands should look like this:\n2 right 5 up\nfor going 2 units right and 2 up\nfor more commands check \n'info' command or info in menu");
            println!("-------------------------------");
        }
        MessageError::CommandTypedIncorrectly => {
            println!("-------------------------------");
            println!("CommandError: command typed incorrectly");
            println!("\nupps, a misstroke \njust write the command again\nor check 'info' to see\nhow to write the commands");
            println!("-------------------------------");
        }
    }
}

fn check_if_char_is_correct(char_option: Option<char>, char_to_compare: char) -> bool {
    match char_option {
        Some(c) => {
            if c == char_to_compare {
                ()
            }
        }
        None => {
            show_helpful_message(MessageError::IncorrectCommand);
            return false;
        }
    }
    true
}

pub fn get_parsed_user_input_map() -> data_structures::Queue {
    //(NOTE) commands available in map mode
    let mut current_number = 1;
    let mut queue = data_structures::Queue::new();
    let mut i = 0;

    let command: String = read_user_input()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    while i < command.chars().count() {
        //println!("i equals {}", i);
        let c = command.chars().nth(i).unwrap();

        if c.is_numeric() {
            current_number = c as u32 - '0' as u32;

            //CHECKING IF IT'S A MULTIPLE DIGIT NUMBER
            let mut end_of_number = false;
            let mut j = 1;

            while !end_of_number {
                let c: Option<char> = command.chars().nth(i + j);
                match c {
                    Some(c) => match c.is_numeric() {
                        true => {
                            let rightmost_number = c as u32 - '0' as u32;
                            current_number = current_number * 10 + rightmost_number;
                            j += 1;
                        }
                        false => {
                            end_of_number = true;
                            i += j - 1;
                        }
                    },
                    None => {
                        show_helpful_message(MessageError::OnlyNumber);
                        return get_parsed_user_input_map();
                    }
                }
            }
        }
        //IT'S NOT A NUMBER SO WE ARE CHECKING IF THE COMMAND IS RIGHT
        else {
            let is_starting_command_letter = check_for_starting_command_letters(&c);
            match is_starting_command_letter {
                true => (),
                false => {
                    show_helpful_message(MessageError::IncorrectCommand);
                    return get_parsed_user_input_map();
                }
            }
            //right
            if c == 'r' {
                //checking if the command is right
                if !check_if_char_is_correct(command.chars().nth(i + 1), 'i'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 2), 'g'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 3), 'h'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 4), 't'){
                    return get_parsed_user_input_map();
                }

                //checking if the last letter is wrong
                let wrong_letter_after_command = command.chars().nth(i + 5);
                match wrong_letter_after_command {
                    Some(letter) => {
                        if !letter.is_numeric() {
                            show_helpful_message(MessageError::CommandTypedIncorrectly);
                            return get_parsed_user_input_map();
                        }
                    }
                    None => (),
                }

                for _ in 0..current_number {
                    queue.push(InputCommand::Right);
                }
                current_number = 1;
                i += 5;
                continue;
            }
            //left
            else if c == 'l'
            {
                if !check_if_char_is_correct(command.chars().nth(i + 1), 'e'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 2), 'f'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 3), 't'){
                    return get_parsed_user_input_map();
                }
                let wrong_letter_after_command = command.chars().nth(i + 5);
                match wrong_letter_after_command {
                    Some(letter) => {
                        if !letter.is_numeric() {
                            show_helpful_message(MessageError::CommandTypedIncorrectly);
                            return get_parsed_user_input_map();
                        }
                    }
                    None => (),
                }
                for _ in 0..current_number {
                    queue.push(InputCommand::Left);
                }
                current_number = 1;
                i += 4;
                continue;
            }
            //up
            else if c == 'u'{
                if !check_if_char_is_correct(command.chars().nth(i + 1), 'p'){
                    return get_parsed_user_input_map();
                }

                let wrong_letter_after_command = command.chars().nth(i + 5);
                match wrong_letter_after_command {
                    Some(letter) => {
                        if !letter.is_numeric() {
                            show_helpful_message(MessageError::CommandTypedIncorrectly);
                            return get_parsed_user_input_map();
                        }
                    }
                    None => (),
                }
                for _ in 0..current_number {
                    queue.push(InputCommand::Up);
                }
                current_number = 1;
                i += 2;
                continue;
            }
            //down
            else if c == 'd'
            {
                if !check_if_char_is_correct(command.chars().nth(i + 1), 'o'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 2), 'w'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 3), 'n'){
                    return get_parsed_user_input_map();
                }
                let wrong_letter_after_command = command.chars().nth(i + 5);
                match wrong_letter_after_command {
                    Some(letter) => {
                        if !letter.is_numeric() {
                            show_helpful_message(MessageError::CommandTypedIncorrectly);
                            return get_parsed_user_input_map();
                        }
                    }
                    None => (),
                }
                for _ in 0..current_number {
                    queue.push(InputCommand::Down);
                }
                current_number = 1;
                i += 4;
                continue;
            }
            //wait
            else if c == 'w'
            {
                if !check_if_char_is_correct(command.chars().nth(i + 1), 'a'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 2), 'i'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 3), 't'){
                    return get_parsed_user_input_map();
                }
                let wrong_letter_after_command = command.chars().nth(i + 5);
                match wrong_letter_after_command {
                    Some(letter) => {
                        if !letter.is_numeric() {
                            show_helpful_message(MessageError::CommandTypedIncorrectly);
                            return get_parsed_user_input_map();
                        }
                    }
                    None => (),
                }
                for _ in 0..current_number {
                    queue.push(InputCommand::Wait);
                }
                current_number = 1;
                i += 4;
                continue;
            }
            //quit
            else if c == 'q'
            {
                if !check_if_char_is_correct(command.chars().nth(i + 1), 'u'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 2), 'i'){
                    return get_parsed_user_input_map();
                }
                if !check_if_char_is_correct(command.chars().nth(i + 3), 't'){
                    return get_parsed_user_input_map();
                }
                let wrong_letter_after_command = command.chars().nth(i + 5);
                match wrong_letter_after_command {
                    Some(letter) => {
                        if !letter.is_numeric() {
                            show_helpful_message(MessageError::CommandTypedIncorrectly);
                            return get_parsed_user_input_map();
                        }
                    }
                    None => (),
                }
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

use std::io;
use std::vec::Vec;
use std::fs::File;
use std::io::{Write, BufRead, BufReader, BufWriter};
use std::path::Path;
use colour::red_ln;

struct Date {
    y: u32,
    m: u8,
    d: u8,
}

struct Profile{
    id:       u64,
    name:     String,
    birthday: Date,
    home:     String,
    comment:  String,
}

fn command_quit(){
    std::process::exit(0);
}

fn command_check(profile_data: &mut Vec<Profile>){
    println!("{} profile(s)", profile_data.len());
}

fn command_print(profile_data: &mut Vec<Profile>, n: i32){
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let length = profile_data.len() as i32;

    if length == 0{
        println!("No Data");
        return;
    }

    if n.abs() > length{
        red_ln!("value error: argument is too large\nnow profile size is {}", length);
        return;
    }

    if n > 0 {
        end = n;
    } else if n == 0 {
        end = length;
    } else if n < 0{
        start = length + n;
        end = length;
    }

    for i in (start as usize)..(end as usize){
        let p = &profile_data[i];
        println!("Id:\t{}", p.id);
        println!("Name:\t{}", p.name);
        println!("Birth:\t{0}-{1}-{2}", p.birthday.y, p.birthday.m, p.birthday.d);
        println!("Addr:\t{}", p.home);
        println!("Note:\t{}\n", p.comment);
    }
}

fn command_read(profile_data: &mut Vec<Profile>, file_path: &str){
    let input_file = File::open(file_path);

    match &input_file {
        Ok(_) => (),
        Err(_) => {
            red_ln!("No such file or directory");
            return;
        },
    }

    let reader = BufReader::new(input_file.unwrap());

    for line in reader.lines(){
        let line = line.unwrap();
        let input_data: Vec<&str> = line.split(',').collect();
        if input_data.len() != 5{
            red_ln!("Invalid data.");
            continue;
        }
        add_profile(profile_data, input_data);
    }
}

fn command_write(profile_data: &mut Vec<Profile>, file_path: &str){

    let file_exists: bool = Path::new(file_path).exists();

    if file_exists {
        let mut input = String::new();

        println!("Overwrite {}？ Type yes or no", file_path);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.replace("\n", "");

        while !(input == "yes" || input == "no") {
            input = String::new();
            println!("Type yes or no");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.replace("\n", "");
        }

        if input == "yes" {
            let output_file = File::create(file_path);

            match &output_file {
                Ok(_) => (),
                Err(_) => {
                    red_ln!("{} cannot create", file_path);
                    return;
                },
            }

            let mut writer = BufWriter::new(output_file.unwrap());

            for p in profile_data {
                 let write_str = format!("{0},{1},{2}-{3}-{4},{5},{6}\n", p.id, p.name, p.birthday.y, p.birthday.m, p.birthday.d, p.home, p.comment);
                 writer.write_all(write_str.as_bytes()).unwrap();
            }
            println!("Saved {}", file_path);
        }
    } else {
        let output_file = File::create(file_path);

        match &output_file {
            Ok(_) => (),
            Err(_) => {
                red_ln!("{} cannot create", file_path);
                return;
            },
        }

        let mut writer = BufWriter::new(output_file.unwrap());

        for p in profile_data {
             let write_str = format!("{0},{1},{2}-{3}-{4},{5},{6}\n", p.id, p.name, p.birthday.y, p.birthday.m, p.birthday.d, p.home, p.comment);
             writer.write_all(write_str.as_bytes()).unwrap();
        }
        println!("Saved {}", file_path);
    }
}

fn command_find(profile_data: &mut Vec<Profile>, search_data: &str){

    let search_string = search_data.to_string();
    let mut find_flag: bool = false;

    for p in profile_data {
        let id_string = p.id.to_string();
        let birthday_string = format!("{0}-{1}-{2}", p.birthday.y, p.birthday.m, p.birthday.d);

        if id_string == search_string || p.name == search_string || birthday_string == search_string || p.home == search_string || p.comment == search_string {
            if !find_flag {
                find_flag = true;
            }
            println!("Id:\t{}", p.id);
            println!("Name:\t{}", p.name);
            println!("Birth:\t{0}-{1}-{2}", p.birthday.y, p.birthday.m, p.birthday.d);
            println!("Addr:\t{}", p.home);
            println!("Note:\t{}\n", p.comment);
        }
    }
    if !find_flag {
        red_ln!("Not found");
    }
}

fn command_sort(profile_data: &mut Vec<Profile>, n: usize){
    match n{
        1 => sort_by_id(profile_data),
        2 => sort_by_name(profile_data),
        3 => sort_by_birthday(profile_data),
        4 => sort_by_home(profile_data),
        5 => sort_by_comment(profile_data),
        _ => {
              red_ln!("value error: argument is 1 to 5");
              println!("\t1: sort by \"Id\"");
              println!("\t2: sort by \"Name\"");
              println!("\t3: sort by \"Birth\"");
              println!("\t4: sort by \"Addr\"");
              println!("\t5: sort by \"Note\"");
        }
    }
}

fn sort_by_id(profile_data: &mut Vec<Profile>){
    println!("Sort by \"Id\"");
    profile_data.sort_by(|a, b|a.id.cmp(&b.id));
}

fn sort_by_name(profile_data: &mut Vec<Profile>){
    println!("Sort by \"Name\"");
    profile_data.sort_by(|a, b|a.name.cmp(&b.name));
}

fn sort_by_birthday(profile_data: &mut Vec<Profile>){
    println!("Sort by \"Birth\"");
    profile_data.sort_by(|a, b|(a.birthday.y, a.birthday.m, a.birthday.d).cmp(&(b.birthday.y, b.birthday.m, b.birthday.d)));
}

fn sort_by_home(profile_data: &mut Vec<Profile>){
    println!("Sort by \"Addr\"");
    profile_data.sort_by(|a, b|a.home.cmp(&b.home));
}

fn sort_by_comment(profile_data: &mut Vec<Profile>){
    println!("Sort by \"Note\"");
    profile_data.sort_by(|a, b|a.comment.cmp(&b.comment));
}

fn command_delete(profile_data: &mut Vec<Profile>, id: u64){
    for (i, p) in profile_data.iter().enumerate(){
        if p.id == id {
            profile_data.remove(i);
            println!("Delete ID:{}", id);
            return;
        }
    }
}

fn exec_command(command: Vec<&str>, profile_data: &mut Vec<Profile>){
    match command[0]{
        "%Q" => if command.len() == 1{
                    command_quit();
                } else {
                    red_ln!("argument error: %Q has no argument");
                },
        "%C" => if command.len() == 1{
                    command_check(profile_data);
                } else {
                    red_ln!("argument error: %C has no argument");
                },
        "%P" => if command.len() == 2 {
                    let n: Result<i32, std::num::ParseIntError> = command[1].parse();
                    match n{
                        Ok(n) => command_print(profile_data, n),
                        Err(_) => red_ln!("value error: argument must be integer"),
                    }
                } else {
                    red_ln!("argument error: %P has 1 argument");
                },
        "%R" => if command.len() == 2 {
                    command_read(profile_data, command[1]);
                } else {
                    red_ln!("argument error: %R has 1 argument");
                },
        "%W" => if command.len() == 2 {
                    command_write(profile_data, command[1]);
                } else {
                    red_ln!("argument error: %W has 1 argument");
                },
        "%F" => if command.len() == 2 {
                    command_find(profile_data, command[1]);
                } else {
                    red_ln!("argument error: %F has 1 argument");
                },
        "%S" => if command.len() == 2 {
                    let n: Result<usize, std::num::ParseIntError> = command[1].parse();
                    match n{
                        Ok(n) => command_sort(profile_data, n),
                        Err(_) => red_ln!("value error: argument must be integer"),
                    }
                } else {
                    red_ln!("argument error: %S has 1 argument");
                },
        "%D" =>  if command.len() == 2 {
                    let id: Result<u64, std::num::ParseIntError> = command[1].parse();
                    match id{
                        Ok(id) => command_delete(profile_data, id),
                        Err(_) => red_ln!("value error: argument must be integer"),
                    }
                } else {
                    red_ln!("argument error: %D has 1 argument");
                },
        _ => {
            red_ln!("Invalid command");
        },
    }
}

fn add_profile(profile_data: &mut Vec<Profile>, input_data: Vec<&str>){
    let id_result: Result<u64, std::num::ParseIntError> = input_data[0].parse();

    match id_result{
        Ok(_) => (),
        Err(_) => {
            red_ln!("Invalid id");
            return;
        },
    }

    let id: u64 = id_result.unwrap();

    for p in profile_data.iter(){
        if p.id == id{
            red_ln!("ID:{} is already exist", id);
            return;
        }
    }

    let date_list: Vec<&str> = input_data[2].split('-').collect();

    if date_list.len() < 3 {
        red_ln!("Invalid date");
        return;
    }

    let y_result: Result<u32, std::num::ParseIntError> = date_list[0].parse();
    let m_result: Result<u8, std::num::ParseIntError> = date_list[1].parse();
    let d_result: Result<u8, std::num::ParseIntError> = date_list[2].parse();

    match y_result{
        Ok(_) => (),
        Err(_) => {
            red_ln!("Invalid year");
            return;
        },
    }

    match m_result{
        Ok(_) => (),
        Err(_) => {
            red_ln!("Invalid month");
            return;
        },
    }

    match d_result{
        Ok(_) => (),
        Err(_) => {
            red_ln!("Invalid day");
            return;
        },
    }

    let birthday_struct = Date {
                                y: y_result.unwrap(),
                                m: m_result.unwrap(),
                                d: d_result.unwrap(),
                               };

    let check_date = valid_date(birthday_struct.y, birthday_struct.m, birthday_struct.d);
    match check_date {
        -1 => {
            red_ln!("Invalid month");
            return;
        },
        -2 => {
            red_ln!("Invalid day");
            return;
        },
        _ => (),
    }

    let push_data = Profile {
                             id: id,
                             name: input_data[1].to_string(),
                             birthday: birthday_struct,
                             home: input_data[3].to_string(),
                             comment: input_data[4].to_string(),
                            };

    profile_data.push(push_data);
}

fn valid_date(y: u32, m: u8, d: u8) -> i8{

    let days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if m < 1 || m > 12 {
        return -1;
    }

    let mut last_day = days[(m as usize) - 1];
    if m == 2 {
        if y % 4 == 0 && y % 100 != 0 || y % 400 == 0{
            last_day = 29;
        }
    }

    if d < 1 || d > last_day{
        return -2;
    }

    return 0;
}

fn main(){
    let mut profile_data: Vec<Profile> = Vec::with_capacity(10000);

    loop{
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.replace("\n", "");

        if input.starts_with("%"){
            let command = input.splitn(2, ' ').collect();
            exec_command(command, &mut profile_data);
        } else {
            let input_data: Vec<&str> = input.splitn(5, ',').collect();
            if input_data.len() < 5{
                println!("Invalid data");
                continue;
            }
            add_profile(&mut profile_data, input_data);
        }
    }
}
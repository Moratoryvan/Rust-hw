use regex::Regex;
use std::env;
use std::process;
use colored::*;

pub mod intersec;
pub mod find;
// use crate find::find;

/* ./myfind -v -path [path] -name [expression] */
/* -v 只能加在最前面，
   -path 之后 -name 之前的字符串都是路径
   -name 之后的字符串都是表达式 */


fn main() {
    let args: Vec<String> = env::args().collect();
    let error = "Error";
    if args.len() < 5 {
        eprintln!("{}: 使用方式：{} -path <目标目录> -name <要搜索的正则表达式>", error.red(), args[0]);
        process::exit(1);
    } 

    let mut verbox: bool = false; // 用来表示是否打印全部内容
    let mut paths: Vec<String> = vec![]; // 路径向量
    let mut names: Vec<String> = vec![]; // 表达式向量
    let mut i = 1;

    loop {
        // 循环寻找路径表达式
        if &args[i] == "-v" || &args[i] == "--verbose" {
            verbox = true;
        } else if &args[i] == "-path" {
            'path: loop {
                i += 1; 
                if &args[i] != "-name" {
                    paths.push(args[i].clone());
                } else {
                    i -= 1;
                    break 'path; 
                }
            }
         } else if &args[i] == "-name" {
            'name: loop{
                i += 1;
                if i < args.len() {
                    names.push(args[i].clone());
                } else {
                    break 'name;
                }
            }
        }
        i += 1;
        if i >= args.len() {
            break;
        }
    }

    //检查寻找的路径和表达式是否为空，如果是的话说明格式错误，报错
    if paths.is_empty() || names.is_empty() {
        eprintln!("{}: 使用方式：{} -path <目标目录> -name <要搜索的正则表达式>", error.red(), args[0]);
        process::exit(1);
    }

    // matches 存储的是最终的结果
    let mut matches: Vec<String> = vec![];// final result 
    if verbox {
        println!("从以下文件中寻找:");
    }

    // 遍历每一个路径。每一个表达式，进行查找
    for name in &names {
        let pattern = &name;
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(err) => {
                eprintln!("{}: 无效的正则表达式 '{}': {}", error.red(), pattern.red(), err);
                process::exit(1);
            }
        };
        for path in &paths {
            match find::find(path,&regex,&verbox) {
                // 对于每一个找到的结果，将其与最终结果相交（如果当前最终结果不为空）
                Ok(r#match) => {
                    matches = if matches.is_empty() {r#match} else {intersec::intersection(&r#match, &matches)};
                }
                Err(error) => {
                    eprintln!("发生错误：{}", error);
                    process::exit(1);
                }
            }
        }
    }

    if matches.is_empty() {
        println!("未找到匹配项。");
    } else {
        println!("找到以下匹配项：");
        for file in matches {
            println!("{}",file.yellow());
        }
    }
    
}



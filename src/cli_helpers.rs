use std::env;
pub fn extract_cli_args() -> String {
    let args: Vec<String> = env::args().collect();
    let arg_count:i32 = args.len() as i32;
    let arg_list: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    let mut path_index:i32 = -1;
    let mut path = "";
    for i in 0..arg_count {
        let arg_i = i as usize;
        let mut arg_at_i = "";
        arg_at_i.clone_from(&arg_list[arg_i].as_str());
        if i == path_index {
            path = arg_at_i;
        }
        if arg_at_i == "--path" {
            path_index = i + 1;
        }
    }
    if path_index == -1 {
        panic!("--path not found");
    }
    path.to_owned()
}
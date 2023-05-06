pub fn extract_args(args: &[String]) -> (&str, &str, &str, &str){
    let io = &args[1];
    let query = &args[2];
    let filename = &args[3];
    let msg = &args[4];

    (io, query, filename, msg)
}
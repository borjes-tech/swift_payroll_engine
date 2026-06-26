fn main() {
    let re = regex::Regex::new(r"(^|[^.])\b(\d+)\b([^.]|$)").unwrap();
    let text = "gross <= 10000 ? 10.5 : 20000";
    let res = re.replace_all(text, "${1}${2}.0${3}");
    println!("{}", res);
}

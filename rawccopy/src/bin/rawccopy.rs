fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    rawccopy::exe(args.iter().map(|arg| arg.as_ref()).collect());
}

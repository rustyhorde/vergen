error_chain!{
    foreign_links {
        Io(::std::io::Error);
        Var(::std::env::VarError);
    }
}

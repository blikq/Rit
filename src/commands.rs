pub enum Git {
    Add, //"add"
    CatFile, //"cat-file"
    CheckIgnore, //"check-ignore"
    Checkout, //"checkout"
    Commit, //"commit"
    HashObject, //"hash-object"
    Init, //"init"
    Log, //"log"
    LsFiles, //"ls-files"
    LsTree, //"ls-tree"
    RevParse, //"rev-parse"
    Rm, //"rm"
    ShowRef, //"show-ref"
    Status, //"status"
    Tag, //"tag"
    Help, //"help"
    BadCommand, //
}

impl Git {
    pub fn new(args: &Vec<String>) -> Self{
        let command = match args.len() {
            0|1 => return Git::Help,
            _ => {
                match args.get(1).unwrap().as_str() {
                    "add" => Git::Add,
                    "cat-file" => Git::CatFile,
                    "check-ignore" => Git::CheckIgnore,
                    "checkout" => Git::Checkout,
                    "commit" => Git::Commit,
                    "hash-object" => Git::HashObject,
                    "init" => Git::Init,
                    "log" => Git::Log,
                    "ls-files" => Git::LsFiles,
                    "LsTree" => Git::LsTree,
                    "rev-parse" => Git::RevParse,
                    "rm" => Git::Rm,
                    "show-ref" => Git::ShowRef,
                    "ststus" => Git::Status,
                    "tag" => Git::Tag,
                    "help" => Git::Help,
                    _  => Git::BadCommand
                }
            }
        };
        command
    }
}
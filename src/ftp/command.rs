pub enum FtpCommand {
    User(String),
    Pass(String),
    Syst,
    Feat,
    Pwd,
    Type(String),
    Pasv,
    List
}
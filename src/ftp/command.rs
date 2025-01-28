/// Represents an FTP command.
#[derive(Clone)]
pub enum FtpCommand {
    /// The USER command is used to specify the user name for authentication.
    User(String),

    /// The PASS command is used to specify the password for authentication.
    Pass(String),

    /// The SYST command is used to return the system type.
    Syst,

    /// The FEAT command is used to list all new features supported by the server.
    Feat,

    /// The PWD command is used to print the current working directory.
    Pwd,

    /// The TYPE command is used to specify the type of file to be transferred.
    Type(String),

    /// The PASV command is used to request the server to enter passive mode.
    Pasv,

    /// The TYPE command is used to specify the type of file to be transferred.
    Epsv,

    /// The LIST command is used to list files in a directory.
    List,

    /// The CWD command is used to change the working directory.
    Cwd(String),

    /// The CDUP command is used to change to the parent directory.
    Cdup,
}

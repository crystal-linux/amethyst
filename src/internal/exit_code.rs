pub enum AppExitCode {
    RunAsRoot = 1,
    FailedCreatingPaths = 4,
    MissingDeps = 5,
    UserCancellation = 6,
    PacmanError = 7,
    GitError = 8,
    MakePkgError = 9,
    RpcError = 10,
    Other = 102,
}

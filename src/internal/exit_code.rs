pub enum AppExitCode {
    RunAsRoot = 1,
    FailedAddingPkg = 2,
    FailedInitDb = 3,
    FailedCreatingPaths = 4,
    MissingDeps = 5,
    UserCancellation = 6,
    PacmanError = 7,
    GitError = 8,
    MakePkgError = 9,
    Other = 102,
}

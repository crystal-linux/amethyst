#[allow(clippy::module_name_repetitions)]
pub enum AppExitCode {
    RunAsRoot = 1,
    FailedCreatingPaths = 2,
    MissingDeps = 3,
    UserCancellation = 4,
    PacmanError = 5,
    GitError = 6,
    MakePkgError = 7,
    Other = 63,
}

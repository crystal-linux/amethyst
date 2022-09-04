#[allow(clippy::module_name_repetitions)]
/// Defined exit codes for the program
pub enum AppExitCode {
    RunAsRoot = 1,
    FailedCreatingPaths = 2,
    MissingDeps = 3,
    UserCancellation = 4,
    PacmanError = 5,
    MakePkgError = 7,
    RpcError = 9,
    Other = 63,
}

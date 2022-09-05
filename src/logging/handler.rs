use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget};
use parking_lot::{Mutex, RwLock};
use std::{
    fmt::Display,
    io::{self, Write},
    mem,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use crate::{internal::utils::wrap_text, uwu};

use super::{get_logger, Verbosity};

const OK_SYMBOL: &str = "❖";
const ERR_SYMBOL: &str = "X";
const WARN_SYMBOL: &str = "!";
const DEBUG_SYMBOL: &str = "⌘";
const TRACE_SYMBOL: &str = "🗲";

pub struct LogHandler {
    level: Arc<RwLock<Verbosity>>,
    output_type: Arc<RwLock<OutputType>>,
    uwu_enabled: Arc<AtomicBool>,
}

impl Default for LogHandler {
    fn default() -> Self {
        Self {
            level: Arc::new(RwLock::new(Verbosity::Info)),
            output_type: Arc::new(RwLock::new(OutputType::Stderr)),
            uwu_enabled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[allow(unused)]
pub enum OutputType {
    Stdout,
    Stderr,
    MultiProgress(Arc<MultiProgress>),
    Progress(Arc<ProgressBar>),
    Buffer {
        buffer: Arc<Mutex<Vec<String>>>,
        suspended: Box<OutputType>,
    },
}

pub struct SuspendHandle;

impl LogHandler {
    pub fn log_error(&self, msg: String) {
        if self.is_loggable(Verbosity::Error) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", ERR_SYMBOL.red().bold(), msg.bold().red());
            self.log(msg);
        }
    }

    pub fn log_warning(&self, msg: String) {
        if self.is_loggable(Verbosity::Warning) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", WARN_SYMBOL.yellow(), msg.yellow().bold());
            self.log(msg);
        }
    }

    pub fn log_info(&self, msg: String) {
        if self.is_loggable(Verbosity::Info) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", OK_SYMBOL.purple(), msg.bold());
            self.log(msg);
        }
    }

    pub fn log_debug(&self, msg: String) {
        if self.is_loggable(Verbosity::Debug) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", DEBUG_SYMBOL.blue(), msg);

            self.log(msg);
        }
    }

    pub fn log_trace(&self, msg: String) {
        if self.is_loggable(Verbosity::Trace) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", TRACE_SYMBOL.cyan(), msg.dimmed());
            self.log(msg);
        }
    }

    pub fn print_list<I: IntoIterator<Item = T>, T: Display>(
        &self,
        list: I,
        separator: &str,
        padding: usize,
    ) {
        let lines = list
            .into_iter()
            .map(|l| self.preformat_msg(l.to_string()))
            .fold(String::new(), |acc, line| {
                format!("{}{}{}", acc, separator, line)
            });

        let lines = wrap_text(lines, padding)
            .join("\n")
            .trim_matches('\n')
            .to_string();
        self.log(lines)
    }

    pub fn print_newline(&self) {
        self.log(String::from(""))
    }

    pub fn set_verbosity(&self, level: Verbosity) {
        (*self.level.write()) = level;
    }

    pub fn reset_output_type(&self) {
        self.set_output_type(OutputType::Stdout);
    }

    #[must_use]
    pub fn suspend(&self) -> SuspendHandle {
        let mut output_type = self.output_type.write();
        let mut old_output_type = OutputType::Stdout;
        mem::swap(&mut *output_type, &mut old_output_type);

        (*output_type) = OutputType::Buffer {
            buffer: Arc::new(Mutex::new(Vec::new())),
            suspended: Box::new(old_output_type),
        };

        SuspendHandle
    }

    pub fn unsuspend(&self) {
        let mut buffered = Vec::new();
        {
            let mut output_type = self.output_type.write();
            let mut old_output_type = OutputType::Stdout;
            mem::swap(&mut *output_type, &mut old_output_type);

            if let OutputType::Buffer { buffer, suspended } = old_output_type {
                (*output_type) = *suspended;
                buffered = mem::take(&mut *buffer.lock());
            }
        }

        buffered.into_iter().for_each(|msg| self.log(msg));
    }

    /// Creates a new progress spinner and registers it on the log handler
    pub fn new_progress_spinner(&self) -> Arc<ProgressBar> {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(250));

        let mut output_type = self.output_type.write();

        if let OutputType::MultiProgress(mp) = &*output_type {
            Arc::new(mp.add(pb))
        } else {
            let pb = Arc::new(pb);
            *output_type = OutputType::Progress(pb.clone());

            pb
        }
    }

    pub fn new_multi_progress(&self) -> Arc<MultiProgress> {
        let mp = Arc::new(MultiProgress::new());
        self.set_output_type(OutputType::MultiProgress(mp.clone()));

        mp
    }

    /// Sets the output type of the log handler to either stdout/stderr or a progress bar
    pub fn set_output_type(&self, mut output: OutputType) {
        {
            let mut output_type = self.output_type.write();
            mem::swap(&mut *output_type, &mut output);
        }

        match &mut output {
            OutputType::MultiProgress(mp) => mp.set_draw_target(ProgressDrawTarget::hidden()),
            OutputType::Progress(p) => p.set_draw_target(ProgressDrawTarget::hidden()),
            OutputType::Buffer {
                buffer,
                suspended: _,
            } => {
                let buffered = mem::take(&mut *buffer.lock());
                buffered.into_iter().for_each(|c| self.log(c));
            }
            _ => {}
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn set_uwu_enabled(&self, enabled: bool) {
        self.uwu_enabled
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }

    pub(crate) fn is_loggable(&self, level: Verbosity) -> bool {
        (*self.level.read()) >= level
    }

    /// Flushes the output buffer
    pub fn flush(&self) {
        let output = self.output_type.read();
        match &*output {
            OutputType::Stdout => io::stdout().flush().unwrap(),
            OutputType::Stderr => io::stderr().flush().unwrap(),
            OutputType::Progress(p) => p.tick(),
            _ => {}
        }
    }

    fn preformat_msg(&self, msg: String) -> String {
        let msg = self.apply_uwu(msg);

        wrap_text(msg, 2).join("\n")
    }

    fn apply_uwu(&self, msg: String) -> String {
        if self.uwu_enabled.load(std::sync::atomic::Ordering::Relaxed) {
            uwu!(msg)
        } else {
            msg
        }
    }

    fn log(&self, msg: String) {
        let output_type = self.output_type.read();
        match &*output_type {
            OutputType::Stdout => println!("{}", msg),
            OutputType::Stderr => eprintln!("{}", msg),
            OutputType::MultiProgress(m) => {
                let _ = m.println(msg);
            }
            OutputType::Progress(p) => p.println(msg),
            OutputType::Buffer {
                buffer,
                suspended: _,
            } => buffer.lock().push(msg),
        };
    }
}

impl Drop for SuspendHandle {
    fn drop(&mut self) {
        get_logger().unsuspend();
    }
}

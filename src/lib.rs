use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

pub struct Sloughi {
    relative_path: String,
    include_precommit: bool,
    ignore_envs: Vec<String>,
}

impl Sloughi {
    pub fn new() -> Self {
        Self {
            relative_path: String::from(".sloughi"),
            include_precommit: true,
            ignore_envs: vec![
                String::from("HUSKY_SKIP_INIT"),
                String::from("SLOUGHI_SKIP_INIT"),
            ],
        }
    }

    pub fn custom_path(&mut self, relative_path: &str) -> &mut Self {
        self.relative_path = relative_path.to_string();
        self
    }

    pub fn ignore_env(&mut self, env: &str) -> &mut Self {
        self.ignore_envs.push(env.to_string());
        self
    }

    pub fn install(&self) -> io::Result<()> {
        let repo_path = self.repo_path()?;

        // setup git config core.hooksPath
        let exist_status = self.set_git_hook_path()?;
        if !exist_status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not setup git hook path!",
            ));
        }

        fs::create_dir_all(self._sloughi_script_abs_dir(repo_path.as_str()))?;
        fs::write(
            self._sloughi_script_abs_dir(repo_path.as_str()).join(".gitignore"),
            "*",
        )
        .map_err(|err| {
            println!("cargo:warning=Could not write to .gitignore file!\n");
            err
        })?;

        fs::write(
            self._hooks_abs_dir(repo_path.as_str()).join("pre-commit"),
            "just for trkk",
        )
        .map_err(|err| {
            println!("cargo:warning=Could not write to pre-commit file!\n");
            err
        })?;

        fs::write(
            self._sloughi_script_abs_dir(repo_path.as_str()).join("sloughi.sh"),
            r#"#!/usr/bin/env sh

if [ -z "$sloughi_skip_init" ]; then
  debug () {
    if [ "$SLOUGHI_DEBUG" = "1" ]; then
      echo "sloughi (debug) - $1"
    fi
  }

  readonly hook_name="$(basename "$0")"
  debug "starting $hook_name..."

  if [ "$SLOUGHI" = "0" ]; then
    debug "SLOUGHI env variable is set to 0, skipping hook"
    exit 0
  fi

  export readonly sloughi_skip_init=1
  sh -e "$0" "$@"
  exitCode="$?"

  if [ $exitCode != 0 ]; then
    echo "sloughi - $hook_name hook exited with code $exitCode (error)"
  fi

  exit $exitCode
fi

"#,
        )?;
        Ok(())
    }

    fn set_git_hook_path(&self) -> io::Result<ExitStatus> {
        Command::new("git")
            .arg("config")
            .arg("core.hooksPath")
            .arg(self.relative_path.as_str())
            .status()
            .map_err(|e| {
                println!("cargo:warning=Could not setup a custom git hook path!\n");
                e
            })
    }

    fn repo_path(&self) -> io::Result<String> {
        let git_cmd = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output();

        let output = git_cmd.map_err(|e| {
            println!("cargo:warning=Could not find a git repository.\n");
            e
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string()) // cleanup stdout from newlines.
    }

    fn _hooks_abs_dir(&self, repo_path: &str) -> PathBuf {
        Path::new(&repo_path).join(self.relative_path.as_str())
    }

    fn _sloughi_script_abs_dir(&self, repo_path: &str) -> PathBuf {
        self._hooks_abs_dir(repo_path).join("_")
    }
}

//
// Copyright (c) 2024 Hemi Labs, Inc.
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

pub mod config;
pub mod prerequisite;
pub mod recipe;
pub mod target;

use std::{
    collections::{BTreeMap, HashMap},
    env,
    fs::{File, FileTimes},
    process::{self, Command},
    sync::{Arc, LazyLock, Mutex},
    time::SystemTime,
};

use crate::{
    config::Config as GlobalConfig,
    error_code::ErrorCode::{self, *},
    parser::{Rule as ParsedRule, VariableDefinition},
    signal_handler, DEFAULT_SHELL, DEFAULT_SHELL_VAR,
};

use config::Config;
use prerequisite::Prerequisite;
use recipe::config::Config as RecipeConfig;
use recipe::Recipe;
use target::Target;

pub static INTERRUPT_FLAG: LazyLock<Arc<Mutex<Option<(String, bool)>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    /// The targets of the rule
    targets: Vec<Target>,
    /// The prerequisites of the rule
    prerequisites: Vec<Prerequisite>,
    /// The recipe of the rule
    recipes: Vec<Recipe>,

    pub config: Config,
}

impl Rule {
    pub fn targets(&self) -> impl Iterator<Item = &Target> {
        self.targets.iter()
    }

    pub fn prerequisites(&self) -> impl Iterator<Item = &Prerequisite> {
        self.prerequisites.iter()
    }

    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.iter()
    }

    /// Runs the rule with the global config and macros passed in.
    ///
    /// Returns `Ok` on success and `Err` on any errors while running the rule.
    pub fn run(
        &self,
        global_config: &GlobalConfig,
        macros: &[VariableDefinition],
        target: &Target,
        up_to_date: bool,
    ) -> Result<(), ErrorCode> {
        let GlobalConfig {
            ignore: global_ignore,
            dry_run: global_dry_run,
            silent: global_silent,
            touch: global_touch,
            env_macros: global_env_macros,
            quit: global_quit,
            rules: ref global_rules,
            clear: global_clear,
            print: global_print,
            keep_going: global_keep_going,
            terminate: global_terminate,
            precious: global_precious,
        } = *global_config;
        let Config {
            ignore: rule_ignore,
            silent: rule_silent,
            phony: rule_phony,
            precious: rule_precious,
        } = self.config;

        for recipe in self.recipes() {
            let RecipeConfig {
                ignore: recipe_ignore,
                silent: recipe_silent,
                force_run: recipe_force_run,
            } = recipe.config;

            let ignore = global_ignore || rule_ignore || recipe_ignore;
            let dry_run = global_dry_run;
            let silent = global_silent || rule_silent || recipe_silent;
            let force_run = recipe_force_run;
            let touch = global_touch;
            let env_macros = global_env_macros;
            let quit = global_quit;
            let clear = global_clear;
            let print = global_print;
            let phony = rule_phony;
            let precious = global_precious || rule_precious;
            let keep_going = global_keep_going;
            let terminate = global_terminate;
            // Note: this feature can be implemented only with parser rewrite
            // Todo: parse all suffixes and return error if rules don't include them
            // -r flag
            let rules = if clear {
                BTreeMap::new()
            } else {
                global_rules.clone()
            };

            *INTERRUPT_FLAG.lock().unwrap() = Some((target.as_ref().to_string(), precious));

            if !ignore || print || quit || dry_run {
                signal_handler::register_signals();
            }

            // Todo: somehow catch parse and print changed default_rules

            if !force_run {
                // -n flag
                if dry_run {
                    println!("{}", recipe);
                    continue;
                }

                // -t flag
                if touch {
                    continue;
                }
                // -q flag
                if quit {
                    if up_to_date {
                        process::exit(0);
                    } else {
                        process::exit(1);
                    }
                }
            }

            // -s flag
            if !silent {
                println!("{}", recipe);
            }
            
            let mut command = Command::new(
                env::var(DEFAULT_SHELL_VAR)
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(DEFAULT_SHELL),
            );

            self.init_env(env_macros, &mut command, macros);

            self.substitute_internal_macros(target, recipe);
            
            command.args(["-c", recipe.as_ref()]);

            let status = match command.status() {
                Ok(status) => status,
                Err(err) => {
                    if ignore {
                        continue;
                    } else {
                        return Err(IoError(err.kind()));
                    }
                }
            };
            if !status.success() && !ignore {
                // -S and -k flags
                if !terminate && keep_going {
                    eprintln!(
                        "make: {:?}",
                        ExecutionError {
                            exit_code: status.code(),
                        }
                    );
                    break;
                } else {
                    return Err(ExecutionError {
                        exit_code: status.code(),
                    });
                }
            }
        }

        let silent = global_silent || rule_silent;
        let touch = global_touch;

        // -t flag
        if touch {
            if !silent {
                println!("touch {target}");
            }
            let file = File::create(target.as_ref())?;
            file.set_times(FileTimes::new().set_modified(SystemTime::now()))?;
            return Ok(());
        }

        Ok(())
    }
    
    fn substitute_internal_macros(&self, target: &Target, recipe: &Recipe) -> Recipe {
        let recipe = recipe.inner();
        let mut stream = recipe.chars();
        let mut result = String::new();
        
        while let Some(ch) = stream.next() {
            if ch != '$' { result.push(ch); continue }
            
            // TODO: Remove panics
            match stream.next() {
                Some('@') => {
                    result.push_str(&target
                        .to_string()
                        .split('(')
                        .next()
                        .expect("Target must have lib part")
                        .to_string())
                }
                Some('%') => {
                    let body = target
                        .to_string()
                        .split('(')
                        .skip(1)
                        .next()
                        .expect("Target must have lib part")
                        .to_string();
                    
                    result.push_str(body.strip_suffix(')').unwrap_or(&body))
                }
                Some('?') => { todo!("Pass rule prerequisites here") }
                Some('$') => { result.push('$') }
                Some('<' | '*') => { todo!("Implement when reference rules are ready") }
                Some(_) => { break }
                None => { panic!("Unexpected `$` at the end of the rule!") }
            }
        }

        todo!()
    }

    /// A helper function to initialize env vars for shell commands.
    fn init_env(&self, env_macros: bool, command: &mut Command, variables: &[VariableDefinition]) {
        let mut macros: HashMap<String, String> = variables
            .iter()
            .map(|v| {
                (
                    v.name().unwrap_or_default(),
                    v.raw_value().unwrap_or_default(),
                )
            })
            .collect();

        if env_macros {
            let env_vars: HashMap<String, String> = std::env::vars().collect();
            macros.extend(env_vars);
        }
        command.envs(macros);
    }
}

impl From<ParsedRule> for Rule {
    fn from(parsed: ParsedRule) -> Self {
        let config = Config::default();
        Self::from((parsed, config))
    }
}

impl From<(ParsedRule, Config)> for Rule {
    fn from((parsed, config): (ParsedRule, Config)) -> Self {
        let targets = parsed.targets().map(Target::new).collect();
        let prerequisites = parsed.prerequisites().map(Prerequisite::new).collect();
        let recipes = parsed.recipes().map(Recipe::new).collect();
        Rule {
            targets,
            prerequisites,
            recipes,
            config,
        }
    }
}

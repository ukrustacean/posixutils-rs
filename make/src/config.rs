//
// Copyright (c) 2024 Hemi Labs, Inc.
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

use std::collections::{HashMap, HashSet};

/// Represents the configuration of the make utility
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Whether to ignore the errors in the rule
    pub ignore: bool,
    /// Whether to execute commands or print to stdout
    pub dry_run: bool,
    /// Whether to print recipe lines
    pub silent: bool,
    /// Whether to touch targets on execution
    pub touch: bool,
    /// Whether to replace macros within makefiles with envs
    pub env_macros: bool,
    /// Whether to quit without build
    pub quit: bool,
    /// Whether to clear default_rules
    pub clear: bool,
    /// Whether to print macro definitions and target descriptions.
    pub print: bool,

    pub default_rules: HashMap<String, HashMap<String, String>>,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Self {
            ignore: false,
            dry_run: false,
            silent: false,
            touch: false,
            env_macros: false,
            quit: false,
            clear: false,
            print: false,
            default_rules: HashMap::from([(
                String::from(".SUFFIXES"),
                HashMap::from([
                    (String::from(".o"), String::from("")),
                    (String::from(".c"), String::from("")),
                    (String::from(".y"), String::from("")),
                    (String::from(".l"), String::from("")),
                    (String::from(".a"), String::from("")),
                    (String::from(".sh"), String::from("")),
                    (String::from(".c~"), String::from("")),
                    (String::from(".y~"), String::from("")),
                    (String::from(".l~"), String::from("")),
                    (String::from(".sh~"), String::from("")),
                ]),
            )]),
        }
    }
}

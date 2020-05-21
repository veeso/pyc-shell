//! ## Cache
//!
//! `Cache` is the module which provides functionality to cache prompt values and speed up the modules

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "Pyc"
*
*   Pyc is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   Pyc is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with Pyc.  If not, see <http://www.gnu.org/licenses/>.
*
*/

extern crate git2;

use git2::Repository;

pub struct PromptCache {
    git_cache: Option<Repository>,
}

impl PromptCache {
    /// ### new
    ///
    /// Instantiate a new Prompt cache object
    pub fn new() -> PromptCache {
        PromptCache { git_cache: None }
    }

    /// ### invalidate
    ///
    /// Invalidate cache
    pub fn invalidate(&mut self) {
        self.git_cache = None
    }

    /// ### cache_git
    ///
    /// Cache git repository
    pub fn cache_git(&mut self, git_repo: Repository) {
        self.git_cache = Some(git_repo);
    }

    /// ### get_git
    ///
    /// Get git repository
    pub fn get_cached_git(&self) -> Option<&Repository> {
        match self.git_cache.as_ref() {
            Some(g) => Some(&g),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cache() {
        //Create temp directory
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let git_repo: Repository = Repository::init(tmpdir.path()).unwrap();
        let mut cache: PromptCache = PromptCache::new();
        //Cache repository
        cache.cache_git(git_repo);
        //Verify git cache is Some
        assert!(cache.get_cached_git().is_some());
        //Invalidate cache
        cache.invalidate();
        //Verify git is None
        assert!(cache.get_cached_git().is_none());
    }
}

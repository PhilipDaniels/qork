use std::borrow::{Cow, Borrow};
use std::env;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};
use shellexpand::full_with_context_no_errors;

/// Expands a leading tilde and environment variables in the `input` string.
pub fn expand_variables<S: AsRef<str> + ?Sized>(input: &S) -> Cow<str> {
    let get_env_var = |input: &str| { env::var(input).ok() };
    full_with_context_no_errors(input, env::home_dir, get_env_var)
}

/// There is a problem: canonicalize does not do what we want. It only
/// works if the file actually exists! Look at the source code and see
/// https://blogs.msdn.microsoft.com/jeremykuhne/2016/04/21/path-normalization/
/// We could implement our own algorithm based on those guidelines, but I can't
/// be bothered right now.


/// Expands a filename by first expanding environment variables and then
/// canonicalizing it.
pub fn expand_filename<S: AsRef<Path> + ?Sized>(input: &S) -> String {
    let t1 = input.as_ref().to_string_lossy().into_owned();
    let t2 = expand_variables(&t1).into_owned();
    t2
}

pub fn expand_filename2<S: AsRef<str> + ?Sized>(input: &S) -> PathBuf {
    //let t1 = input.as_ref().to_string_lossy().into_owned();
    let t2 = expand_variables(input).into_owned();
    let t2 = canonicalize(t2);
    t2.unwrap()
}

// Note that cargo runs tests in parallel, so there is a danger of tests interfering with each
// other: for this reason, we either always set the environment variable to the same value in all
// tests, or we use discrete environment variables.
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const HOME_VAL: &'static str = "homes";
    const FOO_VAL: &'static str = "foos";
    const THUNK_VAL: &'static str = "thunks";

    #[test]
    fn expand_variables_expands_tilde_if_home_is_set() {
        env::set_var("HOME", HOME_VAL);
        let result = expand_variables("~/Documents");
        assert_eq!(result, format!("{}/Documents", HOME_VAL));
    }

    // Actually this doesn't work, because home_dir() will invoke getpwuid_r
    // and still find my home, on Linux at least.
    // #[test]
    // fn expand_variables_does_not_expand_tilde_if_home_is_not_set() {
    //     env::remove_var("HOME");
    //     let result = expand_variables("~/Documents");
    //     assert_eq!(result, "~/Documents");
    // }

    #[test]
    fn expand_variables_only_expands_tilde_at_the_beginning_of_the_input() {
        env::set_var("HOME", HOME_VAL);
        let result = expand_variables("~/Documents/foo/~/bar/aa~");
        assert_eq!(result, format!("{}/Documents/foo/~/bar/aa~", HOME_VAL));
    }

    #[test]
    fn expand_variables_expands_mutiple_occurrences_of_known_variables() {
        env::set_var("FOO", FOO_VAL);
        env::set_var("THUNK", THUNK_VAL);
        let result = expand_variables("/Documents/$FOO/bar/$FOO/baz/${FOO}/wipple${THUNK}");
        assert_eq!(result, format!("/Documents/{0}/bar/{0}/baz/{0}/wipple{1}", FOO_VAL, THUNK_VAL));
    }

    #[test]
    fn expand_variables_does_not_expand_variables_if_case_differs() {
        env::set_var("FOO", FOO_VAL);
        let result = expand_variables("/Documents/$Foo");
        assert_eq!(result, "/Documents/$Foo");
    }

    #[test]
    fn expand_variables_only_expands_one_level_deep() {
        env::set_var("FEZ", "$FEZ/$FEZ/$BAR");
        env::set_var("BAR", "should not be used");
        let result = expand_variables("/Documents/$FEZ");
        assert_eq!(result, "/Documents/$FEZ/$FEZ/$BAR");
    }

    #[test]
    fn expand_variables_does_not_expand_unknown_variables() {
        env::remove_var("NO_CHANCE_OF_ME_REALLY_EXISTING");
        let result = expand_variables("/Documents/$NO_CHANCE_OF_ME_REALLY_EXISTING");
        assert_eq!(result, "/Documents/$NO_CHANCE_OF_ME_REALLY_EXISTING");
    }

    #[test]
    fn expand_filename_expands_and_canonicalizes() {
        env::set_var("HOME", HOME_VAL);
        let result = expand_filename2("~/Documents/foo/..///a.txt");
        assert_eq!(result, PathBuf::from(format!("{}/Documents/a.txt", HOME_VAL)));
    }
}
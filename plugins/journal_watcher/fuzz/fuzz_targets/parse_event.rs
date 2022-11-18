#![no_main]
use journal_watcher::Rule;
use pest::Parser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    _ = journal_watcher::SSHDParser::parse(Rule::event, data);
});

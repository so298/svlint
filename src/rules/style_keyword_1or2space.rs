use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeyword1Or2Space {
    re_split: Option<Regex>,
    re_kw: Option<Regex>,
    re_succ: Option<Regex>,
}

impl Rule for StyleKeyword1Or2Space {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        /*
        re_split extracts keyword from anything following it.
        re_kw is used to selectively apply this rule to specific keywords.
        re_succ matches what is allowed after the keyword.
            - exactly 1space
            - exactly 2space
        */
        if self.re_split.is_none() {
            self.re_split = Some(Regex::new(r"(?P<kw>[a-z_01]+)(?P<succ>(?s:.)*)").unwrap());
        }
        if self.re_kw.is_none() {
            let keywords =
                [ "inout" // {{{
                , "input"
                ].join("|"); // }}}

            self.re_kw = Some(Regex::new(format!("^({})$", keywords).as_str()).unwrap());
        }
        if self.re_succ.is_none() {
            self.re_succ = Some(Regex::new(r"^ [ ]?$").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::Keyword(x) => {
                let re_split = self.re_split.as_ref().unwrap();
                let re_kw = self.re_kw.as_ref().unwrap();
                let t = syntax_tree.get_str(*x).unwrap();
                let caps = re_split.captures(&t).unwrap();

                if re_kw.is_match(&caps[1]) {
                    let re_succ = self.re_succ.as_ref().unwrap();

                    if re_succ.is_match(&caps[2]) {
                        RuleResult::Pass
                    } else {
                        RuleResult::Fail
                    }
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_keyword_1or2space")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Follow keyword with exactly 1 or 2 spaces.")
    }

    fn reason(&self) -> String {
        String::from("Consistent use of whitespace enhances readability by reducing visual noise.")
    }
}

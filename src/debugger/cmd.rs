use super::ast::Command;
use std::usize;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__cmd {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use super::super::ast::Command;
    use std::usize;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22b_22(&'input str),
        Term_22bp_22(&'input str),
        Term_22break_22(&'input str),
        Term_22breakpoints_22(&'input str),
        Term_22c_22(&'input str),
        Term_22continue_22(&'input str),
        Term_22d_22(&'input str),
        Term_22debug_22(&'input str),
        Term_22exit_22(&'input str),
        Term_22p_22(&'input str),
        Term_22print_22(&'input str),
        Term_22q_22(&'input str),
        Term_22quit_22(&'input str),
        Term_22s_22(&'input str),
        Term_22set_22(&'input str),
        Term_22step_22(&'input str),
        Termr_23_22_20_2b_22_23(&'input str),
        Termr_23_22_5bA_2dZa_2dz0_2d9_5d_2b_22_23(&'input str),
        NtBreakpoint(Command),
        NtBreakpoints(Command),
        NtContinue(Command),
        NtDebug(Command),
        NtExit(Command),
        NtPrint(Command),
        NtSet(Command),
        NtStep(Command),
        Nt____cmd(Command),
        Ntalphanumeric(&'input str),
        Ntcmd(Command),
        Nthex(usize),
        Nthex_3f(::std::option::Option<usize>),
        Ntspace(()),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        11, // on "b", goto 10
        12, // on "bp", goto 11
        13, // on "break", goto 12
        14, // on "breakpoints", goto 13
        15, // on "c", goto 14
        16, // on "continue", goto 15
        17, // on "d", goto 16
        18, // on "debug", goto 17
        19, // on "exit", goto 18
        20, // on "p", goto 19
        21, // on "print", goto 20
        22, // on "q", goto 21
        23, // on "quit", goto 22
        24, // on "s", goto 23
        25, // on "set", goto 24
        26, // on "step", goto 25
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 1
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 2
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 3
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 4
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 5
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 6
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 7
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 8
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 9
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 10
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        29, // on r#"[A-Za-z0-9]+"#, goto 28
        // State 11
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 12
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        29, // on r#"[A-Za-z0-9]+"#, goto 28
        // State 13
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 14
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 15
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 16
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 17
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 18
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 19
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        29, // on r#"[A-Za-z0-9]+"#, goto 28
        // State 20
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        29, // on r#"[A-Za-z0-9]+"#, goto 28
        // State 21
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 22
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 23
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        29, // on r#"[A-Za-z0-9]+"#, goto 28
        // State 24
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        35, // on r#"[A-Za-z0-9]+"#, goto 34
        // State 25
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        29, // on r#"[A-Za-z0-9]+"#, goto 28
        // State 26
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 27
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 28
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 29
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 30
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 31
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 32
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 33
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        29, // on r#"[A-Za-z0-9]+"#, goto 28
        // State 34
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        -20, // on r#"[A-Za-z0-9]+"#, reduce `alphanumeric = r#"[A-Za-z0-9]+"# => ActionFn(26);`
        // State 35
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
        // State 36
        0, // on "b", error
        0, // on "bp", error
        0, // on "break", error
        0, // on "breakpoints", error
        0, // on "c", error
        0, // on "continue", error
        0, // on "d", error
        0, // on "debug", error
        0, // on "exit", error
        0, // on "p", error
        0, // on "print", error
        0, // on "q", error
        0, // on "quit", error
        0, // on "s", error
        0, // on "set", error
        0, // on "step", error
        0, // on r#" +"#, error
        0, // on r#"[A-Za-z0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -24, // on EOF, reduce `cmd = Breakpoint => ActionFn(4);`
        -25, // on EOF, reduce `cmd = Breakpoints => ActionFn(5);`
        -21, // on EOF, reduce `cmd = Continue => ActionFn(1);`
        -22, // on EOF, reduce `cmd = Debug => ActionFn(2);`
        -23, // on EOF, reduce `cmd = Exit => ActionFn(3);`
        -26, // on EOF, reduce `cmd = Print => ActionFn(6);`
        -27, // on EOF, reduce `cmd = Set => ActionFn(7);`
        -28, // on EOF, reduce `cmd = Step => ActionFn(8);`
        -19, // on EOF, reduce `__cmd = cmd => ActionFn(0);`
        0, // on EOF, error
        -3, // on EOF, reduce `Breakpoints = "bp" => ActionFn(11);`
        0, // on EOF, error
        -4, // on EOF, reduce `Breakpoints = "breakpoints" => ActionFn(12);`
        -5, // on EOF, reduce `Continue = "c" => ActionFn(13);`
        -6, // on EOF, reduce `Continue = "continue" => ActionFn(14);`
        -7, // on EOF, reduce `Debug = "d" => ActionFn(15);`
        -8, // on EOF, reduce `Debug = "debug" => ActionFn(16);`
        -11, // on EOF, reduce `Exit = "exit" => ActionFn(21);`
        0, // on EOF, error
        0, // on EOF, error
        -9, // on EOF, reduce `Exit = "q" => ActionFn(19);`
        -10, // on EOF, reduce `Exit = "quit" => ActionFn(20);`
        -16, // on EOF, reduce `Step = "s" => ActionFn(31);`
        0, // on EOF, error
        -18, // on EOF, reduce `Step = "step" => ActionFn(33);`
        -29, // on EOF, reduce `hex = alphanumeric => ActionFn(27);`
        -1, // on EOF, reduce `Breakpoint = "b", hex => ActionFn(9);`
        -20, // on EOF, reduce `alphanumeric = r#"[A-Za-z0-9]+"# => ActionFn(26);`
        -2, // on EOF, reduce `Breakpoint = "break", hex => ActionFn(10);`
        -12, // on EOF, reduce `Print = "p", hex => ActionFn(17);`
        -13, // on EOF, reduce `Print = "print", hex => ActionFn(18);`
        -15, // on EOF, reduce `Step = "s", hex => ActionFn(30);`
        0, // on EOF, error
        0, // on EOF, error
        -17, // on EOF, reduce `Step = "step", hex => ActionFn(32);`
        -14, // on EOF, reduce `Set = "set", alphanumeric, hex => ActionFn(22);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Breakpoint, goto 1
        3, // on Breakpoints, goto 2
        4, // on Continue, goto 3
        5, // on Debug, goto 4
        6, // on Exit, goto 5
        7, // on Print, goto 6
        8, // on Set, goto 7
        9, // on Step, goto 8
        0, // on __cmd, error
        0, // on alphanumeric, error
        10, // on cmd, goto 9
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 1
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 2
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 3
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 4
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 5
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 6
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 7
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 8
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 9
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 10
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        27, // on alphanumeric, goto 26
        0, // on cmd, error
        28, // on hex, goto 27
        0, // on hex?, error
        0, // on space, error
        // State 11
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 12
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        27, // on alphanumeric, goto 26
        0, // on cmd, error
        30, // on hex, goto 29
        0, // on hex?, error
        0, // on space, error
        // State 13
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 14
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 15
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 16
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 17
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 18
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 19
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        27, // on alphanumeric, goto 26
        0, // on cmd, error
        31, // on hex, goto 30
        0, // on hex?, error
        0, // on space, error
        // State 20
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        27, // on alphanumeric, goto 26
        0, // on cmd, error
        32, // on hex, goto 31
        0, // on hex?, error
        0, // on space, error
        // State 21
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 22
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 23
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        27, // on alphanumeric, goto 26
        0, // on cmd, error
        33, // on hex, goto 32
        0, // on hex?, error
        0, // on space, error
        // State 24
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        34, // on alphanumeric, goto 33
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 25
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        27, // on alphanumeric, goto 26
        0, // on cmd, error
        36, // on hex, goto 35
        0, // on hex?, error
        0, // on space, error
        // State 26
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 27
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 28
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 29
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 30
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 31
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 32
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 33
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        27, // on alphanumeric, goto 26
        0, // on cmd, error
        37, // on hex, goto 36
        0, // on hex?, error
        0, // on space, error
        // State 34
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 35
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
        // State 36
        0, // on Breakpoint, error
        0, // on Breakpoints, error
        0, // on Continue, error
        0, // on Debug, error
        0, // on Exit, error
        0, // on Print, error
        0, // on Set, error
        0, // on Step, error
        0, // on __cmd, error
        0, // on alphanumeric, error
        0, // on cmd, error
        0, // on hex, error
        0, // on hex?, error
        0, // on space, error
    ];
    pub fn parse_cmd<
        'input,
    >(
        input: &'input str,
    ) -> Result<Command, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                (_, (8, _), _) if true => 8,
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                (_, (12, _), _) if true => 12,
                (_, (13, _), _) if true => 13,
                (_, (14, _), _) if true => 14,
                (_, (15, _), _) if true => 15,
                (_, (16, _), _) if true => 16,
                (_, (17, _), _) if true => 17,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 18 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22b_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22bp_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22break_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22breakpoints_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22continue_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22d_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22debug_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22exit_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22p_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22print_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22q_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22quit_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22s_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22set_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22step_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Termr_23_22_20_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Termr_23_22_5bA_2dZa_2dz0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Command,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Breakpoint = "b", hex => ActionFn(9);
                let __sym1 = __pop_Nthex(__symbols);
                let __sym0 = __pop_Term_22b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBreakpoint(__nt), __end));
                0
            }
            2 => {
                // Breakpoint = "break", hex => ActionFn(10);
                let __sym1 = __pop_Nthex(__symbols);
                let __sym0 = __pop_Term_22break_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBreakpoint(__nt), __end));
                0
            }
            3 => {
                // Breakpoints = "bp" => ActionFn(11);
                let __sym0 = __pop_Term_22bp_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBreakpoints(__nt), __end));
                1
            }
            4 => {
                // Breakpoints = "breakpoints" => ActionFn(12);
                let __sym0 = __pop_Term_22breakpoints_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBreakpoints(__nt), __end));
                1
            }
            5 => {
                // Continue = "c" => ActionFn(13);
                let __sym0 = __pop_Term_22c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtContinue(__nt), __end));
                2
            }
            6 => {
                // Continue = "continue" => ActionFn(14);
                let __sym0 = __pop_Term_22continue_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtContinue(__nt), __end));
                2
            }
            7 => {
                // Debug = "d" => ActionFn(15);
                let __sym0 = __pop_Term_22d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDebug(__nt), __end));
                3
            }
            8 => {
                // Debug = "debug" => ActionFn(16);
                let __sym0 = __pop_Term_22debug_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDebug(__nt), __end));
                3
            }
            9 => {
                // Exit = "q" => ActionFn(19);
                let __sym0 = __pop_Term_22q_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExit(__nt), __end));
                4
            }
            10 => {
                // Exit = "quit" => ActionFn(20);
                let __sym0 = __pop_Term_22quit_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExit(__nt), __end));
                4
            }
            11 => {
                // Exit = "exit" => ActionFn(21);
                let __sym0 = __pop_Term_22exit_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExit(__nt), __end));
                4
            }
            12 => {
                // Print = "p", hex => ActionFn(17);
                let __sym1 = __pop_Nthex(__symbols);
                let __sym0 = __pop_Term_22p_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPrint(__nt), __end));
                5
            }
            13 => {
                // Print = "print", hex => ActionFn(18);
                let __sym1 = __pop_Nthex(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPrint(__nt), __end));
                5
            }
            14 => {
                // Set = "set", alphanumeric, hex => ActionFn(22);
                let __sym2 = __pop_Nthex(__symbols);
                let __sym1 = __pop_Ntalphanumeric(__symbols);
                let __sym0 = __pop_Term_22set_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSet(__nt), __end));
                6
            }
            15 => {
                // Step = "s", hex => ActionFn(30);
                let __sym1 = __pop_Nthex(__symbols);
                let __sym0 = __pop_Term_22s_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStep(__nt), __end));
                7
            }
            16 => {
                // Step = "s" => ActionFn(31);
                let __sym0 = __pop_Term_22s_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStep(__nt), __end));
                7
            }
            17 => {
                // Step = "step", hex => ActionFn(32);
                let __sym1 = __pop_Nthex(__symbols);
                let __sym0 = __pop_Term_22step_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action32(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStep(__nt), __end));
                7
            }
            18 => {
                // Step = "step" => ActionFn(33);
                let __sym0 = __pop_Term_22step_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStep(__nt), __end));
                7
            }
            19 => {
                // __cmd = cmd => ActionFn(0);
                let __sym0 = __pop_Ntcmd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            20 => {
                // alphanumeric = r#"[A-Za-z0-9]+"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5bA_2dZa_2dz0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntalphanumeric(__nt), __end));
                9
            }
            21 => {
                // cmd = Continue => ActionFn(1);
                let __sym0 = __pop_NtContinue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            22 => {
                // cmd = Debug => ActionFn(2);
                let __sym0 = __pop_NtDebug(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            23 => {
                // cmd = Exit => ActionFn(3);
                let __sym0 = __pop_NtExit(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            24 => {
                // cmd = Breakpoint => ActionFn(4);
                let __sym0 = __pop_NtBreakpoint(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            25 => {
                // cmd = Breakpoints => ActionFn(5);
                let __sym0 = __pop_NtBreakpoints(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            26 => {
                // cmd = Print => ActionFn(6);
                let __sym0 = __pop_NtPrint(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            27 => {
                // cmd = Set => ActionFn(7);
                let __sym0 = __pop_NtSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            28 => {
                // cmd = Step => ActionFn(8);
                let __sym0 = __pop_NtStep(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcmd(__nt), __end));
                10
            }
            29 => {
                // hex = alphanumeric => ActionFn(27);
                let __sym0 = __pop_Ntalphanumeric(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nthex(__nt), __end));
                11
            }
            30 => {
                // hex? = hex => ActionFn(28);
                let __sym0 = __pop_Nthex(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nthex_3f(__nt), __end));
                12
            }
            31 => {
                // hex? =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nthex_3f(__nt), __end));
                12
            }
            32 => {
                // space = r#" +"# => ActionFn(25);
                let __sym0 = __pop_Termr_23_22_20_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntspace(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bp_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bp_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22break_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22break_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22breakpoints_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22breakpoints_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22continue_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22continue_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22debug_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22debug_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22exit_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22exit_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22p_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22p_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22q_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22q_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22quit_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22quit_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22s_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22s_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22set_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22set_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22step_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22step_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_20_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_20_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dZa_2dz0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dZa_2dz0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBreakpoint<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBreakpoint(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBreakpoints<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBreakpoints(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtContinue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtContinue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDebug<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDebug(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExit<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExit(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPrint<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPrint(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStep<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStep(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____cmd<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____cmd(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntalphanumeric<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntalphanumeric(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcmd<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcmd(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nthex<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nthex(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nthex_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<usize>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nthex_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntspace<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntspace(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__cmd::parse_cmd;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        102 ... 111 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        113 => /* 'q' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        113 => /* 'q' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 119 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        120 => /* 'x' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        121 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        102 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        99 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 106 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        107 => /* 'k' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        108 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 102 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        104 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 43;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 44;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 45;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 47;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 49;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
) -> Command
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, usize, usize),
) -> Command
{
    Command::Breakpoint(v)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, usize, usize),
) -> Command
{
    Command::Breakpoint(v)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Breakpoints
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Breakpoints
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Continue
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Continue
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Debug
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Debug
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, usize, usize),
) -> Command
{
    Command::Print(v)
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, usize, usize),
) -> Command
{
    Command::Print(v)
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Exit
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Exit
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Command
{
    Command::Exit
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, k, _): (usize, &'input str, usize),
    (_, v, _): (usize, usize, usize),
) -> Command
{
    Command::Set(k.to_owned(), v)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, ::std::option::Option<usize>, usize),
) -> Command
{
    Command::Step(v)
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, ::std::option::Option<usize>, usize),
) -> Command
{
    Command::Step(v)
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
) -> usize
{
    {
    let mut v = v;
    if v.starts_with("0x") {
      v = &v[2..];
    }
    usize::from_str_radix(v, 16).unwrap()
  }
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> ::std::option::Option<usize>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<usize>
{
    None
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Command
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Command
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Command
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Command
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}

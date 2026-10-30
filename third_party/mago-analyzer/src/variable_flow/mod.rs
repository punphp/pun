//! PHPStan cb3b7e99489b2e3789ee2eec10501cb7a72580ea's local value-flow contract.
//!
//! Fragments are published by the successful semantic walk, not by a second variable-name walk.
//! A write is a source identity, not a variable name. Reads can observe a write without using
//! its value at a sink (for example `$dead = $capture`). Closures import writes; reference
//! captures import aliases instead. Constructors deliberately do not consume this producer.

mod builder;
mod resolver;

use std::rc::Rc;

use foldhash::{HashMap, HashSet};
use mago_span::Span;
use mago_word::Word;

use crate::artifacts::VariableThrowKind;

pub(crate) use builder::{expression, reference_argument, statement};

/// A stable write identity: the exact assignment target or imported variable node.
#[derive(Clone, Debug)]
pub struct VariableWrite {
    pub node: Span,
    pub name: Word,
    pub offset: Option<Offset>,
    pub offset_write: bool,
    pub replaces_offset: bool,
    pub parent: Option<Span>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Offset {
    Integer(i64),
    String(Vec<u8>),
}

/// The rule-facing body result, corresponding to PHPStan's VariableWritesNode.
#[derive(Clone, Debug, Default)]
pub struct VariableWritesNode {
    pub writes: HashMap<Span, VariableWrite>,
    pub read: HashSet<Span>,
    pub used: HashSet<Span>,
    pub covered: HashSet<Span>,
    pub referenced: HashSet<Word>,
    pub escaped: HashSet<Word>,
    pub all_names: bool,
    pub opaque: bool,
}

impl VariableWritesNode {
    pub fn is_variable_referenced(&self, name: Word) -> bool {
        self.all_names || self.opaque || self.referenced.contains(&name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ThrowPoint(pub Span, pub VariableThrowKind, pub Word);

type Flow = Rc<VariableFlow>;

#[derive(Clone, Debug)]
enum VariableFlow {
    Empty,
    Sequence(Vec<Flow>),
    Choice(Vec<Flow>),
    Read {
        name: Word,
        target: Option<Span>,
        container: bool,
        offset: Option<Offset>,
    },
    Write(VariableWrite),
    Define(VariableWrite),
    Discard(VariableWrite),
    Inputs {
        write: Span,
        target: Option<Span>,
    },
    Escape(Word),
    Mention(Word),
    ReadAll,
    MentionAll,
    Opaque,
    Return(Option<Word>),
    Break(usize),
    Continue(usize),
    Stop,
    Throw {
        point: ThrowPoint,
        can_continue: bool,
    },
    Arrow {
        body: Flow,
        outputs: Flow,
        parameters: Vec<Word>,
    },
    Loop {
        condition: Flow,
        body: Flow,
        update: Flow,
        at_least_once: bool,
    },
    Try {
        body: Flow,
        catches: Vec<(HashSet<ThrowPoint>, Flow)>,
        finally: Flow,
    },
    Switch {
        condition: Flow,
        cases: Vec<(Flow, Flow, bool)>,
        exhaustive: bool,
    },
}

impl VariableFlow {
    fn sequence(flows: impl IntoIterator<Item = Flow>) -> Flow {
        let flows: Vec<_> = flows
            .into_iter()
            .filter(|flow| !matches!(**flow, Self::Empty))
            .collect();
        match flows.len() {
            0 => Rc::new(Self::Empty),
            1 => flows.into_iter().next().unwrap(),
            _ => Rc::new(Self::Sequence(flows)),
        }
    }

    fn read(name: Word, target: Option<Span>) -> Flow {
        if matches!(
            name.as_bytes(),
            b"$this"
                | b"$_GET"
                | b"$_POST"
                | b"$_COOKIE"
                | b"$_SERVER"
                | b"$_ENV"
                | b"$_REQUEST"
                | b"$_FILES"
                | b"$_SESSION"
                | b"$GLOBALS"
        ) {
            return Rc::new(Self::Empty);
        }
        Rc::new(Self::Read {
            name,
            target,
            container: false,
            offset: None,
        })
    }
}

/// Shared across nested function-body artifacts, but owned by one actual analysis run.
#[derive(Debug, Default)]
pub(crate) struct State {
    expressions: HashMap<Span, Flow>,
    statements: HashMap<Span, Flow>,
    sites: HashMap<Span, VariableWrite>,
    pub catches: HashMap<Span, HashSet<ThrowPoint>>,
    pub completed: HashMap<Span, VariableWritesNode>,
}

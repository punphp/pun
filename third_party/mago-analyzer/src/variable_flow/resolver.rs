//! Backwards liveness, value dependencies and dead-chain coverage from VariableLivenessResolver.

use foldhash::{HashMap, HashSet};
use mago_span::Span;
use mago_word::Word;

use super::{Flow, Offset, ThrowPoint, VariableFlow, VariableWritesNode};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Slot {
    Container,
    Unknown,
    Offset(Offset),
    /// An overwrite marker: the write that left it, and the slot it replaces (None for the whole variable).
    Marker(Span, Option<Offset>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Key {
    name: Word,
    slot: Slot,
    target: Option<Span>,
}

type Live = HashSet<Key>;

#[derive(Default)]
struct Destinations {
    returns: Live,
    breaks: Vec<Live>,
    continues: Vec<Live>,
    catches: Vec<(HashSet<ThrowPoint>, Live)>,
    uncaught: Live,
}

#[derive(Default)]
struct Resolver {
    node: VariableWritesNode,
    slots: HashMap<Word, HashSet<Slot>>,
    dependencies: HashMap<Span, HashSet<Span>>,
    input_copies: HashMap<Span, HashSet<Span>>,
    input_sinks: HashSet<Span>,
    observed: HashSet<Span>,
    literal_items: HashMap<Span, Vec<super::VariableWrite>>,
    returns_by_reference: bool,
    /// Every write access (kind Write, Define or Discard) by variable, in collection order.
    accesses: Vec<(super::VariableWrite, Access)>,
    /// compileOverwrites(): the markers each write observes, and the markers it kills.
    overwrite_keys: HashMap<Span, Vec<Key>>,
    killed_markers: HashMap<Span, Vec<Key>>,
    markers: HashMap<Span, Key>,
}

#[derive(Clone, Copy, PartialEq)]
enum Access {
    Write,
    Define,
    Discard,
}

pub(super) fn resolve(
    flow: &Flow,
    escaped: HashSet<Word>,
    returns_by_reference: bool,
) -> VariableWritesNode {
    let mut resolver = Resolver::default();
    resolver.node.escaped = escaped;
    resolver.returns_by_reference = returns_by_reference;
    resolver.collect(flow);
    if !resolver.node.writes.is_empty() && !resolver.node.opaque {
        resolver.compile_overwrites();
        resolver.live_before(flow, Live::default(), &Destinations::default());
        resolver.resolve_dependencies();
        resolver.resolve_coverage();
    }
    resolver.node.read = resolver
        .observed
        .union(&resolver.node.used)
        .copied()
        .collect();
    resolver.node
}

impl Resolver {
    fn collect(&mut self, flow: &Flow) {
        match &**flow {
            VariableFlow::Read { name, offset, .. } => {
                self.access(*name, offset.as_ref());
                self.node.read_names.insert(*name);
            }
            VariableFlow::Write(write)
            | VariableFlow::Define(write)
            | VariableFlow::Discard(write) => {
                self.access(write.name, write.offset.as_ref());
                let access = match &**flow {
                    VariableFlow::Write(_) => Access::Write,
                    VariableFlow::Define(_) => Access::Define,
                    _ => Access::Discard,
                };
                self.accesses.push((write.clone(), access));
                if !matches!(**flow, VariableFlow::Discard(_)) {
                    self.node.writes.insert(write.node, write.clone());
                    if let Some(parent) = write.parent {
                        self.literal_items
                            .entry(parent)
                            .or_default()
                            .push(write.clone());
                    }
                }
            }
            VariableFlow::Escape(name) => {
                self.access(*name, None);
                self.node.escaped.insert(*name);
            }
            VariableFlow::Mention(name) => {
                self.access(*name, None);
            }
            VariableFlow::Return(Some(name)) if self.returns_by_reference => {
                self.node.escaped.insert(*name);
            }
            VariableFlow::ReadAll | VariableFlow::MentionAll => self.node.all_names = true,
            VariableFlow::Opaque => self.node.opaque = true,
            VariableFlow::Sequence(children) | VariableFlow::Choice(children) => {
                for child in children {
                    self.collect(child);
                }
            }
            VariableFlow::Arrow { body, outputs, .. } => {
                self.collect(body);
                self.collect(outputs);
            }
            VariableFlow::Loop {
                condition,
                body,
                update,
                ..
            } => {
                self.collect(condition);
                self.collect(body);
                self.collect(update);
            }
            VariableFlow::Try {
                body,
                catches,
                finally,
            } => {
                self.collect(body);
                self.collect(finally);
                for (_, catch) in catches {
                    self.collect(catch);
                }
            }
            VariableFlow::Switch {
                condition, cases, ..
            } => {
                self.collect(condition);
                for (condition, body, _) in cases {
                    self.collect(condition);
                    self.collect(body);
                }
            }
            _ => {}
        }
    }

    fn access(&mut self, name: Word, offset: Option<&Offset>) {
        self.node.referenced.insert(name);
        let slots = self
            .slots
            .entry(name)
            .or_insert_with(|| [Slot::Container, Slot::Unknown].into_iter().collect());
        if let Some(offset) = offset {
            slots.insert(Slot::Offset(offset.clone()));
        }
    }

    fn read_keys(
        &self,
        name: Word,
        target: Option<Span>,
        container: bool,
        offset: Option<&Offset>,
    ) -> Live {
        let slots = if container {
            vec![Slot::Container]
        } else if let Some(offset) = offset {
            vec![Slot::Container, Slot::Offset(offset.clone())]
        } else {
            self.slots
                .get(&name)
                .into_iter()
                .flatten()
                .cloned()
                .collect()
        };
        slots
            .into_iter()
            .map(|slot| Key { name, slot, target })
            .collect()
    }

    fn live_before(&mut self, flow: &Flow, mut next: Live, context: &Destinations) -> Live {
        match &**flow {
            VariableFlow::Read {
                name,
                target,
                container,
                offset,
            } => next.extend(self.read_keys(*name, *target, *container, offset.as_ref())),
            VariableFlow::Escape(name) => next.extend(self.read_keys(*name, None, false, None)),
            VariableFlow::Write(write) | VariableFlow::Discard(write) => {
                let selected = |key: &Key| {
                    key.name == write.name
                        && (!write.offset_write
                            || match &write.offset {
                                Some(offset) => key.slot == Slot::Offset(offset.clone()),
                                None => key.slot != Slot::Container,
                            })
                };
                let selected = |key: &Key| !matches!(key.slot, Slot::Marker(..)) && selected(key);
                if matches!(**flow, VariableFlow::Write(_)) {
                    self.observe_overwrite(write.node, &next);
                    for item in self.literal_items.get(&write.node).cloned().into_iter().flatten() {
                        self.observe_overwrite(item.node, &next);
                    }
                    for key in next.iter().filter(|key| selected(key)) {
                        self.observed.insert(write.node);
                        if let Some(target) = key.target {
                            self.dependencies
                                .entry(target)
                                .or_default()
                                .insert(write.node);
                        } else {
                            self.node.used.insert(write.node);
                        }
                    }
                    for item in self.literal_items.get(&write.node).into_iter().flatten() {
                        for key in next.iter().filter(|key| {
                            !matches!(key.slot, Slot::Marker(..))
                                && key.name == item.name
                                && match &item.offset {
                                    Some(offset) => key.slot == Slot::Offset(offset.clone()),
                                    None => key.slot != Slot::Container,
                                }
                        }) {
                            self.observed.insert(item.node);
                            if let Some(target) = key.target {
                                self.dependencies
                                    .entry(target)
                                    .or_default()
                                    .insert(item.node);
                            } else {
                                self.node.used.insert(item.node);
                            }
                        }
                    }
                }
                if !write.offset_write || write.offset.is_some() && write.replaces_offset {
                    next.retain(|key| !selected(key));
                }
                for key in self.killed_markers.get(&write.node).into_iter().flatten() {
                    next.remove(key);
                }
                if matches!(**flow, VariableFlow::Write(_))
                    && let Some(marker) = self.markers.get(&write.node)
                {
                    next.insert(marker.clone());
                }
            }
            VariableFlow::Inputs { write, target } => {
                if let Some(target) = target {
                    self.input_copies.entry(*target).or_default().insert(*write);
                } else {
                    self.input_sinks.insert(*write);
                }
            }
            VariableFlow::Sequence(children) => {
                for child in children.iter().rev() {
                    next = self.live_before(child, next, context);
                }
            }
            VariableFlow::Choice(children) => {
                next = children
                    .iter()
                    .flat_map(|child| self.live_before(child, next.clone(), context))
                    .collect();
            }
            VariableFlow::Arrow {
                body,
                outputs,
                parameters,
            } => {
                let outputs = self.live_before(outputs, Live::default(), &Destinations::default());
                let arrow_context = Destinations {
                    returns: outputs.clone(),
                    uncaught: outputs.clone(),
                    ..Default::default()
                };
                let mut names = self.live_before(body, outputs, &arrow_context);
                // the arrow function's own writes replace nothing outside of it
                names.retain(|key| !parameters.contains(&key.name) && !matches!(key.slot, Slot::Marker(..)));
                next.extend(names);
            }
            VariableFlow::Loop {
                condition,
                body,
                update,
                at_least_once,
            } => {
                let mut head = Live::default();
                let body_entry = loop {
                    let count = head.len();
                    let update_entry = self.live_before(update, head, context);
                    let loop_context = Destinations {
                        returns: context.returns.clone(),
                        breaks: std::iter::once(next.clone())
                            .chain(context.breaks.clone())
                            .collect(),
                        continues: std::iter::once(update_entry.clone())
                            .chain(context.continues.clone())
                            .collect(),
                        catches: context.catches.clone(),
                        uncaught: context.uncaught.clone(),
                    };
                    let body_entry = self.live_before(body, update_entry, &loop_context);
                    // The pinned statement handlers deliberately allow both iteration and exit
                    // regardless of inferred loop conditions (including `while (true)`).
                    let mut after_condition = body_entry.clone();
                    after_condition.extend(next.clone());
                    head = self.live_before(condition, after_condition, context);
                    if head.len() == count {
                        break body_entry;
                    }
                };
                next = if *at_least_once {
                    self.live_before(condition, body_entry, context)
                } else {
                    head
                };
            }
            VariableFlow::Return(_) => next.clone_from(&context.returns),
            VariableFlow::Break(level) => {
                next = context
                    .breaks
                    .get(level.saturating_sub(1))
                    .cloned()
                    .unwrap_or_default()
            }
            VariableFlow::Continue(level) => {
                next = context
                    .continues
                    .get(level.saturating_sub(1))
                    .cloned()
                    .unwrap_or_default()
            }
            VariableFlow::Stop => next.clear(),
            VariableFlow::Throw {
                point,
                can_continue,
            } => {
                if !can_continue {
                    next.clear();
                }
                let mut caught = false;
                for (points, destination) in &context.catches {
                    if points.contains(point) {
                        next.extend(destination.clone());
                        caught = true;
                    }
                }
                if !caught {
                    next.extend(context.uncaught.clone());
                }
            }
            VariableFlow::Try {
                body,
                catches,
                finally,
            } => {
                let normal = self.live_before(finally, next, context);
                let catch_context = Destinations {
                    returns: self.live_before(finally, context.returns.clone(), context),
                    breaks: context
                        .breaks
                        .iter()
                        .map(|destination| self.live_before(finally, destination.clone(), context))
                        .collect(),
                    continues: context
                        .continues
                        .iter()
                        .map(|destination| self.live_before(finally, destination.clone(), context))
                        .collect(),
                    catches: context
                        .catches
                        .iter()
                        .map(|(points, destination)| {
                            (
                                points.clone(),
                                self.live_before(finally, destination.clone(), context),
                            )
                        })
                        .collect(),
                    uncaught: self.live_before(finally, context.uncaught.clone(), context),
                };
                let mut destinations = Vec::new();
                for (points, catch) in catches {
                    destinations.push((
                        points.clone(),
                        self.live_before(catch, normal.clone(), &catch_context),
                    ));
                }
                destinations.extend(catch_context.catches.clone());
                next = self.live_before(
                    body,
                    normal,
                    &Destinations {
                        catches: destinations,
                        ..catch_context
                    },
                );
            }
            VariableFlow::Switch {
                condition,
                cases,
                exhaustive,
            } => {
                let switch_context = Destinations {
                    returns: context.returns.clone(),
                    breaks: std::iter::once(next.clone())
                        .chain(context.breaks.clone())
                        .collect(),
                    continues: std::iter::once(next.clone())
                        .chain(context.continues.clone())
                        .collect(),
                    catches: context.catches.clone(),
                    uncaught: context.uncaught.clone(),
                };
                let mut case_next = next.clone();
                let mut unmatched = if *exhaustive { Live::default() } else { next };
                let mut entries = Vec::new();
                for (_, body, default) in cases.iter().rev() {
                    case_next = self.live_before(body, case_next, &switch_context);
                    entries.push(case_next.clone());
                    if *default {
                        unmatched.clone_from(&case_next);
                    }
                }
                for ((condition, _, default), entry) in cases.iter().rev().zip(entries) {
                    if !default {
                        unmatched.extend(entry);
                        unmatched = self.live_before(condition, unmatched, context);
                    }
                }
                next = self.live_before(condition, unmatched, context);
            }
            VariableFlow::ReadAll => {
                self.node.read_names.extend(self.node.referenced.iter().copied());
                for (&name, slots) in &self.slots {
                    next.extend(slots.iter().cloned().map(|slot| Key {
                        name,
                        slot,
                        target: None,
                    }));
                }
            }
            _ => {}
        }
        next
    }

    /// VariableLivenessResolver::compileOverwrites(): a write replacing the variable or one of its offsets leaves
    /// a marker that the previous replacing write of the same slot finds after it.
    fn compile_overwrites(&mut self) {
        let replaced = |write: &super::VariableWrite| -> Option<Option<Offset>> {
            if !write.offset_write {
                return Some(None);
            }
            write.offset.clone().filter(|_| write.replaces_offset).map(Some)
        };
        let mut markers: Vec<(Option<Offset>, Key, Span)> = Vec::new();
        for (write, access) in &self.accesses {
            if *access != Access::Write {
                continue;
            }
            let Some(slot) = replaced(write) else { continue };
            let key = Key {
                name: write.name,
                slot: Slot::Marker(write.node, slot.clone()),
                target: None,
            };
            self.markers.insert(write.node, key.clone());
            markers.push((slot, key, write.node));
        }
        for (write, _) in &self.accesses {
            let observed = markers.iter().filter(|(slot, key, writer)| {
                key.name == write.name
                    && (slot.is_none() || write.offset_write && slot.is_some() && *slot == write.offset)
                    && *writer != write.node
                    && Some(*writer) != write.parent
            });
            self.overwrite_keys
                .entry(write.node)
                .or_default()
                .extend(observed.map(|(_, key, _)| key.clone()));
            let Some(slot) = replaced(write) else { continue };
            let killed = markers.iter().filter(|(marker_slot, key, _)| {
                key.name == write.name && (slot.is_none() || *marker_slot == slot)
            });
            self.killed_markers
                .entry(write.node)
                .or_default()
                .extend(killed.map(|(_, key, _)| key.clone()));
        }
    }

    fn observe_overwrite(&mut self, id: Span, next: &Live) {
        if self.overwrite_keys.get(&id).into_iter().flatten().any(|key| next.contains(key)) {
            self.node.overwritten.insert(id);
        }
    }

    fn resolve_dependencies(&mut self) {
        let mut stack: Vec<_> = self
            .node
            .used
            .iter()
            .map(|id| (*id, false))
            .chain(self.input_sinks.iter().map(|id| (*id, true)))
            .collect();
        for (id, write) in &self.node.writes {
            if self.node.escaped.contains(&write.name) {
                stack.push((*id, false));
                stack.push((*id, true));
            }
        }
        let mut visited = HashSet::default();
        while let Some((id, inputs)) = stack.pop() {
            if !visited.insert((id, inputs)) {
                continue;
            }
            for dependency in self.dependencies.get(&id).into_iter().flatten() {
                self.node.used.insert(*dependency);
                stack.push((*dependency, false));
            }
            for source in self.input_copies.get(&id).into_iter().flatten() {
                stack.push((*source, true));
            }
            if inputs {
                for item in self.literal_items.get(&id).into_iter().flatten() {
                    stack.push((item.node, true));
                }
            }
        }
    }

    fn resolve_coverage(&mut self) {
        let mut stack: Vec<_> = self
            .node
            .writes
            .keys()
            .filter(|id| !self.observed.contains(id) && !self.node.used.contains(id))
            .copied()
            .collect();
        let mut visited = HashSet::default();
        while let Some(id) = stack.pop() {
            if !visited.insert(id) {
                continue;
            }
            let mut sources: Vec<_> = self
                .dependencies
                .get(&id)
                .into_iter()
                .flatten()
                .copied()
                .collect();
            for copied in self.input_copies.get(&id).into_iter().flatten() {
                sources.extend(self.dependencies.get(copied).into_iter().flatten().copied());
            }
            for source in sources {
                self.node.covered.insert(source);
                stack.push(source);
            }
        }
    }
}

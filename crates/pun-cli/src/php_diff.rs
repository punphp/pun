//! sebastian/diff 6.0.2 as PHP-CS-Fixer's UnifiedDiffer drives it: `Differ::diffToArray` (common prefix/suffix
//! stripped, then the time- or memory-efficient LCS, then a greedy walk) rendered by `StrictUnifiedDiffOutputBuilder`
//! (3 context lines, hunks split after 6 common lines). Which lines a diff pairs up depends on these exact
//! algorithms, so a generic differ prints different (equally valid) hunks than Pint.

const OLD: u8 = 0;
const ADDED: u8 = 1;
const REMOVED: u8 = 2;
const DIFF_LINE_END_WARNING: u8 = 3;
const NO_LINE_END_EOF_WARNING: u8 = 4;

/// `--- {path}\n+++ {path}\n` and the hunks; empty when nothing changed.
pub fn unified(old: &[u8], new: &[u8], path: &str) -> Vec<u8> {
    let diff = diff_to_array(split_lines(old), split_lines(new));
    get_diff(&format!("--- {path}\n+++ {path}\n"), diff)
}

/// `preg_split('/(.*\R)/', $s, -1, DELIM_CAPTURE | NO_EMPTY)`: `.` stops only at `\n`, so a line ends at the next
/// `\n`; without one, the greedy `.*` backtracks to the last other `\R` byte (`\r`, `\v`, `\f`, `\x85`).
fn split_lines(mut s: &[u8]) -> Vec<&[u8]> {
    let mut lines = vec![];
    while !s.is_empty() {
        let end = match s.iter().position(|&c| c == b'\n') {
            Some(i) => i + 1,
            None => s.iter().rposition(|c| b"\r\x0b\x0c\x85".contains(c)).map_or(s.len(), |i| i + 1),
        };
        lines.push(&s[..end]);
        s = &s[end..];
    }
    lines
}

fn diff_to_array<'a>(mut from: Vec<&'a [u8]>, mut to: Vec<&'a [u8]>) -> Vec<(&'a [u8], u8)> {
    let start = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let end = from[start..].iter().rev().zip(to[start..].iter().rev()).take_while(|(a, b)| a == b).count();
    let tail: Vec<&[u8]> = from.drain(from.len() - end..).collect();
    to.truncate(to.len() - end);
    let head: Vec<&[u8]> = from.drain(..start).collect();
    to.drain(..start);

    // 144 bytes per matrix item on 64-bit PHP, against a 100 MiB budget.
    let common = if 144 * from.len().min(to.len()).pow(2) > 100 * 1024 * 1024 {
        lcs_memory_efficient(&from, &to)
    } else {
        lcs_time_efficient(&from, &to)
    };

    let mut diff: Vec<(&[u8], u8)> = head.into_iter().map(|l| (l, OLD)).collect();
    let (mut f, mut t) = (from.into_iter().peekable(), to.into_iter().peekable());
    for token in common {
        while f.peek() != Some(&token) {
            diff.push((f.next().unwrap(), REMOVED));
        }
        while t.peek() != Some(&token) {
            diff.push((t.next().unwrap(), ADDED));
        }
        diff.push((token, OLD));
        f.next();
        t.next();
    }
    diff.extend(f.map(|l| (l, REMOVED)));
    diff.extend(t.map(|l| (l, ADDED)));
    diff.extend(tail.into_iter().map(|l| (l, OLD)));
    if unmatched_line_endings(&diff) {
        diff.insert(0, (b"#Warning: Strings contain different line endings!\n", DIFF_LINE_END_WARNING));
    }
    diff
}

fn lcs_time_efficient<'a>(from: &[&'a [u8]], to: &[&'a [u8]]) -> Vec<&'a [u8]> {
    let width = from.len() + 1;
    let mut m = vec![0u32; width * (to.len() + 1)];
    for j in 1..=to.len() {
        for i in 1..=from.len() {
            let o = j * width + i;
            let first_or_last = if from[i - 1] == to[j - 1] { m[o - width - 1] + 1 } else { 0 };
            let best = if m[o - 1] > m[o - width] { m[o - 1] } else { m[o - width] };
            m[o] = first_or_last.max(best);
        }
    }
    let (mut i, mut j, mut common) = (from.len(), to.len(), vec![]);
    while i > 0 && j > 0 {
        if from[i - 1] == to[j - 1] {
            common.push(from[i - 1]);
            i -= 1;
            j -= 1;
        } else if m[j * width + i - width] > m[j * width + i - 1] {
            j -= 1;
        } else {
            i -= 1;
        }
    }
    common.reverse();
    common
}

fn lcs_memory_efficient<'a>(from: &[&'a [u8]], to: &[&'a [u8]]) -> Vec<&'a [u8]> {
    // Never called with an empty `from`: the first call has min(len)^2 large, the halves of 2+ lines are non-empty.
    if from.len() == 1 {
        return if to.contains(&from[0]) { vec![from[0]] } else { vec![] };
    }
    let i = from.len() / 2;
    let (from_start, from_end) = from.split_at(i);
    let ll_b = lcs_length(from_start.iter(), to);
    let to_rev: Vec<&[u8]> = to.iter().rev().copied().collect();
    let ll_e = lcs_length(from_end.iter().rev(), &to_rev);
    let (mut j_max, mut max) = (0, 0);
    for j in 0..=to.len() {
        let m = ll_b[j] + ll_e[to.len() - j];
        if m >= max {
            max = m;
            j_max = j;
        }
    }
    let (to_start, to_end) = to.split_at(j_max);
    let mut common = lcs_memory_efficient(from_start, to_start);
    common.extend(lcs_memory_efficient(from_end, to_end));
    common
}

fn lcs_length<'a, 'b>(from: impl Iterator<Item = &'b &'a [u8]>, to: &[&'a [u8]]) -> Vec<u32>
where
    'a: 'b,
{
    let mut current = vec![0u32; to.len() + 1];
    for f in from {
        let prev = current.clone();
        for j in 0..to.len() {
            current[j + 1] = if *f == to[j] { prev[j] + 1 } else { current[j].max(prev[j + 1]) };
        }
    }
    current
}

fn line_break(line: &[u8]) -> &'static [u8] {
    match line {
        [.., b'\r', b'\n'] => b"\r\n",
        [.., b'\n'] => b"\n",
        [.., b'\r'] => b"\r",
        _ => b"",
    }
}

fn unmatched_line_endings(diff: &[(&[u8], u8)]) -> bool {
    let (mut new, mut old): (Vec<&[u8]>, Vec<&[u8]>) = (vec![b""], vec![b""]);
    let add = |set: &mut Vec<&'static [u8]>, lb: &'static [u8]| {
        if !set.contains(&lb) {
            set.push(lb);
        }
    };
    for &(line, kind) in diff {
        let lb = line_break(line);
        match kind {
            OLD => {
                add(&mut old, lb);
                add(&mut new, lb);
            }
            ADDED => add(&mut new, lb),
            _ => add(&mut old, lb),
        }
    }
    if new.len() == 1 || old.len() == 1 {
        return false;
    }
    new.iter().any(|b| !old.contains(b)) || old.iter().any(|b| !new.contains(b))
}

/// Only called for changed files, so `diff` has an added or removed line and the output ends with a line break
/// (a last line without one gets the "No newline" marker), which getDiff would otherwise append.
fn get_diff(header: &str, mut diff: Vec<(&[u8], u8)>) -> Vec<u8> {
    let mut out = header.as_bytes().to_vec();
    write_diff_hunks(&mut out, &mut diff);
    out
}

const NO_EOL: &[u8] = b"\n\\ No newline at end of file\n";
const CONTEXT: usize = 3;
const CUT_OFF: usize = 6; // max(commonLineThreshold 6, contextLines 3)

fn write_diff_hunks(out: &mut Vec<u8>, diff: &mut Vec<(&[u8], u8)>) {
    let upper = diff.len();
    if diff[upper - 1].1 == OLD {
        if !diff[upper - 1].0.ends_with(b"\n") {
            diff.push((NO_EOL, NO_LINE_END_EOF_WARNING));
        }
    } else {
        let mut to_find = vec![ADDED, REMOVED];
        for i in (0..upper).rev() {
            if let Some(p) = to_find.iter().position(|&k| k == diff[i].1) {
                to_find.remove(p);
                if !diff[i].0.ends_with(b"\n") {
                    diff.insert(i + 1, (NO_EOL, NO_LINE_END_EOF_WARNING));
                }
                if to_find.is_empty() {
                    break;
                }
            }
        }
    }

    let mut hunk_capture: Option<usize> = None;
    let (mut same_count, mut to_range, mut from_range) = (0, 0, 0);
    let (mut to_start, mut from_start) = (1, 1);
    let mut last = 0;
    for (i, &(_, kind)) in diff.iter().enumerate() {
        last = i;
        if kind == OLD {
            let Some(capture) = hunk_capture else {
                from_start += 1;
                to_start += 1;
                continue;
            };
            same_count += 1;
            to_range += 1;
            from_range += 1;
            if same_count == CUT_OFF {
                let ctx_start = capture.min(CONTEXT);
                write_hunk(
                    out,
                    diff,
                    capture - ctx_start,
                    i + 1 + CONTEXT - CUT_OFF,
                    (from_start - ctx_start, from_range + ctx_start + CONTEXT - CUT_OFF),
                    (to_start - ctx_start, to_range + ctx_start + CONTEXT - CUT_OFF),
                );
                from_start += from_range;
                to_start += to_range;
                hunk_capture = None;
                same_count = 0;
                to_range = 0;
                from_range = 0;
            }
            continue;
        }
        same_count = 0;
        if kind == NO_LINE_END_EOF_WARNING {
            continue;
        }
        hunk_capture.get_or_insert(i);
        if kind == ADDED {
            to_range += 1;
        }
        if kind == REMOVED {
            from_range += 1;
        }
    }
    let Some(capture) = hunk_capture else { return };
    let ctx_start = capture.min(CONTEXT);
    let ctx_end = same_count.min(CONTEXT);
    from_range -= same_count;
    to_range -= same_count;
    write_hunk(
        out,
        diff,
        capture - ctx_start,
        last - same_count + ctx_end + 1,
        (from_start - ctx_start, from_range + ctx_start + ctx_end),
        (to_start - ctx_start, to_range + ctx_start + ctx_end),
    );
}

fn write_hunk(out: &mut Vec<u8>, diff: &[(&[u8], u8)], start: usize, end: usize, from: (usize, usize), to: (usize, usize)) {
    let range = |(s, r): (usize, usize)| if r == 1 { s.to_string() } else { format!("{s},{r}") };
    out.extend_from_slice(format!("@@ -{} +{} @@\n", range(from), range(to)).as_bytes());
    for &(line, kind) in &diff[start..end] {
        match kind {
            ADDED => out.push(b'+'),
            REMOVED => out.push(b'-'),
            OLD => out.push(b' '),
            NO_LINE_END_EOF_WARNING => {}
            _ => continue,
        }
        out.extend_from_slice(line);
    }
}

#[cfg(test)]
mod tests {
    // Expectations: PhpCsFixer\Differ\UnifiedDiffer::diff (php-cs-fixer v3.95.17) on the same strings.
    #[test]
    fn missing_final_newline_and_cr_lines() {
        let d = |a: &[u8], b: &[u8]| String::from_utf8(super::unified(a, b, "/f.php")).unwrap();
        assert_eq!(d(b"x\r", b"y\r"), "--- /f.php\n+++ /f.php\n@@ -1 +1 @@\n-x\r\n\\ No newline at end of file\n+y\r\n\\ No newline at end of file\n");
        assert_eq!(d(b"a\nb", b"A\nb"), "--- /f.php\n+++ /f.php\n@@ -1,2 +1,2 @@\n-a\n+A\n b\n\\ No newline at end of file\n");
        assert_eq!(d(b"a\rb\r", b"a\rc\r"), "--- /f.php\n+++ /f.php\n@@ -1 +1 @@\n-a\rb\r\n\\ No newline at end of file\n+a\rc\r\n\\ No newline at end of file\n");
    }
}

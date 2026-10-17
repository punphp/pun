//! `--features pprof` builds only: `PUN_PPROF=<file>` samples every thread (SIGPROF, 997 Hz, works where `perf` is
//! not allowed) and writes folded stacks (`a;b;c count`, flamegraph.pl / inferno input) to `<file>` on exit.
pub struct Sampler(pprof::ProfilerGuard<'static>, std::ffi::OsString);

pub fn start() -> Option<Sampler> {
    let out = std::env::var_os("PUN_PPROF")?;
    Some(Sampler(
        pprof::ProfilerGuardBuilder::default()
            .frequency(997)
            .build()
            .expect("pprof"),
        out,
    ))
}

impl Sampler {
    pub fn report(self) {
        let report = self.0.report().build().expect("pprof report");
        let mut folded = String::new();
        for (frames, n) in &report.data {
            let names: Vec<String> = frames
                .frames
                .iter()
                .rev()
                .flat_map(|f| f.iter().rev().map(|s| s.name().replace(';', ",")))
                .collect();
            folded += &format!("{} {n}\n", names.join(";"));
        }
        std::fs::write(&self.1, folded).expect("write PUN_PPROF file");
    }
}

struct AllProbes<'a> {
    empty_tls_probes: [Vec<EmptyProbe<'a>>; 3],
}

struct EmptyProbe<'a> {
    service: &'a str,
}
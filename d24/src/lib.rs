use fxhash::FxHashMap;
use itertools::Itertools;

pub fn p1(s: &str) -> u64 {
    let (mut vals, graph) = parse(s);
    for &k in graph.keys().filter(|k| k.starts_with('z')) {
        let v = dfs(k, &graph, &mut vals);
        vals.insert(k, v);
    }
    vals.into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted_unstable_by_key(|(k, _)| *k)
        .rev()
        .fold(0, |acc, b| (acc << 1) | u64::from(b.1))
}

pub fn p2(s: &str) -> String {
    let (_, out_in) = parse(s);
    let in_out = out_in.iter().fold(
        FxHashMap::<&str, Vec<&str>>::default(),
        |mut acc, (k, v)| {
            acc.entry(v[0]).or_default().push(k);
            acc.entry(v[2]).or_default().push(k);
            acc
        },
    );
    let maxz = out_in
        .keys()
        .filter(|v| v.starts_with('z'))
        .copied()
        .max()
        .unwrap_or("z45");
    let mut res = Vec::with_capacity(8);
    for (&output, &[a, op, _b]) in out_in.iter() {
        match op {
            "AND" => {
                if ["x00", "y00"].contains(&a) {
                    if in_out.get(output).is_none_or(|downstream| {
                        downstream.len() != 2 || downstream.iter().all(|v| out_in[v][1] == "OR")
                    }) {
                        res.push(output);
                    }
                } else if in_out.get(output).is_none_or(|downstream| {
                    downstream.len() != 1 || out_in[downstream[0]][1] != "OR"
                }) {
                    res.push(output);
                }
            }
            "XOR" => {
                if a.starts_with(['x', 'y']) && !["x00", "y00"].contains(&a) {
                    if in_out.get(output).is_none_or(|downstream| {
                        downstream.len() != 2 || downstream.iter().all(|v| out_in[v][1] == "OR")
                    }) {
                        res.push(output);
                    }
                } else if !output.starts_with('z') {
                    res.push(output);
                }
            }
            "OR" => {
                if let Some(downstream) = in_out.get(output) {
                    if downstream.len() != 2 || downstream.iter().all(|v| out_in[v][1] == "OR") {
                        res.push(output);
                    }
                } else if output != maxz {
                    res.push(output);
                }
            }
            _ => unreachable!(),
        }
    }
    res.sort_unstable();
    res.into_iter().join(",")
}

fn dfs<'a>(
    curr: &'a str,
    graph: &FxHashMap<&str, [&'a str; 3]>,
    vals: &mut FxHashMap<&'a str, u8>,
) -> u8 {
    if let Some(&v) = vals.get(curr) {
        return v;
    }
    let [a, op, c] = graph[curr];
    let v1 = dfs(a, graph, vals);
    let v2 = dfs(c, graph, vals);
    let res = match op {
        "AND" => v1 & v2,
        "XOR" => v1 ^ v2,
        "OR" => v1 | v2,
        _ => unreachable!(),
    };
    vals.insert(curr, res);
    res
}

fn parse(s: &str) -> (FxHashMap<&str, u8>, FxHashMap<&str, [&str; 3]>) {
    let mut is_init = true;
    let mut init = FxHashMap::default();
    let mut graph = FxHashMap::default();
    for line in s.lines() {
        if is_init {
            let Some((a, b)) = line.split_once(": ") else {
                is_init = false; // empty line
                continue;
            };
            init.insert(a, b.parse().unwrap_or_default());
        } else {
            let v = line
                .split(|c: char| !c.is_ascii_alphanumeric())
                .filter(|v| !v.is_empty())
                .collect_vec();
            graph.insert(v[3], [v[0], v[1], v[2]]);
        }
    }
    (init, graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    const TEST2: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    const TEST3: &str = r#"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST1), 4);
        assert_eq!(p1(TEST2), 2024);
        assert_eq!(p1(INPUT), 43942008931358);

        // assert_eq!(p2(TEST3), "z00,z01,z02,z05"); // Output all z's
        assert_eq!(p2(INPUT), "dvb,fhg,fsq,tnc,vcf,z10,z17,z39");
    }
}

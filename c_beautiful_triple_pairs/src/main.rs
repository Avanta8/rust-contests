//{"name":"C. Beautiful Triple Pairs","group":"Codeforces - Codeforces Round 946 (Div. 3)","url":"https://codeforces.com/problemset/problem/1974/C","interactive":false,"timeLimit":4000,"tests":[{"input":"8\n5\n3 2 2 2 3\n5\n1 2 1 2 1\n8\n1 2 3 2 2 3 4 2\n4\n2 1 1 1\n8\n2 1 1 2 1 1 1 1\n7\n2 1 1 1 1 1 1\n6\n2 1 1 1 1 1\n5\n2 1 1 1 1\n","output":"2\n0\n3\n1\n8\n4\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBeautifulTriplePairs"}}}

#[allow(unused_imports)]
use algo_lib::{
    collections::fxhash::FxHashMap,
    dbg,
    io::{input::Input, output::Output},
};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let arr = input.read_long_vec(n);

    let mut all_counts = vec![FxHashMap::default(); 3];

    for window in arr.windows(3) {
        for i in 0..3 {
            let mut rem = window.to_vec();
            let value = rem.remove(i);

            *all_counts[i]
                .entry(rem)
                .or_insert_with(FxHashMap::default)
                .entry(value)
                .or_insert(0) += 1;
        }
    }

    let mut total = 0;
    for i_counts in all_counts {
        for counts in i_counts.values() {
            let sum = counts.values().sum::<i64>();
            for count in counts.values() {
                total += count * (sum - count);
            }
        }
    }

    out.print_line(total / 2);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::time;
use regex::Regex;
use std::mem::size_of;
use std::time::{Duration, Instant};

pub type STREAM_TYPE = i128;

const STREAM_ARRAY_SIZE: usize = 10000000; // maybe change this to u128??
const N_TIMES: usize = 20;
const OFFSET: usize = 0;
const SCALAR: STREAM_TYPE = 3;
const BYTES_PER_WORD: usize = size_of::<STREAM_TYPE>();
const M: usize = 20;
const BYTES: usize = 3_usize * size_of::<STREAM_TYPE>() * STREAM_ARRAY_SIZE; //total size of all the arrays used in computation(Triad in this case, b/c this benchmark only have Triad for now...) in bytes
const LABEL: &str = "Triad:     ";
const H_LINE: &str = "-------------------------------------------------------------";

static mut STREAM_ARRAY_A: [STREAM_TYPE; STREAM_ARRAY_SIZE + OFFSET] =
    [1; STREAM_ARRAY_SIZE + OFFSET];
static mut STREAM_ARRAY_B: [STREAM_TYPE; STREAM_ARRAY_SIZE + OFFSET] =
    [2; STREAM_ARRAY_SIZE + OFFSET];
static mut STREAM_ARRAY_C: [STREAM_TYPE; STREAM_ARRAY_SIZE + OFFSET] =
    [0; STREAM_ARRAY_SIZE + OFFSET];

fn checktick() -> i32 {
    let mut minDelta: i32;
    let mut Delta: i32;
    let mut timesfound: [u128; M] = [0; M];

    /*  Collect a sequence of M unique time values from the system. */

    // timesfound.into_iter().map(|mut x| {
    //     let t1 = Instant::now();
    //     while (Instant::now() - t1) < time::Duration::from_micros(1) {}
    //     x = t1.elapsed().as_micros();
    // });

    for i in 1..M {
        let t1 = Instant::now();
        while (Instant::now() - t1) < time::Duration::from_micros(1) {}
        timesfound[i] = t1.elapsed().as_micros();
    }

    /*
     * Determine the minimum difference between these M values.
     * This result will be our estimate (in microseconds) for the
     * clock granularity.
     */

    minDelta = 1000000;
    for i in 1..M {
        Delta = (timesfound[i] - timesfound[i - 1]) as i32; // this is in microseconds
        minDelta = std::cmp::min(minDelta, std::cmp::max(Delta, 0));
    }

    minDelta
}

// fn check_stream_results() {
//     let epsilon: f64;
//     let (mut ierr, mut err): (i32, i32);
//     let (mut aj, bj, cj): (STREAM_TYPE, STREAM_TYPE, STREAM_TYPE) = (1, 2, 0);
//     let (mut aSumErr, mut bSumErr, mut cSumErr): (STREAM_TYPE, STREAM_TYPE, STREAM_TYPE);
//     let (aAvgErr, bAvgErr, cAvgErr): (STREAM_TYPE, STREAM_TYPE, STREAM_TYPE);
//     /* a[] is modified during timing check */
//     aj = 2 * aj;

//     /* now execute timing loop */
//     for _k in 0..N_TIMES {
//         aj = bj + SCALAR * cj;
//     }

//     /* accumulate deltas between observed and expected results */
//     aSumErr = 0;
//     bSumErr = 0;
//     cSumErr = 0;
//     for j in 0..STREAM_ARRAY_SIZE {
//         unsafe {
//             aSumErr += (STREAM_ARRAY_A[j] - aj).abs();
//             bSumErr += (STREAM_ARRAY_B[j] - bj).abs();
//             cSumErr += (STREAM_ARRAY_C[j] - cj).abs();
//         }
//     }
//     aAvgErr = aSumErr / STREAM_ARRAY_SIZE as STREAM_TYPE;
//     bAvgErr = bSumErr / STREAM_ARRAY_SIZE as STREAM_TYPE;
//     cAvgErr = cSumErr / STREAM_ARRAY_SIZE as STREAM_TYPE;

//     if size_of::<STREAM_TYPE>() == 4 {
//         epsilon = 1.0e-6;
//     } else if size_of::<STREAM_TYPE>() == 8 {
//         epsilon = 1.0e-13;
//     } else {
//         println!(
//             "WEIRD: size_of::<STREAM_TYPE>() = {}",
//             size_of::<STREAM_TYPE>()
//         );
//         epsilon = 1.0e-6;
//     }

//     err = 0;
//     if (aAvgErr / aj).abs() > epsilon as i128 {
//         err += 1;
//         println!(
//             "Failed Validation on array a[], AvgRelAbsErr > epsilon ({})",
//             epsilon
//         );
//         println!(
//             "     Expected Value: {}, AvgAbsErr: {}, AvgRelAbsErr: {}",
//             aj,
//             aAvgErr,
//             aAvgErr.abs() / aj
//         );
//         ierr = 0;
//         for j in 0..STREAM_ARRAY_SIZE {
//             unsafe {
//                 if (STREAM_ARRAY_A[j] / aj - 1).abs() > epsilon as i128 {
//                     ierr += 1;
//                     //TODO: maybe consider using the tracing crate for the VERBOSE in c
//                 }
//             }
//         }
//         println!("     For array a[], {} errors were found.", ierr);
//     }
//     if (bAvgErr / bj).abs() > epsilon {
//         err += 1;
//         println!(
//             "Failed Validation on array b[], AvgRelAbsErr > epsilon ({})",
//             epsilon
//         );
//         println!(
//             "     Expected Value: {}, AvgAbsErr: {}, AvgRelAbsErr: {}",
//             bj,
//             bAvgErr,
//             bAvgErr.abs() / bj
//         );
//         println!("     AvgRelAbsErr > Epsilon ({})", epsilon);
//         ierr = 0;
//         for j in 0..STREAM_ARRAY_SIZE {
//             unsafe {
//                 if (STREAM_ARRAY_B[j] / bj - 1.0).abs() > epsilon {
//                     ierr += 1;
//                     //TODO: VERBOSE in c
//                 }
//             }
//         }
//         println!("     For array b[], {} errors were found.", ierr);
//     }
//     if (cAvgErr / cj).abs() > epsilon {
//         err += 1;
//         println!(
//             "Failed Validation on array c[], AvgRelAbsErr > epsilon ({})",
//             epsilon
//         );
//         println!(
//             "     Expected Value: {}, AvgAbsErr: {}, AvgRelAbsErr: {}",
//             cj,
//             cAvgErr,
//             (cAvgErr).abs() / cj
//         );
//         println!("     AvgRelAbsErr > Epsilon ({})", epsilon);
//         ierr = 0;
//         for j in 0..STREAM_ARRAY_SIZE {
//             unsafe {
//                 if (STREAM_ARRAY_C[j] / cj - 1.0).abs() > epsilon {
//                     ierr += 1;
//                     //TODO: VERBOSE in c
//                 }
//             }
//         }
//         println!("     For array c[], {} errors were found.", ierr);
//     }
//     if err == 0 {
//         println!(
//             "Solution Validates: avg error less than {} on all three arrays",
//             epsilon
//         );
//     }
// }

fn get_cpu_info() -> usize {
    let stdout;
    if cfg!(target_os = "linux") {
        stdout = std::process::Command::new("nproc")
            .arg("--all")
            .output()
            .expect("failed to get cpuinfo from nproc");
    } else if cfg!(target_os = "macos") {
        stdout = std::process::Command::new("sysctl")
            .arg("-n")
            .arg("hw.ncpu")
            .output()
            .expect("failed to get cpuinfo from sysctl");
    } else if cfg!(target_os = "windows") {
        stdout = std::process::Command::new("wmic")
            .arg("cpu")
            .arg("get")
            .arg("NumberOfCores")
            .output()
            .expect("failed to get cpuinfo from wmic");
    } else {
        panic!("unsupported OS");
    }

    let s = std::str::from_utf8(&stdout.stdout)
        .unwrap_or_else(|_| panic!("failed to get cpuinfo from stdout"))
        .trim();
    let re = Regex::new(r"[0-9]+")
        .unwrap_or_else(|_| panic!("failed to get the cores count of the CPU"));
    let result = re.captures(s).unwrap();

    result
        .get(0)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("failed to parse cpuinfo output")
}

fn main() {
    let cpu_count = get_cpu_info();
    // let mut times: MaybeUninit<[Instant; N_TIMES]> = MaybeUninit::uninit();
    let mut times: [Instant; N_TIMES] = [Instant::now(); N_TIMES];
    let mut durations: [Duration; N_TIMES] = [Default::default(); N_TIMES];
    let t: Instant;
    let mut quantum: i32;
    let mut avgtime: f64 = 0.;
    let mut maxtime: f64 = 0.;
    let mut mintime: f64 = f64::MAX;
    /* --- SETUP --- determine precision and check timing --- */
    println!("{}", H_LINE);
    println!("WELCOME TO SAWTOOTH STREAM TRIAD BENCHMARK");
    println!("{}", H_LINE);
    println!(
        "This system uses {} bytes per array element.",
        BYTES_PER_WORD
    );
    println!("This system has {} threads.", cpu_count);
    println!("{}", H_LINE);

    println!(
        "Array size = {} (elements), OFFSET = {} (elements)",
        STREAM_ARRAY_SIZE, OFFSET
    );
    println!(
        "Memory per array = {:.1} MiB (= {:.1} GiB).",
        BYTES_PER_WORD as f64 * (STREAM_ARRAY_SIZE as f64 / 1024.0 / 1024.0),
        BYTES_PER_WORD as f64 * (STREAM_ARRAY_SIZE as f64 / 1024.0 / 1024.0 / 1024.0)
    );
    println!(
        "Total memory required = {:.1} MiB (= {:.1} GiB).\n",
        (3.0 * BYTES_PER_WORD as f64) * (STREAM_ARRAY_SIZE as f64 / 1024. / 1024.),
        (3.0 * BYTES_PER_WORD as f64) * (STREAM_ARRAY_SIZE as f64 / 1024. / 1024. / 1024.)
    );
    println!("Each kernel will be executed {} times.", N_TIMES);
    println!(" The *best* time for each kernel (excluding the first iteration)");
    println!(" will be used to compute the reported bandwidth.");
    println!("{}", H_LINE);

    quantum = checktick();
    if quantum >= 1 {
        println!(
            "Your clock granularity/precision appears to be {} microseconds.",
            quantum
        );
    } else {
        println!("Your clock granularity appears to be less than one microsecond.");
        quantum = 1;
    }

    t = Instant::now();
    unsafe {
        STREAM_ARRAY_A.iter_mut().for_each(|x| *x = 2);
    }
    let order_of_dur = (Instant::now() - t).as_micros();
    println!(
        "Each test below will take on the order of {} microseconds.",
        order_of_dur
    );
    println!("   (= {} clock ticks)", order_of_dur as i32 / quantum);
    println!("Increase the size of the arrays if this shows that");
    println!("you are not getting at least 20 clock ticks per test.");

    println!("{}", H_LINE);

    println!("WARNING -- The above is only a rough guideline.");
    println!("For best results, please be sure you know the");
    println!("precision of your system timer.");
    println!("{}", H_LINE);

    /*	--- MAIN LOOP --- repeat test cases NTIMES times --- */
    for k in 0..N_TIMES {
        times[k] = Instant::now();
        unsafe {
            for j in (0..STREAM_ARRAY_SIZE).rev() {
                STREAM_ARRAY_A[j] = STREAM_ARRAY_B[j] + SCALAR * STREAM_ARRAY_C[j];
            }
        }
        durations[k] = Instant::now() - times[k];
    }

    /*	--- SUMMARY --- */

    for k in 1..N_TIMES
    /* note -- skip first iteration */
    {
        avgtime = avgtime + durations[k].as_secs_f64();
        mintime = mintime.min(durations[k].as_secs_f64());
        maxtime = maxtime.max(durations[k].as_secs_f64());
    }

    println!("Function    Best Rate MB/s  Avg time     Min time     Max time     Access Times     Avg Time per Access");

    avgtime = avgtime / (N_TIMES - 1) as f64;

    println!(
        "{}{:12.1}  {:11.6}  {:11.6}  {:11.6}    {:11.6e}        {:11.6e}",
        LABEL,
        1.0e-06 * BYTES as f64 / mintime,
        avgtime,
        mintime,
        maxtime,
        (3 * STREAM_ARRAY_SIZE * N_TIMES),
        (avgtime / (3 * STREAM_ARRAY_SIZE * N_TIMES) as f64)
    );
    println!("{}", H_LINE);

    /* --- Check Results --- */
    // check_stream_results();
    println!("{}", H_LINE);
}

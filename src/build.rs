use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn generate_code<W: std::io::Write>(f: &mut W, output: &str) -> std::io::Result<()> {
    let mut calls: Vec<(&str, u64)> = output
        .lines()
        .map(|line| {
            let (call, nr) = line
                .split_once(' ')
                .expect("correct output from extract.sh");
            assert!(
                call.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
                "correct syscall name"
            );
            let nr = nr.parse::<u64>().expect("valid syscall number");
            (call, nr)
        })
        .collect();

    let num_calls = calls.len();
    assert_ne!(num_calls, 0, "expected non-zero number of syscalls");

    calls.sort_by_key(|(_call, nr)| *nr);

    writeln!(
        f,
        "/// All syscalls and their numbers, sorted ascending by number."
    )?;
    writeln!(f, "pub static SYSCALLS: [(&str, u64); {num_calls}] = [")?;
    for (call, nr) in &calls {
        writeln!(f, "    (\"{call}\", {nr}),")?;
    }
    writeln!(f, "];")?;
    writeln!(f)?;

    writeln!(f, "/// Try to find the syscall corresponding to a number")?;
    writeln!(
        f,
        "pub fn syscall_of_nr(nr: u64) -> Option<&'static str> {{"
    )?;
    writeln!(f, "    match nr {{")?;
    for (call, nr) in &calls {
        writeln!(f, "        {nr} => Some(\"{call}\"),")?;
    }
    writeln!(f, "        _ => None,")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    calls.sort_by_key(|(call, _nr)| *call);

    writeln!(f, "/// Try to find the number corresponding to a syscall")?;
    writeln!(f, "pub fn nr_of_syscall(call: &str) -> Option<u64> {{")?;
    writeln!(f, "   match call {{")?;
    for (call, nr) in &calls {
        writeln!(f, "         \"{call}\" => Some({nr}),")?;
    }
    writeln!(f, "        _ => None,")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;

    Ok(())
}

fn main() {
    let extract = Command::new("utils/extract.sh")
        .output()
        .expect("run utils/extract.sh");

    if !extract.status.success() {
        println!("cargo:error=could not extract syscalls from syscall.h");
        println!("cargo:error={:?}", extract.stderr);
        return;
    }

    let output = String::from_utf8_lossy(&extract.stdout);

    let out_dir = env::var("OUT_DIR").expect("output directory for build.rs-generated code");
    let dest_path = Path::new(&out_dir).join("syscall_table.rs");
    let mut f = File::create(&dest_path).expect("create syscall_table.rs");

    generate_code(&mut f, &output).expect("generate syscall_table.rs");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=utils/extract.sh");
}

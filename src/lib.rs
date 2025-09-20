include!(concat!(env!("OUT_DIR"), "/syscall_table.rs"));

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    #[test]
    fn nr_roundtrip() {
        for (call, nr) in SYSCALLS {
            let call2 = syscall_of_nr(nr).unwrap();
            assert_eq!(call, call2);
            let nr2 = nr_of_syscall(call2).unwrap();
            assert_eq!(nr, nr2);
        }
    }

    #[test]
    fn call_roundtrip() {
        for (call, nr) in SYSCALLS {
            let nr2 = nr_of_syscall(call).unwrap();
            assert_eq!(nr, nr2);
            let call2 = syscall_of_nr(nr).unwrap();
            assert_eq!(call, call2);
        }
    }

    #[test]
    fn libc_geteuid_spot_check() {
        let libc_geteuid = Command::new("utils/find_geteuid.sh")
            .output()
            .expect("run utils/find_geteuid.sh");

        assert!(libc_geteuid.status.success());

        let geteuid_nr = nr_of_syscall("geteuid").unwrap();
        let libc_assembly = String::from_utf8(libc_geteuid.stdout).unwrap();

        let nr_decimal = format!("{geteuid_nr}");
        let nr_hex = format!("{geteuid_nr:x}");

        assert!(libc_assembly.contains(&nr_decimal) || libc_assembly.contains(&nr_hex));
    }
}

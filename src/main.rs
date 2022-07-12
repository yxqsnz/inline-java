macro_rules! inline_java {
        ($java_file_class_name:expr => $($content:tt)*) => {{
        let data = stringify!($($content)*);

        std::fs::write(concat!(stringify!($java_file_class_name), ".java"), data.as_bytes()).unwrap();

        let command = std::process::Command::new("javac")
                    .arg(concat!(stringify!($java_file_class_name), ".java")).output();


        let _ = std::fs::remove_file(concat!(stringify!($java_file_class_name), ".java"));
        let res = command.unwrap();

        assert!(res.status.success());
        let command = std::process::Command::new("java")
                    .arg(concat!(stringify!($java_file_class_name))).output();


        let _ = std::fs::remove_file(concat!(stringify!($java_file_class_name), ".class"));
        let res = command.unwrap();
        assert!(res.status.success());

        let mut stdout = String::from_utf8(res.stdout).unwrap();
        let stderr = String::from_utf8(res.stderr).unwrap();
        stdout.push_str(&stderr);
        stdout
    }}
}
fn main() {
    let out = inline_java! {
        Hello =>
            public class Hello {

                /* This is my first java program.
                 * his will print 'Hello World' as the output
                 */

                public static void main(String []args) {
                    System.out.println("Hello World"); // prints Hello World
                }
            }
    };
    println!("==> {out}");
}

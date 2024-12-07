
    use std::io::Write;


    fn main() -> std::io::Result<()> {

        let prog_name = "test";

        let mut writer = std::io::stdout().lock();

        match writeln!(&mut writer, "usage: {} <script_file_name>, [whinge]", prog_name) {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e)
        }
        
        Ok(())
    }


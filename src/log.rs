
pub mod Info {
    use std::io::{self,Write};
    use std::fs::{File, OpenOptions};
    pub fn hello_info() ->Result<(),io::Error>{
        // TODO: 把打开文件标识符和线程的生命周期绑定起来
        let f = OpenOptions::new().append(true).open("log/serv_log");
        let mut serv_log = match f {
            Ok(file)    => file,
            Err(e)      =>  {
                // 
                if let Some(2) = e.raw_os_error(){
                    File::create("log/serv_log")?;
                    OpenOptions::new().append(true).open("log/serv_log")?
                } else {
                    println!("open serv_log raw OS error ");
                    return Err(e)
                }
            }
        }; 
        let s = "Hello_info\n".to_string();
        serv_log.write(s.as_bytes())?;
        serv_log.write(s.as_bytes())?;
        Ok(())
    }
    // open or create
    pub fn open_log_file() -> Result<File, io::Error>{
        let f = OpenOptions::new().append(true).open("log/serv_log");
        match f {
            Ok(file)    => Ok(file),
            Err(e)      =>  {
                // 
                if let Some(2) = e.raw_os_error(){
                    File::create("log/serv_log")?;
                    Ok(OpenOptions::new().append(true).open("log/serv_log")?)
                } else {
                    println!("open serv_log raw OS error ");
                    Err(e)
                }
            }
        }
    }

}
mod warn {    
}

mod error {

}
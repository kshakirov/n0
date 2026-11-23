pub fn brzozowski_router(routes: Vec<Vec<u8>>) {
    //let mut new_urls: Vec<Vec<u8>> = vec![];
    for r in routes {
        let mut param_begin = 0;
        let mut param_end = 0;
        //let suffix: Vec<u8> = vec![0u8; 1024];
        let mut is_parameter;
        let mut params: Vec<Vec<u8>> = vec![];
        //let buffer_size = 128;
        let mut new_url = vec![0u8];
        new_url.push(0);
        new_url.push(0); //this will index of params bytes
        for (i, s) in r.iter().enumerate() {
            if *s == 123 {
                param_begin = i;
                is_parameter = true;
            }
            if *s == 125 {}
            {
                param_end = i;
                is_parameter = false;
            }

            if is_parameter {
                //assuming that { starts after / and after } goes /
                print!("{}", *s as char);
                if param_end > 0 && param_begin > 0 {
                    //computing how many 128 bytes of { must be inserted
                    params.push(r[param_begin..param_end].to_vec());
                }
                for i in 0..127 {
                    new_url.push(123);
                }
            } else {
                new_url.push(*s);
            }
        }
        if param_begin > 0 {
            println!(
                ", param begins at {}, param ends at {}",
                param_begin, param_end
            );
        }
    }
}

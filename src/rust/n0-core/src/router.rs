pub fn brzozowski_router(routes: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_urls: Vec<Vec<u8>> = vec![];
    for r in routes {
        let mut param_begin = 0;
        let mut param_end;
        let mut is_parameter = false;
        //let suffix: Vec<u8> = vec![0u8; 1024];
        let mut params: Vec<Vec<u8>> = vec![];
        //let buffer_size = 128;
        let mut new_url = vec![0u8];
        new_url.push(0);
        new_url.push(0); //this will index of params bytes
        for (i, s) in r.iter().enumerate() {
            match *s {
                123 => {
                    param_begin = i;
                    is_parameter = true;
                }
                125 => {
                    param_end = i;
                    //assuming that { starts after / and after } goes /
                    print!("{}", *s as char);
                    if param_end > 0 && param_begin > 0 {
                        //computing how many 128 bytes of { must be inserted
                        params.push(r[param_begin..param_end].to_vec());
                    }
                    for _i in 1..128 {
                        new_url.push(123);
                    }
                    if param_begin > 0 {
                        println!(
                            ", param begins at {}, param ends at {}",
                            param_begin, param_end
                        );
                    }
                    is_parameter = false;
                }
                _ => {
                    if !is_parameter {
                        new_url.push(*s);
                    }
                }
            }
        }
        let l_n = new_url.len();
        new_url[0] = (l_n << 8) as u8;
        new_url[1] = l_n as u8;

        new_urls.push(new_url);
    }
    new_urls
}

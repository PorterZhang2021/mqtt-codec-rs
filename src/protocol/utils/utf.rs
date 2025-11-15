pub(crate) mod utf_8_handler {
    
}

#[cfg(test)]
mod utf_8_tests {
    // todo read 2 byte to get length
    // todo read 2 byte to get length then decode utf-8 string

    // this case need filter invalid utf-8
    // todo decode utf-8 can not allowed U+D800 to U+DFFF
    // todo decode utf-8 can not allowed U+0000
    // todo decode utf-8 can not allowed U+0001 to U+001F
    // todo decode utf-8 can not allowed U+007F to U+009F
    // todo decode utf-8 can not allowed U+FDD0 to U+FDEF
    // todo decode utf-8 receive 0XEF 0XBB 0XBF to U+EFFF
}
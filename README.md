# faf-http-date (no_std)

Quickly (~20ns) generate a date for an HTTP header, formatted to be fully compliant with RFCs 822/1123/2616. This is used in the [faf web server](https://github.com/errantmind/faf)

This crate is simple, you pass a buffer to the provided function. The function populates the buffer with the date

* Why use a buffer? It is faster.
* Why pass a buffer instead of just having the date function return one? It is faster.
* Why bytes instead of a string/str? It is faster and bytes are what you will write to the TCP socket, not a string. If you want a string, you can convert the buffer to a `&str` or `String`. See the examples below

This is a heavily optimized, stripped, and otherwise modified version of [pyfisch/httpdate](https://github.com/pyfisch/httpdate)

## Examples

// Format: b"Thu, 01 Jan 1970 00:00:00 GMT"
```
let mut buf = faf_http_date::get_date_buff_no_key();
faf_http_date::get_date_no_key(&mut buf);

// Optional, convert to str
let date_str = unsafe { std::str::from_utf8_unchecked(&buf[..]) };
```

Format: b"Date: Thu, 01 Jan 1970 00:00:00 GMT" (notice the 'Date: ' at the first)
```
let mut buf = faf_http_date::get_date_buff_with_key();
faf_http_date::get_date_with_key(&mut buf);

// Optional, convert to str
let date_str = unsafe { std::str::from_utf8_unchecked(&buf[..]) };
```

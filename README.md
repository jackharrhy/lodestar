# ✭ lodestar

---

`main.rs`

```rust
use std::thread;
use std::io::{self, Write};
use std::net::{TcpListener};

static RESPONSE: &'static [u8] = include_bytes!("response.txt");

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1024")?;
    for stream in listener.incoming() {
        thread::spawn(move || {
            match stream.ok() {
                Some(mut stream) => stream.write(RESPONSE).ok(),
                None => None,
            }
        });
    }
    Ok(())
}
```

`response.txt`

```txt
HTTP/1.1 200 OK
Content-Type: text/html;

<!DOCTYPE html>
<html>
<head>
<title>lodestar</title>
<style>
body {
  font-family: monospace;
  text-align: center;
  background-color: #111;
}
p {
  text-align: center;
  font-size: 10em;
  color: #f7ed5f;
}
</style>
</head>
<body>
<p>&#10029;</p>
</body>
</html>
```

---

```sh
cargo run &
telnet localhost 1024
```

```txt
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
HTTP/1.1 200 OK
Content-Type: text/html;

<!DOCTYPE html>
<html>
<head>
<title>lodestar</title>
<style>
body {
  font-family: monospace;
  text-align: center;
  background-color: #111;
}
p {
  text-align: center;
  font-size: 10em;
  color: #f7ed5f;
}
</style>
</head>
<body>
<p>&#10029;</p>
</body>
</html>
Connection closed by foreign host.
```

---

```sh
surf localhost:1024
```

![lodestar's default response page rendered in the surf browser](surf_view.png)


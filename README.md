# WHAT-I-WANT

Some tools to help with the return value.

### Reduce using "if"

```rust
use what_i_want::*;

fn login(username: String) -> bool {
    require!(username == "admin", false);
    ...
}

fn login2(username: String) {
    require!(username == "admin");
    ...
}
```

### Handling some nested Result and Option

Before using `what_i_want`

```rust
pub async fn get_mutipart_data(mut mutipart_data: Multipart) -> MultipartData {
    // Nested hell, and different Enum (Result, Option) handling
    // Of course this code is just for demonstration
    while let Some(Ok(mut field)) = mutipart_data.next().await {
        if let Some(disposition) = field.headers().get(&header::CONTENT_DISPOSITION) {
            if let Ok(disposition_str) = disposition.to_str() {
                if let Some(dis) = ContentDisposition::parse(disposition_str) {
                    if let Some(key) = dis.name {
                        while let Some(Ok(chunk)) = field.next().await {
                            ...
                        }
                    }
                }
            }
        }
    }
    MultipartData { ... }
}
```

After using `what_i_want`

```rust
use what_i_want::*;

async fn get_mutipart_data(mut mutipart_data: Multipart) -> MultipartData {
    while let Some(Ok(mut field)) = mutipart_data.next().await {
        let disposition = unwrap_or_continue!(field.headers().get(&header::CONTENT_DISPOSITION));
        let disposition_str = unwrap_or_continue!(disposition.to_str());
        let dis = unwrap_or_continue!(ContentDisposition::parse(disposition_str));
        let key = unwrap_or_continue!(dis.name);
        while let Some(Ok(chunk)) = field.next().await {
            ...
        }
    }
    MultipartData { ... }
}
```

### Can be used by any enum that implements `WhatIwant`

```rust
use what_i_want::*;

enum LoginReply {
    Success,
    Failed(i32)
}

impl WhatIwant for LoginReply {
    fn is_i_want(&self) -> bool {
        match self {
            LoginReply::Success => true,
            _ => false
        }
    }
}

fn handle(reply: LoginReply) -> () {
    let re = unwrap_or_return!(reply);
    // Do something
    ...
}
```

## Macros

```rust
macro_rules! unwrap_or_do {
    ($exp: expr, $do: expr) => {
        if $exp.is_i_want() {
            $do
        } else {
            $exp.unwrap()
        }
    };
}

macro_rules! unwrap_or_continue {
    ($exp: expr) => {
        unwrap_or_do!($exp, continue)
    };
}

macro_rules! unwrap_or_return {
    ($exp: expr) => {
        unwrap_or_do!($exp, return)
    };
}


macro_rules! unwrap_or_false {
    ($exp: expr) => {
        unwrap_or_do!($exp, return false)
    };
}

macro_rules! unwrap_or_true {
    ($exp: expr) => {
        unwrap_or_do!($exp, return true)
    };
}

macro_rules! unwrap_or_val {
    ($exp: expr, $val: expr) => {
        unwrap_or_do!($exp, return $val)
    };
}

macro_rules! require {
    ($condition: expr) => {
        if !$condition {
            return;
        }
    };
    ($condition: expr, $return: expr) => {
        if !$condition {
            return $return;
        }
    };
}
```

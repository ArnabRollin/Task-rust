# Task-rust
Task-rust is a simple task manager (like `npm run`) which can be used to shorten commands significantly. The tasks are stored in a `tasks.json` file.

### Examples
```jsonc
// tasks.json
[
  {
    "name": "greet",
    "command": "echo Hello"
  }
]
```

```console
% task greet
Hello
% task greet user
Hello user
```

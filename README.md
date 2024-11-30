# Rust Port Scanner

A simple, multi-threaded port scanner built in Rust that scans for open ports on a given IP address using a specified number of threads.

## Usage

```bash
cargo run -- -j <threads> <ip_address>
```

### Example
```bash
cargo run -- -j 1000 192.168.1.1
```

- `-j`: Specifies the number of threads to use.  
- `<threads>`: Number of threads (e.g., 1000).  
- `<ip_address>`: Target IP address to scan (e.g., 192.168.1.1).

## Output
- Displays a dot (`.`) for each scanned port.  
- Lists the open ports after scanning is complete.

## Features
- **Multi-threaded scanning**: Leverages threads to scan ports in parallel for efficiency.  
- **Supports both IPv4 and IPv6**: Accepts both IP versions.  
- **Customizable thread count**: Use the `-j` flag to control the number of threads.

## Error Handling
- If invalid arguments are provided, the program will display an error message and exit.
- For help, use:
  ```bash
  cargo run -- -h
  ```

## Additional Notes
- The scanner targets ports from `1` to `65535`.
- Scans start from a given port and increment based on the number of threads.
- The program flushes output immediately to keep track of progress.

# Orbbec Rust Examples

This directory contains example programs demonstrating how to use the orbbec-sdk Rust library.  
The examples are organized by increasing complexity:

1. **Device Info**  
   ```bash
   cargo run --release --example device_info
   ```  
   Lists all connected Orbbec devices along with their detailed information.

2. **Depth**  
   ```bash
   cargo run --release --example depth
   ```  
   Configures and captures a basic depth stream.

3. **Depth Filtered**  
   ```bash
   cargo run --release --example depth_filtered
   ```  
   Applies multiple available filters to a depth stream.

4. **Depth Aligned**  
   ```bash
   cargo run --release --example depth_aligned
   ```  
   Aligns a depth stream with a corresponding color stream.

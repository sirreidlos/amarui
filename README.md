# amarui

- [x] Boots
- [x] Basic VGA output

- [ ] Memory Management
    - [x] Physical frame allocator
    - [x] Virtual memory paging
    - [ ] Heap allocator

- [ ] Interrupts and Exceptions
    - [x] IDT setup
    - [x] Timer interrupt
    - [x] Keyboard interrupt
    - [x] Page fault exception
    - [ ] GPF / division by zero handler

- [ ] Basic Drivers
    - [ ] PS/2 keyboard
    - [ ] Timer (e.g., PIT or HPET)

- [ ] User Mode
    - [ ] Ring 3 switch
    - [ ] Set up user stack and code segment
    - [ ] Return to kernel on syscall or trap

- [ ] Executable Loading
    - [ ] ELF parser
    - [ ] Load segments into user memory
    - [ ] Set instruction pointer and jump

- [ ] Scheduler & Multitasking
    - [ ] Basic task struct (registers, memory)
    - [ ] Context switching
    - [ ] Preemptive or cooperative scheduling

- [ ] Shell
    - [ ] Basic input loop
    - [ ] Parse commands
    - [ ] Execute user binaries

- [ ] File System
    - [ ] Block device driver (e.g. ATA or RAM disk)
    - [ ] Read directory/file structure (e.g., FAT, ext2)
    - [ ] Basic file read syscall

- [ ] Networking
    - [ ] NIC driver (e.g., RTL8139, e1000)
    - [ ] ARP/IP stack
    - [ ] Send/receive packets

- [ ] IPC
    - [ ] Pipes, message queues, or shared memory
    - [ ] Syscalls for communication between processes

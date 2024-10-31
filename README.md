# qwOS
QwOS is the working title of the operating system kernel I am developing to hone my knowledge of the hardware-software interface.

QwOS came about after writing [lilOS](https://github.com/elchukc/lilOS/) and realizing I wanted more. It was also a great vehicle to develop my skills in Rust, the way lilOS was a vehicle to hone my skills in C. In some ways, this kernel is a mark 2 to lilOS. It will use [Philipp Oppermann's incredible blog](https://os.phil-opp.com/) to get the basic functionality down, then grow further from there.

### So far...
It has a bootloader, the beginnings of a standard library, printing to screen using the VGA buffer,  hardware interrupt handling, and keyboard interrupt handling. Still todo is memory management, including a heap and paging.

## Hardware details
Currently developed only for the x86_64 architecture, but the plan is to support other platforms.
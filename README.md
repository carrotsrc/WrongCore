# "WrongCore"
#### or "Experiments in making the kernel cry"

This is a hobby module for hacking rust code into the Linux kernel, in a vaguely practical though ineffecient way. A truely heinous act.

* Dynamic mutex allocation and locking/unlocking test performed on kernal init routine (Can't do compile-time mutex definition because it's all done through macro wizardry)
* WrongCore now functions as a rust module, and links correctly in the final kbuild


#### Log

**24/06**

The kernel has had jitters on every other build because the relocation type has been based on dynamic linking. After looking, kbuild was not lying to me -- everything was based on global offset table; changing the relocation-model to static has resolved this problem. This is done via the codegen flag

Interestingly the kernel print method was trashing the pointer to the mutex. It seemed that the pointer value was stored in the stack but not retrieved again, then the `kernel::print()` was overwriting the eax register with the return and then putting that value into the stack position to be read by the `il_mutex_init()`. It was also causing some other strange behaviour, which I was calling stack-ladder, where the eax value would move down the stack from location, back to eax, to location, back to eax and so on.

In the end, removing the return value has aligned everything again and it is back to functional. Kernel panics have stopped! (... for now ...)

rlib does actually need to be compiled - got caught out by the `kernel::print()` signature being completely different to the one that was being compiled into the object code for linking. It was as if it was cached somewhere and the cache wasn't getting updated. The problem, as far as I can tell, was the rlib that was used as reference wasn't getting updated. Back to being a bit of a hack solution. 

**22/06**

Fixed the rlib problem by shoving wrcore into its own directory which avoids having to compile it twice! Wrongcore now works with namespaces and links correctly so to print to the kernel log you can use `wrcore::kernel::print()`.

Add no-stack-check on the flag, which removed the split stack prologues and also negates the requirement of using a __morestack routine.

When there is no `wrcore::kernel::print()` in the mutex test, the pointer passed to `il_mutex_init()` is the same address. If there is a print(), the pointer address that is passed to `il_mutex_init()` is consistantly 0x1a and so the kernel blows up. Something about the print trashes the value assigned to the mutex pointer.


**21/06**

After messing around, there is a horrible hack for making the crate work without complaints- build the crate twice: as rlib and obj file; compile against the lib but link against the obj in the final kbuild. Needs more research, there is undoubtedly a better way to achieve this.



#### Why?

Just, because.



### License

GPLv2

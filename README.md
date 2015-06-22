# "WrongCore"
#### or "Experiments in making the kernel cry"

This is a hobby module for hacking rust code into the Linux kernel, in a vaguely practical though ineffecient way. A truely heinous act.

* `Dodgy:` Dynamic mutex allocation and locking/unlocking test performed on kernal init routine (Can't do compile-time mutex definition because it's all done through macro wizardry)
* WrongCore now functions as a rust module, and links correctly in the final kbuild
* Kernel blowing up due to not properly doing anything with __morestack

#### Log

**22/06**

Fixed the rlib problem by shoving wrcore into its own directory which avoids having to compile it twice! Wrongcore now works with namespaces and links correctly so to print to the kernel log you can use `wrcore::kernel::print()`.

Now been stung by not properly looking at how `__morestack` works. Any kernel API call from rust code is trashed after a call to a function with a prologue.

**21/06**

After messing around, there is a horrible hack for making the crate work without complaints- build the crate twice: as rlib and obj file; compile against the lib but link against the obj in the final kbuild. Needs more research, there is undoubtedly a better way to achieve this.



#### Why?

Just, because.



### License

GPLv2

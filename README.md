# "WrongCore"
#### or "Experiments in making the kernel cry"

This is a hobby module for hacking rust code into the Linux kernel, in a vaguely practical though ineffecient way. A truely heinous act.

* Dynamic mutex allocation and locking/unlocking test performed on kernal init routine (Can't do compile-time mutex definition because it's all done through macro wizardry)


21/06:

After messing around, there is a horrible hack for making the crate work without complaints- build the crate twice: as rlib and obj file; compile against the lib but link against the obj in the final kbuild. Needs more research, there is undoubtedly a better way to achieve this.



#### Why?

Just, because.



### License

GPLv2

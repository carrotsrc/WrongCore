$(obj)/integration.o: $(src)/integration.s
	$(AS) --32 $(src)/integration.s -o $(obj)/integration.o


$(obj)/wrcore.o: $(src)/wrongcore/wrcore.rs $(src)/wrongcore/kernel.rs
	rustc --target=i686-unknown-linux-gnu $(src)/wrongcore/wrcore.rs
	rustc -C no-stack-check -C relocation-model=static --emit=obj --target=i686-unknown-linux-gnu $(src)/wrongcore/wrcore.rs -o $(obj)/wrcore.o

$(obj)/wr_mutex.o: $(src)/wr_mutex.rs
	rustc -C no-stack-check -C relocation-model=static --target=i686-unknown-linux-gnu --emit obj -L . $(src)/wr_mutex.rs -o $(obj)/wr_mutex.o

$(obj)/ikif_cpu.o: $(src)/ikif_cpu.rs
	rustc -C no-stack-check -C relocation-model=static --target=i686-unknown-linux-gnu --emit obj -L . $(src)/ikif_cpu.rs -o $(obj)/ikif_cpu.o


obj-y=il_mutex.o il_cpuflags.o wrcore.o wr_mutex.o ikif_cpu.o

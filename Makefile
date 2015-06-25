$(obj)/integration.o: $(src)/integration.s
	$(AS) --32 $(src)/integration.s -o $(obj)/integration.o


$(obj)/wrcore.o: $(src)/wrongcore/wrcore.rs $(src)/wrongcore/kernel.rs $(src)/wrongcore/lang/ops.rs
	./rskbuild crate $(src)/wrongcore/wrcore.rs
	./rskbuild obj $(src)/wrongcore/wrcore.rs -o $(obj)/wrcore.o

$(obj)/wr_mutex.o: $(src)/wr_mutex.rs
	./rskbuild obj $(src)/wr_mutex.rs -o $(obj)/wr_mutex.o

$(obj)/ikif_cpu.o: $(src)/ikif_cpu.rs
	./rskbuild obj $(src)/ikif_cpu.rs -o $(obj)/ikif_cpu.o


obj-y=il_mutex.o il_cpuflags.o wrcore.o wr_mutex.o ikif_cpu.o

$(obj)/integration.o: $(src)/integration.s
	$(AS) --32 $(src)/integration.s -o $(obj)/integration.o


$(obj)/wrcore.o: $(src)/wrongcore/wrcore.rs
	rustc -C no-stack-check --emit=obj --target=i686-unknown-linux-gnu $(src)/wrongcore/wrcore.rs -o $(obj)/wrcore.o

$(obj)/wr_mutex.o: $(obj)/wrcore.o $(src)/wr_mutex.rs
	rustc -C no-stack-check --target=i686-unknown-linux-gnu --emit obj -L . $(src)/wr_mutex.rs -o $(obj)/wr_mutex.o




obj-y=integration.o il_mutex.o wrcore.o wr_mutex.o

$(obj)/integration.o: $(src)/integration.s
	$(AS) --32 $(src)/integration.s -o $(obj)/integration.o


$(obj)/wr_mutex.o: $(src)/wr_mutex.rs
	rustc --target=i686-unknown-linux-gnu --emit obj --crate-type lib $(src)/wr_mutex.rs -o $(obj)/wr_mutex.o

obj-y=integration.o il_mutex.o wr_mutex.o

$(obj)/integration.o: $(src)/integration.s
	$(AS) --32 $(src)/integration.s -o $(obj)/integration.o

# Need to build wrcore twice, once as a lib so rustc doesn't complain
# on other compiles, and once as a obj to link in to the final build
$(obj)/wrcore.o: $(src)/wrcore.rs
	rustc --emit=obj --target=i686-unknown-linux-gnu $(src)/wrcore.rs -o $(obj)/wrcore.o
	rustc --target=i686-unknown-linux-gnu $(src)/wrcore.rs

$(obj)/wr_mutex.o: $(obj)/wrcore.o $(src)/wr_mutex.rs
	rustc --target=i686-unknown-linux-gnu --emit obj -L . $(src)/wr_mutex.rs -o $(obj)/wr_mutex.o

obj-y=integration.o il_mutex.o wrcore.o wr_mutex.o


struct klp_func *il_create_patch_func(const char* old_name, void* new_func, unsigned long old_addr) {
	return (klp_func*) kmalloc(sizeof(struct klp_func), GFP_KERNEL);
	
}

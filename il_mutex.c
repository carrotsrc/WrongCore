#include <linux/mutex.h>
#include <linux/slab.h>

extern void il_mutex_init(struct mutex *mutex) {
	mutex_init(mutex);
}

extern struct mutex* il_mutex_allocate(void) {
	struct mutex* m = (struct mutex*) kmalloc(sizeof(struct mutex), GFP_KERNEL);
	if(m == NULL) {
		printk("IF Failed to allocate mutex\n");
	} else {
		printk("IF Allocated Mutex\n");
	}

	return m;
}

/* This is inline usually, so we need to implement a
 * layer of wrongness
 */
extern int il_mutex_is_locked(struct mutex *m) {
	return mutex_is_locked(m);
}

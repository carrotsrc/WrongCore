#include <linux/mutex.h>
#include <linux/slab.h>

extern void il_mutex_init(struct mutex *mutex) {
	printk("Called successfully");
	return;
//	mutex_init(mutex);
}

extern struct mutex* il_mutex_allocate(void) {
	return (struct mutex*) kmalloc(sizeof(struct mutex), GFP_KERNEL);

}

/* This is inline usually, so we need to implement a
 * layer of wrongness
 */
extern int il_mutex_is_locked(struct mutex *m) {
	return mutex_is_locked(m);
}

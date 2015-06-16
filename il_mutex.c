#include <linux/mutex.h>

extern struct mutex il_create_mutex(void) {
	DEFINE_MUTEX(TmpMutex);

	return TmpMutex;
}

/* This is inline usually, so we need to implement a
 * layer of wrongness
 */
extern int il_mutex_is_locked(struct mutex *m) {
	return mutex_is_locked(m);
}

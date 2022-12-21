#ifndef __CORE_PROBE_USER_BPF_COMMON__
#define __CORE_PROBE_USER_BPF_COMMON__

#include <vmlinux.h>
#include <bpf/bpf_helpers.h>

#include "events.h"

enum userspace_event_type {
	USDT = 1,
};

/* Userspace section of the event data. */
struct user_event {
	u64 symbol;
	u64 pid;
	u8  event_type;
} __attribute__((packed));

#define DEFINE_USDT_HOOK(inst)							\
	SEC("ext/hook")								\
	int hook(struct pt_regs *ctx, struct trace_raw_event *event)		\
	{									\
		/* Let the verifier be happy */					\
		if (!ctx || !event)						\
			return 0;						\
		inst								\
	}

#endif // __CORE_PROBE_USER_BPF_COMMON

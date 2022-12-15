#ifndef __CORE_PROBE_USER_BPF_COMMON__
#define __CORE_PROBE_USER_BPF_COMMON__

#include <vmlinux.h>
#include <bpf/bpf_helpers.h>

#include "events.h"

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

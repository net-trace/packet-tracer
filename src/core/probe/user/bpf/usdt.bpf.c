#include <vmlinux.h>
#include <bpf/bpf_helpers.h>
#include <bpf/usdt.bpf.h>

#include <user_common.h>

/* Hook placeholder */
__attribute__ ((noinline))
int hook0(struct pt_regs *ctx, struct trace_raw_event *event) {
	volatile int ret = 0;
	if (!ctx || !event)
		return 0;
	return ret;
}

SEC("usdt")
int probe_usdt(struct pt_regs *ctx)
{
    struct pt_regs ctx_fp = *ctx;
	struct common_event *e;
	struct trace_raw_event *event;
	struct user_event *u;

	event = get_event();
	if (!event)
		return 0;

	e = get_event_section(event, COMMON, 1, sizeof(*e));
	if (!e) {
		discard_event(event);
		return 0;
	}
	e->timestamp = bpf_ktime_get_ns();

	u = get_event_section(event, USERSPACE, 1, sizeof(*u));
	if (!u) {
		discard_event(event);
		return 0;
	}
	u->symbol = PT_REGS_IP(ctx);
	u->pid = bpf_get_current_pid_tgid();
	u->event_type = USDT;

    /* UST only supports a single hook. */
    hook0(&ctx_fp, event);

	send_event(event);

	return 0;
}

char __license[] SEC("license") = "GPL";

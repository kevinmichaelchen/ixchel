<script lang="ts">
  import type { Status } from '$lib/types/issue';
  import { Badge } from '$lib/components/ui/badge';

  interface Props {
    status: Status;
    class?: string;
  }

  let { status, class: className = '' }: Props = $props();

  const statusConfig: Record<
    Status,
    { label: string; variant: 'default' | 'secondary' | 'destructive' | 'outline' }
  > = {
    open: { label: 'Open', variant: 'secondary' },
    in_progress: { label: 'In Progress', variant: 'default' },
    blocked: { label: 'Blocked', variant: 'destructive' },
    closed: { label: 'Closed', variant: 'outline' },
  };

  const config = $derived(statusConfig[status]);
</script>

<Badge variant={config.variant} class={className}>
  {config.label}
</Badge>

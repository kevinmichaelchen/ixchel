<script lang="ts">
  import { Badge } from '$lib/components/ui/badge';

  interface Props {
    priority: number | string;
    class?: string;
  }

  let { priority, class: className = '' }: Props = $props();

  const priorityConfig: Record<string, { label: string; class: string }> = {
    '0': { label: 'P0', class: 'bg-red-600 text-white hover:bg-red-700' },
    '1': { label: 'P1', class: 'bg-orange-500 text-white hover:bg-orange-600' },
    '2': { label: 'P2', class: 'bg-yellow-500 text-black hover:bg-yellow-600' },
    '3': { label: 'P3', class: 'bg-blue-500 text-white hover:bg-blue-600' },
    '4': { label: 'P4', class: 'bg-gray-500 text-white hover:bg-gray-600' },
  };

  const config = $derived(priorityConfig[String(priority)] ?? priorityConfig['2']);
</script>

<Badge variant="outline" class="{config.class} {className}">
  {config.label}
</Badge>

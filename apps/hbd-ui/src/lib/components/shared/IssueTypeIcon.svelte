<script lang="ts">
  import type { IssueType } from '$lib/types/issue';

  interface Props {
    type: IssueType;
    class?: string;
  }

  let { type, class: className = '' }: Props = $props();

  const typeIcons: Record<IssueType, { icon: string; title: string }> = {
    epic: { icon: '🎯', title: 'Epic' },
    feature: { icon: '✨', title: 'Feature' },
    task: { icon: '📋', title: 'Task' },
    bug: { icon: '🐛', title: 'Bug' },
    chore: { icon: '🔧', title: 'Chore' },
  };

  const config = $derived(typeIcons[type] ?? typeIcons.task);
</script>

<span class={className} title={config.title} role="img" aria-label={config.title}>
  {config.icon}
</span>

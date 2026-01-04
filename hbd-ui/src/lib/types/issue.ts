export type Status = 'open' | 'in_progress' | 'blocked' | 'closed';
export type IssueType = 'bug' | 'feature' | 'task' | 'epic' | 'chore';
export type DepType = 'blocks' | 'related' | 'waits_for';
export type Priority = 0 | 1 | 2 | 3 | 4;
export type CreatorType = 'human' | 'agent';

export interface Dependency {
	id: string;
	dep_type: DepType;
}

export interface Comment {
	id: string;
	body: string;
	created_at: string;
	created_by: string;
	created_by_type: CreatorType;
}

export interface Issue {
	id: string;
	title: string;
	body: string;
	status: Status;
	priority: number | string;
	issue_type: IssueType;
	created_at: string;
	updated_at: string;
	closed_at?: string;
	created_by: string;
	created_by_type: CreatorType;
	assignee?: string;
	agent_id?: string;
	session_id?: string;
	external_ref?: string;
	parent_id?: string;
	labels: string[];
	depends_on: Dependency[];
	comments: Comment[];
	estimated_minutes?: number;
}

// Note: Graph node/edge types are now handled by @xyflow/svelte's Node and Edge types

export type LayoutMode = 'hierarchical' | 'force';

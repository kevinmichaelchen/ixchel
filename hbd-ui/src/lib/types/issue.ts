export type Status = 'open' | 'in_progress' | 'blocked' | 'closed';
export type IssueType = 'bug' | 'feature' | 'task' | 'epic' | 'chore';
export type DepType = 'blocks' | 'related' | 'waits_for';
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
	priority: number;
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

export interface GraphNode {
	id: string;
	issue: Issue;
	x: number;
	y: number;
	z: number;
	vx?: number;
	vy?: number;
	vz?: number;
}

export interface GraphEdge {
	source: string | GraphNode;
	target: string | GraphNode;
	dep_type: DepType;
}

export interface GraphData {
	nodes: GraphNode[];
	edges: GraphEdge[];
}

export type LayoutMode = 'hierarchical' | 'force';

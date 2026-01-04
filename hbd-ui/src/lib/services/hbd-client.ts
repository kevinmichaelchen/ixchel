import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import * as fs from 'fs';
import type { Issue } from '$lib/types/issue';

const execAsync = promisify(exec);

export function findTicketsDir(startDir: string): string | null {
	let currentDir = startDir;

	while (currentDir !== path.dirname(currentDir)) {
		const ticketsPath = path.join(currentDir, '.tickets');
		if (fs.existsSync(ticketsPath) && fs.statSync(ticketsPath).isDirectory()) {
			return currentDir;
		}
		currentDir = path.dirname(currentDir);
	}

	return null;
}

export async function listIssues(cwd: string): Promise<Issue[]> {
	const { stdout } = await execAsync('hbd list --json', { cwd });
	return JSON.parse(stdout);
}

export async function getIssue(cwd: string, id: string): Promise<Issue> {
	const { stdout } = await execAsync(`hbd show ${id} --json`, { cwd });
	return JSON.parse(stdout);
}

export async function getBlockedIssues(
	cwd: string
): Promise<Array<{ issue: { id: string; title: string }; blocked_by: Array<{ id: string; title: string; status: string }> }>> {
	const { stdout } = await execAsync('hbd blocked --json', { cwd });
	return JSON.parse(stdout);
}

export async function getReadyIssues(cwd: string): Promise<Issue[]> {
	const { stdout } = await execAsync('hbd ready --json', { cwd });
	return JSON.parse(stdout);
}

export async function getStats(
	cwd: string
): Promise<{
	total: number;
	by_status: Record<string, number>;
	by_type: Record<string, number>;
	by_priority: Record<string, number>;
}> {
	const { stdout } = await execAsync('hbd stats --json', { cwd });
	return JSON.parse(stdout);
}

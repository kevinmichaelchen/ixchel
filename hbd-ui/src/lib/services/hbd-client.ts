import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';
import type { Issue } from '$lib/types/issue';

const execAsync = promisify(exec);

function findHbdBinary(): string | null {
	const homeDir = os.homedir();
	const possiblePaths = [
		path.join(homeDir, '.cargo', 'bin', 'hbd'),
		'/usr/local/bin/hbd',
		'/usr/bin/hbd',
		'/opt/homebrew/bin/hbd'
	];

	for (const binPath of possiblePaths) {
		if (fs.existsSync(binPath)) {
			return binPath;
		}
	}

	return null;
}

function getExecEnv(): NodeJS.ProcessEnv {
	const homeDir = os.homedir();
	const extraPaths = [
		path.join(homeDir, '.cargo', 'bin'),
		'/usr/local/bin',
		'/opt/homebrew/bin'
	];

	return {
		...process.env,
		PATH: `${extraPaths.join(':')}:${process.env.PATH || ''}`
	};
}

async function runHbd(args: string, cwd: string): Promise<string> {
	const hbdPath = findHbdBinary();
	const command = hbdPath ? `${hbdPath} ${args}` : `hbd ${args}`;

	const { stdout } = await execAsync(command, {
		cwd,
		env: getExecEnv()
	});

	return stdout;
}

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
	const stdout = await runHbd('list --json', cwd);
	return JSON.parse(stdout);
}

export async function getIssue(cwd: string, id: string): Promise<Issue> {
	const stdout = await runHbd(`show ${id} --json`, cwd);
	return JSON.parse(stdout);
}

export async function getBlockedIssues(
	cwd: string
): Promise<Array<{ issue: { id: string; title: string }; blocked_by: Array<{ id: string; title: string; status: string }> }>> {
	const stdout = await runHbd('blocked --json', cwd);
	return JSON.parse(stdout);
}

export async function getReadyIssues(cwd: string): Promise<Issue[]> {
	const stdout = await runHbd('ready --json', cwd);
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
	const stdout = await runHbd('stats --json', cwd);
	return JSON.parse(stdout);
}

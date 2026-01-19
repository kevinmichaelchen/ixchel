import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';
import type { Issue } from '$lib/types/issue';

const execAsync = promisify(exec);

function isDev(): boolean {
  return process.env.NODE_ENV === 'development' || !process.env.NODE_ENV;
}

function findMonorepoRoot(startDir: string): string | null {
  let currentDir = startDir;

  while (currentDir !== path.dirname(currentDir)) {
    const cargoTomlPath = path.join(currentDir, 'Cargo.toml');
    if (fs.existsSync(cargoTomlPath)) {
      try {
        const content = fs.readFileSync(cargoTomlPath, 'utf-8');
        if (content.includes('[workspace]') && content.includes('hbd')) {
          return currentDir;
        }
      } catch {}
    }
    currentDir = path.dirname(currentDir);
  }

  return null;
}

function findHbdBinary(): string | null {
  const homeDir = os.homedir();
  const possiblePaths = [
    path.join(homeDir, '.cargo', 'bin', 'hbd'),
    '/usr/local/bin/hbd',
    '/usr/bin/hbd',
    '/opt/homebrew/bin/hbd',
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
  const username = os.userInfo().username;
  const extraPaths = [
    `/etc/profiles/per-user/${username}/bin`,
    '/run/current-system/sw/bin',
    path.join(homeDir, '.nix-profile', 'bin'),
    path.join(homeDir, '.cargo', 'bin'),
    '/usr/local/bin',
    '/opt/homebrew/bin',
  ];

  return {
    ...process.env,
    PATH: `${extraPaths.join(':')}:${process.env.PATH || ''}`,
  };
}

let cachedMonorepoRoot: string | null | undefined = undefined;

function getMonorepoRoot(): string | null {
  if (cachedMonorepoRoot === undefined) {
    cachedMonorepoRoot = findMonorepoRoot(process.cwd());
  }
  return cachedMonorepoRoot;
}

async function runHbd(args: string, cwd: string): Promise<string> {
  const monorepoRoot = getMonorepoRoot();
  const useCargoRun = isDev() && monorepoRoot !== null;

  let command: string;
  let execCwd: string;

  if (useCargoRun) {
    command = `cargo run -p hbd --quiet -- ${args}`;
    execCwd = monorepoRoot;
  } else {
    const hbdPath = findHbdBinary();
    command = hbdPath ? `${hbdPath} ${args}` : `hbd ${args}`;
    execCwd = cwd;
  }

  const { stdout } = await execAsync(command, {
    cwd: execCwd,
    env: {
      ...getExecEnv(),
      HBD_PROJECT_DIR: cwd,
    },
  });

  return stdout;
}

export function findIxchelDir(startDir: string): string | null {
  let currentDir = startDir;

  while (currentDir !== path.dirname(currentDir)) {
    const issuesPath = path.join(currentDir, '.ixchel', 'issues');
    if (fs.existsSync(issuesPath) && fs.statSync(issuesPath).isDirectory()) {
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

export async function getBlockedIssues(cwd: string): Promise<
  Array<{
    issue: { id: string; title: string };
    blocked_by: Array<{ id: string; title: string; status: string }>;
  }>
> {
  const stdout = await runHbd('blocked --json', cwd);
  return JSON.parse(stdout);
}

export async function getReadyIssues(cwd: string): Promise<Issue[]> {
  const stdout = await runHbd('ready --json', cwd);
  return JSON.parse(stdout);
}

export async function getStats(cwd: string): Promise<{
  total: number;
  by_status: Record<string, number>;
  by_type: Record<string, number>;
  by_priority: Record<string, number>;
}> {
  const stdout = await runHbd('stats --json', cwd);
  return JSON.parse(stdout);
}

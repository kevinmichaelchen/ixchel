import type { PageServerLoad } from './$types';
import { findTicketsDir, listIssues } from '$lib/services/hbd-client';
import { DEMO_ISSUES } from '$lib/services/demo-data';
import type { Issue } from '$lib/types/issue';

export const load: PageServerLoad = async ({ url }) => {
	const demoMode = url.searchParams.get('demo') === 'true';

	if (demoMode) {
		return {
			issues: DEMO_ISSUES,
			demoMode: true,
			projectPath: null,
			error: null
		};
	}

	const projectPath = findTicketsDir(process.cwd());

	if (!projectPath) {
		return {
			issues: DEMO_ISSUES,
			demoMode: true,
			projectPath: null,
			error: 'No .tickets directory found. Running in demo mode.'
		};
	}

	try {
		const issues: Issue[] = await listIssues(projectPath);
		return {
			issues,
			demoMode: false,
			projectPath,
			error: null
		};
	} catch (e) {
		const errorMessage = e instanceof Error ? e.message : 'Failed to load issues';
		const isCommandNotFound = errorMessage.includes('command not found') || errorMessage.includes('ENOENT');
		
		const hint = isCommandNotFound
			? 'Run: cargo install --path hbd'
			: errorMessage.slice(0, 80);
		
		return {
			issues: DEMO_ISSUES,
			demoMode: true,
			projectPath,
			error: `hbd CLI unavailable. ${hint}`
		};
	}
};

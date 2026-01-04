import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';
import tsParser from '@typescript-eslint/parser';

/** @type {import('eslint').Linter.Config[]} */
export default [
  {
    files: ['**/*.svelte'],
    plugins: {
      svelte,
    },
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: tsParser,
        extraFileExtensions: ['.svelte'],
        project: './tsconfig.json',
      },
    },
    rules: {
      ...svelte.configs.recommended.rules,
      
      'svelte/no-unused-svelte-ignore': 'error',
      'svelte/valid-compile': 'error',
      'svelte/no-at-html-tags': 'warn',
      'svelte/no-target-blank': 'error',
      'svelte/no-reactive-reassign': ['error', { props: false }],
      'svelte/button-has-type': 'warn',
      'svelte/no-at-debug-tags': 'warn',
      'svelte/require-each-key': 'error',
      
      'svelte/prefer-class-directive': 'warn',
      'svelte/prefer-style-directive': 'warn',
      'svelte/shorthand-attribute': 'warn',
      'svelte/shorthand-directive': 'warn',
      'svelte/spaced-html-comment': 'warn',
    },
  },
  {
    ignores: [
      'node_modules/**',
      '.svelte-kit/**',
      'build/**',
      'dist/**',
    ],
  },
];

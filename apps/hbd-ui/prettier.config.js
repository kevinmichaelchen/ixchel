/** @type {import('prettier').Config} */
export default {
  plugins: ['prettier-plugin-svelte'],
  overrides: [
    {
      files: '*.svelte',
      options: {
        parser: 'svelte',
      },
    },
  ],
  singleQuote: true,
  trailingComma: 'es5',
  printWidth: 100,
  tabWidth: 2,
  semi: true,
  useTabs: false,
  bracketSpacing: true,
  arrowParens: 'always',
  endOfLine: 'lf',
  svelteStrictMode: false,
  svelteIndentScriptAndStyle: true,
  svelteSortOrder: 'options-scripts-markup-styles',
};

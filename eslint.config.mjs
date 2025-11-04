import antfu from '@antfu/eslint-config'

export default antfu(
  {
    gitignore: true,
    stylistic: true,
    typescript: true,
    unicorn: false,
    formatters: {
      css: true,
      html: true,
      markdown: true,
    },
    yaml: true,
    vue: false,
    react: true,
    ignores: [
      '**/dist/**',
      '**/coverage/**',
      '**/node_modules/**',
    ],
  },
  {
    rules: {
      'antfu/consistent-list-newline': 'off',
      'style/brace-style': ['error', '1tbs', { allowSingleLine: true }],

      'toml/padding-line-between-pairs': 'off',
      'toml/array-element-newline': 'off',
      'toml/array-bracket-spacing': ['error', 'never'],

      // 'ts/consistent-type-definitions': ['error', 'type'],
      'ts/consistent-type-definitions': 'off',
      'ts/consistent-type-imports': 'off',

      'curly': ['error', 'all'],

      'node/prefer-global/process': 'off',
      'node/prefer-global/buffer': 'off',
      'node/no-path-concat': 'off',

      'no-console': 'off',
    },
  },
)

// @ts-check

import eslint from '@eslint/js'
import tseslint from 'typescript-eslint'
import vueParser from 'vue-eslint-parser'
import prettierConfig from 'eslint-config-prettier'

export default tseslint.config(
  {
    ignores: [
      '**/node_modules/**/*',
      '**/dist/**/*',
      'src/script/lexer/MyParserCst.d.ts',
      'src/script/lexer/lexer.ts'
    ]
  },
  eslint.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
  {
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
        sourceType: 'module',
        extraFileExtensions: ['.vue'],
        projectService: true,
        tsconfigRootDir: import.meta.dirname
      }
    }
  },
  prettierConfig
)

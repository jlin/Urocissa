module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: [
    'plugin:vue/vue3-essential',
    'plugin:vuetify/recommended',
    'eslint:recommended',
    '@vue/eslint-config-typescript',
  ],
  rules: {
    'vue/multi-word-component-names': 'off',
    "no-multiple-empty-lines": ["error", { "max": 1, "maxEOF": 0 }],
    "no-trailing-spaces": "error",
  },
}

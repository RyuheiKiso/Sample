module.exports = [
  {
    files: ["**/*.{js,jsx,ts,tsx}"],
    languageOptions: {
      parser: require("@typescript-eslint/parser"),
      parserOptions: {
        ecmaFeatures: { jsx: true },
        ecmaVersion: "latest",
        sourceType: "module"
      }
    },
    plugins: {
      "react": require("eslint-plugin-react"),
      "@typescript-eslint": require("@typescript-eslint/eslint-plugin"),
      "import": require("eslint-plugin-import"),
      "jsx-a11y": require("eslint-plugin-jsx-a11y")
    },
    rules: {
      "react/react-in-jsx-scope": "off",
      "@typescript-eslint/no-unused-vars": ["error", { "argsIgnorePattern": "^_" }],
      "import/order": ["warn", { "groups": ["builtin", "external", "internal"], "newlines-between": "always", "alphabetize": { "order": "asc", "caseInsensitive": true } }]
    },
    settings: {
      react: {
        version: "detect"
      }
    }
  },
  require("eslint-config-prettier")
];

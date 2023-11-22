module.exports = {
  'pre-commit': 'pnpm lint-staged',
  'commit-msg': 'pnpm cross-env FORCE_COLOR=1 commitlint --edit ${1}',
};

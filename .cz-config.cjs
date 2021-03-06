module.exports = {
  types: [
    { value: ':sparkles: feat', name: 'feat:     โจ  A new feature' },
    { value: ':bug: fix', name: 'fix:      ๐  A bug fix' },
    { value: ':memo: docs', name: 'docs:     ๐  Documentation only changes' },
    {
      value: ':recycle: refactor',
      name: 'refactor: โป๏ธ   A code change that neither fixes a bug nor adds a feature',
    },
    {
      value: ':thread: style',
      name: 'style:    ๐งต  An update that changes how the app looks',
    },
    {
      value: ':zap: perf',
      name: 'perf:     โก  A code change that improves performance',
    },
    { value: ':white_check_mark: test', name: 'test:     โ  Adding missing tests' },
    {
      value: ':truck: build',
      name:
        'build:    ๐  Changes to the build process, libraries and tools',
    },
    { value: ':rewind: revert', name: 'revert:   โช  Revert to a commit' },
    { value: ':wrench: WIP', name: 'WIP:      ๐ง  Work in progress' },
  ],

  // scopes: [{ name: 'front-end' }, { name: 'back-end' }, { name: 'both' }],

  allowTicketNumber: false,
  isTicketNumberRequired: false,
  ticketNumberPrefix: 'TICKET-',
  ticketNumberRegExp: '\\d{1,5}',

  // it needs to match the value for field type. Eg.: 'fix'
  /*
  scopeOverrides: {
    fix: [
      {name: 'merge'},
      {name: 'style'},
      {name: 'e2eTest'},
      {name: 'unitTest'}
    ]
  },
  */
  // override the messages, defaults are as follows
  messages: {
    type: "Select the type of change that you're committing:",
    scope: '\nDenote the SCOPE of this change (optional):',
    // used if allowCustomScopes is true
    customScope: 'Denote the SCOPE of this change:',
    subject: 'Write a SHORT, IMPERATIVE tense description of the change:\n',
    body: 'Provide a LONGER description of the change (optional). Use "|" to break new line:\n',
    breaking: 'List any BREAKING CHANGES (optional):\n',
    footer: 'List any ISSUES CLOSED by this change (optional). E.g.: #31, #34:\n',
    confirmCommit: 'Are you sure you want to proceed with the commit above?',
  },

  allowCustomScopes: true,
  allowBreakingChanges: ['feat', 'fix'],
  // skip any questions you want
  skipQuestions: ['scope', 'footer', 'breaking'],

  // limit subject length
  // subjectLimit: 100
  // breaklineChar: '|', // It is supported for fields body and footer.
  // footerPrefix : 'ISSUES CLOSED:'
  // askForBreakingChangeFirst : true, // default is false
};
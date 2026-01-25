/*
Language: Meson
Description: Meson build system language
Website: https://mesonbuild.com/

Based on:
- Meson Syntax Reference: https://mesonbuild.com/Syntax.html
- Meson Reference Manual: https://mesonbuild.com/Reference-manual.html
- VSCode Meson Extension: https://github.com/mesonbuild/vscode-meson
*/

hljs.registerLanguage('meson', function (hljs) {
  var KEYWORDS = {
    keyword: 'if elif else endif foreach endforeach break continue and or not in',
    literal: 'true false',
    built_in: 'meson host_machine build_machine target_machine',
  };

  return {
    name: 'Meson',
    aliases: ['meson'],
    case_insensitive: false,
    keywords: KEYWORDS,
    contains: [
      hljs.COMMENT('#', '$'),
      {
        className: 'string',
        begin: "'''",
        end: "'''",
        contains: [{ className: 'subst', begin: /@[a-zA-Z_][a-zA-Z0-9_]*@|@\d+@/ }],
      },
      {
        className: 'string',
        begin: "f'''",
        end: "'''",
        contains: [{ className: 'subst', begin: /@[a-zA-Z_][a-zA-Z0-9_]*@|@\d+@/ }],
      },
      {
        className: 'string',
        begin: "f'",
        end: "'",
        illegal: /\n/,
        contains: [
          { className: 'subst', begin: /@[a-zA-Z_][a-zA-Z0-9_]*@|@\d+@/ },
          { begin: /\\[\\'"nrtabfv]/ },
        ],
      },
      {
        className: 'string',
        begin: "'",
        end: "'",
        illegal: /\n/,
        contains: [{ begin: /\\[\\'"nrtabfv]/ }],
      },
      {
        className: 'number',
        variants: [
          { begin: /\b0b[01]+\b/ },
          { begin: /\b0o[0-7]+\b/ },
          { begin: /\b0x[0-9a-fA-F]+\b/ },
          { begin: /\b\d+\b/ },
        ],
      },
      {
        className: 'function',
        begin: /\b[a-z_][a-z0-9_]*(?=\s*\()/,
      },
      {
        className: 'function',
        begin: /\.\s*[a-z_][a-z0-9_]*(?=\s*\()/,
      },
      {
        className: 'attr',
        begin: /\b[a-z_][a-z0-9_]*(?=\s*:(?!:))/,
      },
    ],
  };
});

// Re-highlight meson blocks (this script loads after initial highlighting)
document.querySelectorAll('code.language-meson').forEach(function (block) {
  delete block.dataset.highlighted;
  hljs.highlightBlock(block);
});

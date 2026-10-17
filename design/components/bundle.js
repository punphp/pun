/* @ds-bundle: {"format":4,"namespace":"Pun","components":[{"name":"Button"},{"name":"Pill"},{"name":"Tag"},{"name":"Highlight"},{"name":"InstallBox"},{"name":"CodeBlock"},{"name":"Benchmark"},{"name":"ToolCard"},{"name":"Callout"},{"name":"Ribbon"},{"name":"Burst"},{"name":"StickerPanel"}]} */
(function () {
  var React = window.React;
  var h = React.createElement;
  function cx() { return Array.prototype.filter.call(arguments, Boolean).join(' '); }
  function omit(o, keys) { var r = {}; for (var k in o) if (keys.indexOf(k) < 0) r[k] = o[k]; return r; }

  function Button(p) {
    var variant = p.variant || 'plain', size = p.size || 'md';
    var rest = omit(p, ['variant', 'size', 'arrow', 'className', 'children', 'href']);
    rest.className = cx('pun-btn', variant !== 'plain' && 'pun-btn-' + variant, size === 'sm' && 'pun-btn-sm', p.className);
    var kids = [p.children, p.arrow ? h('span', { key: 'a', className: 'pun-arrow', 'aria-hidden': true }, '→') : null];
    if (p.href) { rest.href = p.href; return h.apply(null, ['a', rest].concat(kids)); }
    rest.type = rest.type || 'button';
    return h.apply(null, ['button', rest].concat(kids));
  }

  function Tag(p) {
    return h('span', { className: cx('pun-tag', p.tone && p.tone !== 'plain' && 'pun-tag-' + p.tone, p.className) }, p.children);
  }

  function Pill(p) {
    return h('span', { className: cx('pun-pill', p.className) },
      p.tag ? h(Tag, { tone: p.tagTone || 'mint' }, p.tag) : h('span', { className: 'pun-pill-dot', 'aria-hidden': true }),
      p.children);
  }

  function Highlight(p) {
    var v = p.variant || 'sticker';
    return h('span', { className: cx(v === 'marker' ? 'pun-hl-marker' : 'pun-hl-sticker', p.tone === 'gum' && 'pun-gum') }, p.children);
  }

  function InstallBox(p) {
    var opts = p.options || [];
    var st = React.useState(p.defaultIndex || 0), i = st[0], set = st[1];
    var cp = React.useState(false), copied = cp[0], setCopied = cp[1];
    var cur = opts[i] || { command: '' };
    function copy() {
      try { navigator.clipboard.writeText(cur.command); } catch (e) {}
      setCopied(true); setTimeout(function () { setCopied(false); }, 1200);
    }
    return h('div', { className: cx('pun-install', p.className) },
      opts.length > 1 ? h('div', { className: 'pun-tabs', role: 'tablist' }, opts.map(function (o, j) {
        return h('button', { key: o.label, className: 'pun-tab', role: 'tab', 'aria-selected': j === i ? 'true' : 'false', onClick: function () { set(j); } }, o.label);
      })) : null,
      h('div', { className: 'pun-cmd' },
        h('code', null, h('span', { className: 'pun-prompt', 'aria-hidden': true }, '$'), cur.command),
        h('button', { className: 'pun-copy', onClick: copy }, copied ? 'copied' : 'copy')));
  }

  function lineClass(t) {
    if (/^@@/.test(t)) return 'pun-l-dim';
    if (/^\+/.test(t)) return 'pun-l-add';
    if (/^-/.test(t)) return 'pun-l-del';
    if (/^\s*(#|\/\/)/.test(t)) return 'pun-l-dim';
    if (/^\s*✓/.test(t)) return 'pun-l-ok';
    return '';
  }
  function CodeBlock(p) {
    var lines = p.lines || String(p.code || '').split('\n');
    return h('div', { className: cx('pun-code', p.className) },
      (p.title || p.meta) ? h('div', { className: 'pun-code-bar' },
        h('span', null, p.dots ? h('span', { className: 'pun-dots', 'aria-hidden': true }, h('i', { style: { background: '#FF7AB8' } }), h('i', { style: { background: '#FFE53D' } }), h('i', { style: { background: '#8EF0C8' } })) : null, p.title),
        h('span', null, p.meta)) : null,
      h('pre', null, lines.map(function (t, j) {
        if (/^\$ /.test(t)) return h('div', { key: j, className: 'pun-l-cmd' }, h('span', { className: 'pun-prompt' }, '$'), t.slice(2));
        var hot = /^!/.test(t);
        return h('div', { key: j, className: hot ? 'pun-l-hot' : lineClass(t) }, (hot ? t.slice(1) : t) || ' ');
      })));
  }

  function fmt(s) { return s >= 1 ? (Math.round(s * 10) / 10) + ' s' : s + ' s'; }
  function Benchmark(p) {
    var rows = p.rows || [];
    var max = Math.max.apply(null, rows.map(function (r) { return r.seconds; }).concat([1]));
    return h('figure', { className: cx('pun-bench', p.className), style: { margin: 0 }, 'aria-label': rows.map(function (r) { return r.label + ' ' + (r.sub || '') + ' ' + fmt(r.seconds); }).join(', ') },
      p.title ? h('div', { className: 'pun-bench-title' }, h('span', null, p.title), h('span', null, 'lower is better')) : null,
      rows.map(function (r, j) {
        var pct = r.seconds / max;
        var inside = pct > 0.6;
        var w = 'calc((100% - ' + (inside ? 0 : 80) + 'px) * ' + pct.toFixed(4) + ' + ' + (inside ? 0 : 40) + 'px)';
        return h('div', { key: j, className: 'pun-lane' },
          h('div', { className: 'pun-who' }, r.label, r.sub ? h('small', null, r.sub) : null),
          h('div', { className: 'pun-bar' },
            h('div', { className: 'pun-fill pun-fill-' + (r.tone || 'zap'), style: { width: inside ? pct * 100 + '%' : w } }),
            h('span', { className: cx('pun-val', inside && 'pun-val-in') }, r.display || fmt(r.seconds))));
      }),
      (p.caption || p.href) ? h('figcaption', { className: 'pun-bench-foot' }, h('span', null, p.caption), p.href ? h('a', { href: p.href }, 'reproduce →') : null) : null);
  }

  function ToolCard(p) {
    return h('article', { className: cx('pun-tool', p.className), style: { '--pun-tilt': (p.tilt || 0) + 'deg' } },
      p.status ? h(Tag, { tone: p.statusTone || 'mint' }, p.status) : null,
      h('div', { className: 'pun-tile', style: { background: 'var(--' + (p.tone || 'zap') + ')' }, 'aria-hidden': true }, p.letter || (p.title || '?').charAt(0)),
      h('h3', null, p.title),
      p.replaces ? h('div', { className: 'pun-replaces' }, 'replaces ' + p.replaces) : null,
      p.description ? h('p', null, p.description) : null,
      p.command ? h('code', null, '$ ' + p.command) : null);
  }

  var CALLOUT_GLYPH = { note: 'i', tip: '✓', warn: '!' };
  function Callout(p) {
    var tone = p.tone || 'note';
    return h('aside', { className: cx('pun-callout', 'pun-callout-' + tone, p.className) },
      h('span', { className: 'pun-callout-ic', 'aria-hidden': true }, CALLOUT_GLYPH[tone]),
      h('div', null, p.title ? h('b', null, p.title) : null, p.children));
  }

  function Ribbon(p) {
    var items = p.items || [];
    var all = items.concat(items);
    return h('div', { className: cx('pun-ribbon', p.className), 'aria-label': items.join(', ') },
      h('div', { className: 'pun-ribbon-track', 'aria-hidden': true }, all.map(function (t, j) { return h('span', { key: j }, t); })));
  }

  function Burst(p) {
    return h('div', { className: cx('pun-burst', p.tone && p.tone !== 'gum' && 'pun-burst-' + p.tone, p.className), style: p.style }, p.children, p.sub ? h('small', null, p.sub) : null);
  }

  function StickerPanel(p) {
    var rest = omit(p, ['tone', 'className', 'children']);
    rest.className = cx('pun-panel', p.tone && p.tone !== 'lilac' && 'pun-panel-' + p.tone, p.className);
    return h('div', rest, p.children);
  }

  window.Pun = Object.assign(window.Pun || {}, { Button: Button, Pill: Pill, Tag: Tag, Highlight: Highlight, InstallBox: InstallBox, CodeBlock: CodeBlock, Benchmark: Benchmark, ToolCard: ToolCard, Callout: Callout, Ribbon: Ribbon, Burst: Burst, StickerPanel: StickerPanel });
})();

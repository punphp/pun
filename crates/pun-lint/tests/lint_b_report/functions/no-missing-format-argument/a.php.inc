<?php

namespace App;

use function Psl\Str\format;

sprintf('%s %s', $a);
sprintf('%s');
sprintf('%d%%', 1);
sprintf('100%%');
printf('%1$s %2$s %1$s', 'a');
printf('%2$s', 'a', 'b');
fprintf($fh, '%s %s', 'a');
fprintf($fh);
vsprintf('%s');
vsprintf('%s', []);
vprintf();
vfprintf($fh, '%s');
vfprintf($fh, '%s', $v);
format('%s');
\Psl\IO\write_line('%s %s', 1);
sprintf(...$args);
sprintf($fmt, 1);
sprintf("%s {$x}", 1);
sprintf('%-+ 05.2f %\'*10s %x', 1);
sprintf('%\'', 1);
sprintf('%');
sprintf('%5', 1);
sprintf('%.3e %u %c %o %b %E %F %X', 1, 2);
sprintf('%y');
sprintf('%3$s %s');
sprintf(format: '%s');
sprintf('%s', ...$rest);
\sprintf('%s %s', 1);
Foo\sprintf('%s');
-----
7:8: Call to `sprintf` contains 2 placeholders, 2 values given.
8:8: Call to `sprintf` contains 1 placeholder, 1 value given.
11:7: Call to `printf` contains 3 placeholders, 2 values given.
13:8: Call to `fprintf` contains 2 placeholders, 2 values given.
15:9: Call to `vsprintf` expects a format string and a values array, but only 1 argument given.
18:9: Call to `vfprintf` expects a format string and a values array, but only 2 arguments given.
20:7: Call to `Psl\Str\format` contains 1 placeholder, 1 value given.
21:19: Call to `Psl\IO\write_line` contains 2 placeholders, 2 values given.
25:8: Call to `sprintf` contains 3 placeholders, 3 values given.
29:8: Call to `sprintf` contains 8 placeholders, 8 values given.
30:8: Call to `sprintf` contains 1 placeholder, 1 value given.
31:8: Call to `sprintf` contains 2 placeholders, 3 values given.
34:9: Call to `sprintf` contains 2 placeholders, 2 values given.

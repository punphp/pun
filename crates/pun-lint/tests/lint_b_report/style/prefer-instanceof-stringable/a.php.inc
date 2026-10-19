<?php

namespace App;

if (is_object($x) && method_exists($x, '__toString')) {}
if (method_exists($x, '__TOSTRING') && is_object($x)) {}
if (is_object($x) and method_exists($x, '__toString')) {}
if (is_object($x) || method_exists($x, '__toString')) {}
if (is_object($x) && method_exists($y, '__toString')) {}
if (is_object($x) && method_exists($x, 'foo')) {}
if (is_object($x) && method_exists($x, "__toString{$a}")) {}
if (is_object($x) && method_exists($x, $m)) {}
if (is_object($x, 1) && method_exists($x, '__toString')) {}
if (is_object($x) && method_exists($x)) {}
if (is_object($o->x) && method_exists($o->x, '__toString')) {}
if (is_object($x) && method_exists($o->x, '__toString')) {}
if (is_object($x) && is_object($x)) {}
if (is_object($x) && $y) {}
if (\is_object($x) && \method_exists($x, '__toString')) {}
if (is_object($x) && foo($x, '__toString')) {}
if (is_object(x: $x) && method_exists($x, '__toString')) {}
-----
5:5: Consider using `instanceof Stringable` instead of `is_object() && method_exists()`.
6:5: Consider using `instanceof Stringable` instead of `is_object() && method_exists()`.
19:5: Consider using `instanceof Stringable` instead of `is_object() && method_exists()`.
21:5: Consider using `instanceof Stringable` instead of `is_object() && method_exists()`.

<?php

function f(string $s, array $a, int $i)
{
    $x = (array) str_split($s);
    $x = (array) array_keys($a);
    $x = (array) array_filter($a);
    $x = (array) array_map(fn ($v) => $v, $a);
    $x = (array) range(1, 3);
    $x = (array) array_merge($a, $a);
    $x = (array) explode(',', 'a,b');
    $x = (string) strtoupper($s);
    $x = (string) substr($s, 1);
    $x = (string) str_repeat('a', 3);
    $x = (string) mb_strtolower($s);
    $x = (string) lcfirst($s);
    $x = (string) ltrim($s);
    $x = (string) str_replace('a', 'b', $s);
    $x = (string) md5($s);
    $x = (string) date('Y');
    $x = (int) max(1, 2);
    $x = (int) intval($s);
    $x = (string) ($s . $s);
    $x = (string) ($s . '');
    $x = (string) ('a' . 'b');
    $u = $s . 'x';
    $x = (string) $u;
    if ($s !== '') {
        $x = (string) $s;
    }
    $e = explode(',', $s);
    $x = (array) $e;
    $m = [];
    foreach ($a as $k => $v) {
        $m[$k] = $v;
    }
    $x = (array) $m;
    $l = [1];
    $l[] = $i;
    $x = (array) $l;
    $x = (int) ($i * 2);
    $x = (int) -$i;
    $x = (float) ($i / 2);
    $x = (int) ($i % 2);
    $x = (int) ($i ** 2);
    $x = (string) __FUNCTION__;
    $x = (string) __DIR__;
    $x = (bool) ($s === 'a');
    $x = (bool) in_array($i, $a, true);
    $x = (bool) is_int($i);
    $x = (int) $i++;
    $x = (string) $s ?? 'x';
}
-----
<?php

function f(string $s, array $a, int $i)
{
    $x = str_split($s);
    $x = array_keys($a);
    $x = array_filter($a);
    $x = array_map(fn ($v) => $v, $a);
    $x = range(1, 3);
    $x = array_merge($a, $a);
    $x = explode(',', 'a,b');
    $x = (string) strtoupper($s);
    $x = substr($s, 1);
    $x = str_repeat('a', 3);
    $x = (string) mb_strtolower($s);
    $x = lcfirst($s);
    $x = ltrim($s);
    $x = str_replace('a', 'b', $s);
    $x = md5($s);
    $x = date('Y');
    $x = max(1, 2);
    $x = intval($s);
    $x = $s . $s;
    $x = $s . '';
    $x = 'a' . 'b';
    $u = $s . 'x';
    $x = $u;
    if ($s !== '') {
        $x = $s;
    }
    $e = explode(',', $s);
    $x = $e;
    $m = [];
    foreach ($a as $k => $v) {
        $m[$k] = $v;
    }
    $x = $m;
    $l = [1];
    $l[] = $i;
    $x = $l;
    $x = $i * 2;
    $x = -$i;
    $x = (float) ($i / 2);
    $x = $i % 2;
    $x = (int) ($i ** 2);
    $x = __FUNCTION__;
    $x = __DIR__;
    $x = $s === 'a';
    $x = in_array($i, $a, true);
    $x = is_int($i);
    $x = $i++;
    $x = $s ?? 'x';
}

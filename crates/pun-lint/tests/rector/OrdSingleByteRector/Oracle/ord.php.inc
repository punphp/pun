<?php

echo ord("ab");
echo ord('a' . 'b');
echo ord((string) $x);
echo ord("x$y");
echo ord(12);
echo ord(0x7B);
echo ord(-12);
echo ord($arr[0]);
echo ord(true);
echo ord('');
echo ord((int) $x);
echo ord(("ab"));

function f(string $s, int $i, ?string $n)
{
    return ord($s) + ord($i) + ord($n);
}

$v = 'abc';
$w = $v;
echo ord($w);
-----
<?php

echo ord("ab"[0]);
echo ord(('a' . 'b')[0]);
echo ord(((string) $x)[0]);
echo ord(("x$y")[0]);
echo ord(1);
echo ord(1);
echo ord(-12);
echo ord($arr[0]);
echo ord(true);
echo ord(''[0]);
echo ord(((int) $x)[0]);
echo ord(("ab"[0]));

function f(string $s, int $i, ?string $n)
{
    return ord($s[0]) + ord($i[0]) + ord($n);
}

$v = 'abc';
$w = $v;
echo ord($w[0]);

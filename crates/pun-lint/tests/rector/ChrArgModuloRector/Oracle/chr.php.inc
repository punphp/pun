<?php

echo chr(-1);
echo chr(256);
echo chr(PHP_INT_MAX);
echo chr($x);
echo chr(255);
echo chr(0x100);
echo chr(1 % 2);
echo chr((300));
echo chr(+300);
echo chr('300');
echo chr(1.5);
$y = 1000;
$y = 1001;
echo chr($y);
echo chr();

function f(int $i)
{
    $j = -5;
    return chr($j) . chr($i);
}
-----
<?php

echo chr(-1 % 256);
echo chr(256 % 256);
echo chr(PHP_INT_MAX);
echo chr($x);
echo chr(255);
echo chr(0x100 % 256);
echo chr(1 % 2);
echo chr((300 % 256));
echo chr(+300 % 256);
echo chr('300');
echo chr(1.5);
$y = 1000;
$y = 1001;
echo chr($y % 256);
echo chr();

function f(int $i)
{
    $j = -5;
    return chr($j % 256) . chr($i);
}

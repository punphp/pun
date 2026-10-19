<?php

namespace App;

echo $_GET['a'], $x, $_POST;
print $_REQUEST['x'];
print $x;
printf('%s', $_COOKIE['c'], $_FILES);
\printf($_SERVER['HTTP_HOST']);
sprintf('%s', $_GET['a']);
echo ($_SESSION['s']);
echo $a = $_GET['a'];
echo $c ? $_GET['a'] : 'x';
echo $c ? 'x' : 'y';
echo $_GET['a'] ?: 'x';
echo $c ?: 'x';
echo match ($x) { 1 => $_GET['a'], default => 'x' };
echo match ($x) { 1 => 'y', default => $_POST['p'] };
echo match ($x) { 1 => 'y' };
echo 'a' . $_GET['a'];
echo $_GET['a'] ?? 'x';
echo $a + $_GET['a'];
echo $$_GET;
echo ${$_GET['a']};
echo $_get['a'];
echo $_ENV['a'];
echo $o->$_GET;
?>
<?= $_GET['a'] ?>
<?php
printf(...$_GET);
printf(format: $_GET['f']);
$f($_GET);
-----
5:6: Tainted data passed to a sink function/construct.
5:22: Tainted data passed to a sink function/construct.
6:7: Tainted data passed to a sink function/construct.
8:14: Tainted data passed to a sink function/construct.
8:29: Tainted data passed to a sink function/construct.
9:9: Tainted data passed to a sink function/construct.
11:6: Tainted data passed to a sink function/construct.
12:6: Tainted data passed to a sink function/construct.
13:6: Tainted data passed to a sink function/construct.
15:6: Tainted data passed to a sink function/construct.
17:6: Tainted data passed to a sink function/construct.
18:6: Tainted data passed to a sink function/construct.
20:6: Tainted data passed to a sink function/construct.
21:6: Tainted data passed to a sink function/construct.
23:6: Tainted data passed to a sink function/construct.
24:6: Tainted data passed to a sink function/construct.
25:6: Tainted data passed to a sink function/construct.
31:11: Tainted data passed to a sink function/construct.
32:16: Tainted data passed to a sink function/construct.

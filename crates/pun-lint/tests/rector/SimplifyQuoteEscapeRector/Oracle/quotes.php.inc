<?php

$a = 'it\'s';
$b = "say \"hi\"";
$c = 'both \' and "';
$d = "no quotes";
$e = 'has $dollar\'s';
$f = "it's";
$g = 'it\'s \\ back';
$h = "tab\t\"x\"";
$i = 'é\'s';
$j = "x {$y} \"z\"";
$k = ['key\'s' => "v\"al"];
-----
<?php

$a = "it's";
$b = 'say "hi"';
$c = 'both \' and "';
$d = "no quotes";
$e = 'has $dollar\'s';
$f = "it's";
$g = 'it\'s \\ back';
$h = "tab\t\"x\"";
$i = "é's";
$j = "x {$y} \"z\"";
$k = ["key's" => 'v"al'];

<?php

$a[array_key_first($a)] = 1;
$a[array_key_last($a)] .= 'x';
unset($a[array_key_first($a)]);
echo $b[array_key_last($a)];
echo array_values($a)[1];
echo array_values($a)[count($a) - 2];
echo array_values($a)[count($b) - 1];
echo (array_values($a))[0];
echo $a[ARRAY_KEY_LAST($a)];
echo $a[array_key_first()];
echo $this->items[array_key_last($this->items)];
echo array_values($a, 1)[0];
echo array_values($a)[count($a) + 1];
echo array_values($a)[count($a, 1) - 1];
[$x[array_key_first($x)]] = [1];
echo $a[foo($a)];
-----
<?php

$a[array_key_first($a)] = 1;
$a[array_key_last($a)] .= 'x';
unset($a[array_key_first($a)]);
echo $b[array_key_last($a)];
echo array_values($a)[1];
echo array_values($a)[count($a) - 2];
echo array_values($a)[count($b) - 1];
echo array_first($a);
echo array_last($a);
echo $a[array_key_first()];
echo array_last($this->items);
echo array_values($a, 1)[0];
echo array_values($a)[count($a) + 1];
echo array_values($a)[count($a, 1) - 1];
[$x[array_key_first($x)]] = [1];
echo $a[foo($a)];

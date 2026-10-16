<?php
$a[ 0]; "$a[0x1A] $a[0b1] $a[007] $a[1_0] $a[ x] $a[a b] $a[\x] $a['x'] $a[#] $a[;] $a[€]" ;
"$a->
b" "$a?->b" "$a-> b" "$a->/*c*/b" "$a->#x
b" "$a->//x
b" "$a->1" "${a}" "${a[1]}" "{$a}" "$" "{" "\$a" "x\
y";
$x = <<<EOT
  $a
 EOT;
$y = <<<EOT
EOTA
EOT
;
$z = <<<EOT
x\
EOT;
$w = <<<EOT
x
EOT

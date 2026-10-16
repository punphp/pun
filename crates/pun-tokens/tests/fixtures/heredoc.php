<?php
$a = <<<EOT
plain $var {$arr['k']} ${name} ${expr . 1} $obj->prop $obj?->p $arr[0] $arr[-1] $arr[key] $arr[$i] \$esc \{$x}
EOT;
$b = <<<'NOW'
  raw $x {$y}
  NOW;
$c = <<<"DQ"
    flexible
      indented
    DQ . 'x';
$d = <<<EOT
EOT;
$e = <<<  EOT
    a
    EOT;
$f = [<<<A
  one
  A, <<<B
  two
  B];
$g = b<<<EOT
bin
EOT;
$h = <<<EOT
nested {$x(<<<INNER
  in
  INNER)}
EOT;
$i = "dq $a[0x1A] $a[01] {$a->b->c} ${a}[x] {$a::$b} \u{1F600} \x41 \101";
$j = `ls $dir`;
$k = b"bin $x";
$l = b'bin';
$m = "unterminated $x

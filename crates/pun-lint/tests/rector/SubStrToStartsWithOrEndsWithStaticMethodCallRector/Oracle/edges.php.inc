<?php
$a = 'prefix' === substr($value, 0);
$b = substr($value, !1) === 'prefix';
$c = $value === $other;
-----
<?php
$a = \Illuminate\Support\Str::startsWith($value, 'prefix');
$b = substr($value, !1) === 'prefix';
$c = $value === $other;

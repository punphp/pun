<?php
class_alias('A', 'B');
class_exists('A');
if (true) { echo 1; } elseif (false) { class C {} }
class D {}
?>

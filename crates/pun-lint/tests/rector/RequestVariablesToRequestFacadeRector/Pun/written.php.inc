<?php

namespace App;

$_GET['a']++;
$copy = &$_GET['a'];
$both = isset($_GET['a'], $b);
$float = $_POST[1.5];
$name = "{$_GET['a']}";

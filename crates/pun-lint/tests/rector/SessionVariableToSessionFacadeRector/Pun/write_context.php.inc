<?php
// Replacing writable variables or interpolated offsets with calls makes invalid PHP.
$_SESSION[$key] += 1;
$_SESSION[$key] = &$reference;
$_SESSION = [];
$_SESSION['x']['y'] = 1;
$a = "{$_SESSION['x']}";

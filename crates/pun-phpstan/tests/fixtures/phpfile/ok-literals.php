<?php
// A line comment
# A hash comment
/* A block
   comment */
declare(strict_types=1);

$list = [];
$list[] = 'it\'s';
$list[] = 'back\\slash and \n not-a-newline';
return ['list' => $list];

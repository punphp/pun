<?php

// With the default `style: interpolation`:
$a = "Hello, {$name}!";
$b = "value: {$obj->name}";

// Complex expressions stay as concatenation (never flagged):
$c = "result: " . strtolower($input);
$d = "class: " . Foo::class;

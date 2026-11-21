<?php

$names = ['a' => 'x', 'b' => 'y'];
$pairs = [];
foreach ($names as $key => $name) {
    $pairs[] = $key . '=' . $name;
}
foreach ([1, 2] as $n) $pairs[] = $n;
foreach ([] as $never) {
    $pairs[] = $never;
}

$truth = [];
foreach ([null, false, true, 0, 3, '', '0', 'a', [], [1]] as $v) {
    if ($v) {
        $truth[] = 'yes';
    } else {
        $truth[] = 'no';
    }
    $truth[] = !$v;
}

$picked = [];
foreach ([1, 0, 'b'] as $v) {
    if (true) {
    }
    if (!$v) {
        $picked[] = 'zero';
    } elseif ($v) {
        $picked[] = 'first';
    } elseif ($nope) {
        $picked[] = 'never';
    } else if ($nope) {
        $picked[] = 'never';
    } else {
        $picked[] = 'never';
    }
}
if (false) {
    return $nope;
}
if (false) {
} elseif (true) {
    $picked[] = 'elseif';
} else {
    $picked[] = 'never';
}
if (false) {
} else if (false) {
} else {
    $picked[] = 'else';
}

$missing = glob(__DIR__ . '/no-such-dir/*.neon');
$file = is_file(__DIR__ . '/no-such-file.php');
$found = [];
foreach ($missing as $m) {
    $found[] = $m;
}
$classes = [
    \Foo\Bar::class,
    Foo\Bar::class,
    class_exists(ArrayObject::class),
    class_exists('\\arrayobject'),
    class_exists(Countable::class),
    interface_exists(Countable::class),
    interface_exists(ArrayObject::class),
];
if (false) {
    foreach ($nope as $x) {
        $found[] = $x;
    }
}
foreach ([1] as $n) {
    if (true) {
        return ['pairs' => $pairs, 'truth' => $truth, 'picked' => $picked, 'missing' => $missing, 'file' => $file, 'classes' => $classes];
    }
}

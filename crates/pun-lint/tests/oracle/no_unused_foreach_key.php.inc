<?php
function a($items) {
    /** @var int $k1 */
    foreach ($items as $k1 => $value) {
        echo $value;
    }
    /**
     * @var int $k2
     */
    foreach ($items as $k2 => $value) {
        echo $value;
    }
    /**
     * Loop.
     *
     * @var int $k3
     * @var string $value
     */
    foreach ($items as $k3 => $value) {
        echo $value;
    }
    // plain
    foreach ($items as $k4 => $value) echo $value;
    foreach ($items as $k5 => $value):
        echo $value;
    endforeach;
    foreach ($items as $K6 => $value) { echo $k6; }
    foreach ($items as $k7 => [$a, $b]) { echo $a; }
    foreach ($items as $k8   =>   $v) { }
    /** @var array<int, string> $items */
    foreach ($items as $k9 => $v) { }
}
function b($items, $o) {
    foreach ($items as $o->k => $v) { echo $v; }
    foreach ($items as $$k => $v) { echo $v; }
    /** @var int $k10
     * Loop.
     */
    foreach ($items as $k10 => $v) { echo $v; }
    /**
     * @varx int $k11
     */
    foreach ($items as $k11 => $v) { echo $v; }
    foreach ($items as $k12 => $v) { echo A::$k12; }
    foreach ($items as $k13 => $v) { echo A::$$k13; }
    foreach ($items as $k14 => $v) { echo A::${'k14'}; }
    foreach ($items as $k15 => $v) { echo $k15::$v; }
    foreach ($items as $k16 => $v) { echo compact(var_name: 'k16'); }
    foreach ($items as $k17 => $v) { echo \compact(['k17']); }
    foreach ($items as $k18 => $v) { echo compact(array('x' => 'k18')); }
    foreach ($items as $k19 => $v) { echo compact([...$z]); }
    foreach ($items as $k20 => $v) { echo Foo\compact('k20'); }
    foreach ($items as $k21 => $v) { echo $f('k21'); }
}
function c($items) {
    /** @var int $k2
     * @return x
     */
    foreach ($items as $k2 => $v) { echo $v; }
    /**
     * Text.
     * @var int $k3 */
    foreach ($items as $k3 => $v) { echo $v; }
    /**
     * @var int $k4
     *
     * More.
     */
    foreach ($items as $k4 => $v) { echo $v; }
    /**
     * @var int $k5
     *
     * @var int $v
     */
    foreach ($items as $k5 => $v) { echo $v; }
    /**
     * @var int $k6 desc
     * more desc
     * @var int $v
     */
    foreach ($items as $k6 => $v) { echo $v; }
    /**
     * Summary.
     * @var int $k7 desc
     *
     */
    foreach ($items as $k7 => $v) { echo $v; }
}
-----
<?php
function a($items) {
    foreach ($items as $value) {
        echo $value;
    }
    foreach ($items as $value) {
        echo $value;
    }
    /**
     * Loop.
     *
     * @var string $value
     */
    foreach ($items as $value) {
        echo $value;
    }
    // plain
    foreach ($items as $value) echo $value;
    foreach ($items as $value):
        echo $value;
    endforeach;
    foreach ($items as $K6 => $value) { echo $k6; }
    foreach ($items as [$a, $b]) { echo $a; }
    foreach ($items as $v) { }
    /** @var array<int, string> $items */
    foreach ($items as $v) { }
}
function b($items, $o) {
    foreach ($items as $o->k => $v) { echo $v; }
    foreach ($items as $v) { echo $v; }
    foreach ($items as $v) { echo $v; }
    /**
     * @varx int $k11
     */
    foreach ($items as $v) { echo $v; }
    foreach ($items as $v) { echo A::$k12; }
    foreach ($items as $k13 => $v) { echo A::$$k13; }
    foreach ($items as $v) { echo A::${'k14'}; }
    foreach ($items as $k15 => $v) { echo $k15::$v; }
    foreach ($items as $k16 => $v) { echo compact(var_name: 'k16'); }
    foreach ($items as $k17 => $v) { echo \compact(['k17']); }
    foreach ($items as $k18 => $v) { echo compact(array('x' => 'k18')); }
    foreach ($items as $v) { echo compact([...$z]); }
    foreach ($items as $v) { echo Foo\compact('k20'); }
    foreach ($items as $v) { echo $f('k21'); }
}
function c($items) {
    /**
     * @return x
     */
    foreach ($items as $v) { echo $v; }
    /**
     * Text. */
    foreach ($items as $v) { echo $v; }
    foreach ($items as $v) { echo $v; }
    /**
     *
     * @var int $v
     */
    foreach ($items as $v) { echo $v; }
    /**
     * @var int $v
     */
    foreach ($items as $v) { echo $v; }
    /**
     * Summary.
     *
     */
    foreach ($items as $v) { echo $v; }
}

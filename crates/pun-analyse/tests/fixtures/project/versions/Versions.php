<?php

namespace Versions;

interface HasName
{
    public string $name { get; }
}

interface Duplicates
{
    public function same(): void;

    public function SAME(): void;
}

final class Features
{
    final public string $final = '';

    public string $hooked {
        get => 'hooked';
    }

    public private(set) static int $asymmetric = 1;

    public readonly int $readonly;

    public string $setHook {
        set(public string $value) {
        }
    }

    public function __construct(public int $promoted, final public int $finalPromoted = 0)
    {
    }
}

if (80000 > PHP_VERSION_ID) {
    class OlderFlipped
    {
        final private function foo(): void {}
    }
}

if (70000 < PHP_VERSION_ID) {
    class NewerFlipped
    {
        final private function foo(): void {}
    }
}

if (PHP_VERSION_ID <= 70400) {
    class AtMost74
    {
        final private function foo(): void {}
    }
}

if (PHP_VERSION_ID === 80100) {
    class Exactly81
    {
        final private function foo(): void {}
    }
}

if (PHP_VERSION_ID < 70000 && PHP_VERSION_ID >= 80000) {
    class Never
    {
        final private function foo(): void {}
    }
}

if (!(PHP_VERSION_ID >= 80000)) {
    class NotAtLeast80
    {
        final private function foo(): void {}
    }
}

class Docs
{
    /** @readonly */
    /* a plain comment */
    public $afterComment = 1;

    /**/
    public $emptyDoc = 1;

    /**
     * text @readonly
     * @ readonly
     */
    public $notTags = 1;

    /**
     * @final
     */
    private function finalByDoc(): void {}
}

class FromAnalysed extends AnalysedImmutable
{
    public $a = 1;

    public $b = 2;
}

class FromScanned extends ScannedImmutable
{
    public $c = 1;
}

class FromMissing extends Missing
{
    public $d = 1;
}

class VirtualParent
{
    public int $v {
        get => 1;
    }
}

class VirtualChild extends VirtualParent
{
    public int $v = 2 {
        get => 3;
    }
}

$anonymous = new class {
    public readonly $untyped;

    public function shown(): void {}
};

class Blocks
{
    public int $backed = 1 {
        get {
            return $this->backed;
        }
    }

    public int $virtual = 1 {
        get {
            return 2;
        }
    }
}

if (class_exists('Versions\\Features')) {
    if (PHP_VERSION_ID < 80000) {
        if (PHP_VERSION_ID < 70400) {
            class Nested
            {
                final private function foo(): void {}
            }
        }
    }
}

if (PHP_VERSION_ID <> 80100 && PHP_VERSION_ID >= $version) {
    class NotEqual
    {
        final private function foo(): void {}
    }
}

enum FloatCase: int
{
    case A = 1.5;
}

enum Unbacked
{
    case N = null;
    case P = (2.5);
    case C = Features::class;
}

enum Values: int
{
    case A = 1;
    case B = Values::A->value;
}

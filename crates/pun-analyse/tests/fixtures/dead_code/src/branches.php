<?php

namespace DeadCodeFixture\Branches;

enum Suit
{
    case Hearts;
}

class Base
{
    protected static int $shared = 0;
}

class Members extends Base
{
    private int $keyed = 0;
    private int $braced = 0;
    private int $bumped = 0;
    private int $named = 0;
    private int $fresh = 0;
    private static int $dynamic = 0;
    private string $name = '';
    private int $onArray = 0;
    private int $onOther = 0;
    private int $onUnknown = 0;
    private int $skipped = 0;

    public function __construct()
    {
    }

    public function run(array $values, Suit $suit, Other $other): void
    {
        ['k' => $this->keyed, , $x] = $values;
        echo $this->{'braced'};
        ++$this->bumped;
        $this->fill(into: $this->named);
        $made = new self;
        $prop = 'dynamic';
        echo self::$$prop;
        echo self::${'dyn' . 'amic'};
        echo parent::$shared;
        echo $suit->name;
        echo $values->onArray;
        echo $other->onOther;
        echo \Nowhere\Missing::$onUnknown;
        var_dump($made, $x);
        $f = array($this, 'legacy');
        $g = [self::class, 'viaClass'];
        $f();
        $g();
        function inner(): void
        {
        }
    }

    private function fill(int &$into): void
    {
        $into = 1;
    }

    private function legacy(): void
    {
    }

    private static function viaClass(): void
    {
    }
}

class Other
{
    public int $onOther = 0;
}

function whileLoop(): void
{
    while (true):
        echo 'while';
        return;
        echo 'never';
    endwhile;
}

function forLoop(): void
{
    for (;;):
        echo 'for';
        return;
        echo 'never';
    endfor;
}

function foreachLoop(array $values): void
{
    foreach ($values as $v):
        echo $v;
        return;
        echo 'never';
    endforeach;
}

function declareBlock(): void
{
    declare(ticks=1):
        echo 'declare';
        return;
        echo 'never';
    enddeclare;
}

function terminating(int $i): void
{
    match (throw new \Exception()) {
        default => 1,
    };
    echo 'after match';
}

function notTerminating(): void
{
    array_map(function () {
        throw new \Exception();
    }, []);
    echo 'reached';
}

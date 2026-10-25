<?php

namespace ConstantExpressionGuards;

use ConstantExpressionGuards\Receiver as Alias;

class Receiver {
    private const PRIVATE_NAME = 1;
    protected const PROTECTED_NAME = 2;
    public const PRESENT = 3;

    public function thisReceiver(): void {
        if ($this::MISSING) { echo $this::MISSING; }
        echo $this::MISSING;
        if (!$this::SECOND) { echo $this::SECOND; } else { echo $this::SECOND; }
        echo $this::SECOND;
    }
}

class Other {}

function truthy(Receiver $receiver): void {
    if ($receiver::MISSING) { echo $receiver::MISSING; }
    echo $receiver::MISSING;
    if ($receiver::PRIVATE_NAME) { echo $receiver::PRIVATE_NAME; }
    echo $receiver::PRIVATE_NAME;
}

function falsy(Receiver $receiver): void {
    if (!$receiver::MISSING) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    echo $receiver::MISSING;
}

function identical(Receiver $receiver): void {
    if ($receiver::MISSING === 1) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    echo $receiver::MISSING;
}

function equal(Receiver $receiver): void {
    if (1 == $receiver::MISSING) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    echo $receiver::MISSING;
}

function notIdentical(Receiver $receiver): void {
    if ($receiver::MISSING !== null) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    echo $receiver::MISSING;
}

function booleanComparison(Receiver $receiver): void {
    if ($receiver::MISSING === false) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    echo $receiver::MISSING;
}

function greater(Receiver $receiver): void {
    if ($receiver::MISSING > 0) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    echo $receiver::MISSING;
}

function conjunction(Receiver $receiver, bool $flag): void {
    if ($flag && $receiver::MISSING) { echo $receiver::MISSING; }
    echo $receiver::MISSING;
    if ($receiver::SECOND && $flag) { echo $receiver::SECOND; }
    echo $receiver::SECOND;
}

function disjunction(Receiver $receiver, bool $flag): void {
    if ($flag || $receiver::MISSING) { echo $receiver::MISSING; }
    echo $receiver::MISSING;
    if ($receiver::SECOND || $flag) { echo $receiver::SECOND; }
    echo $receiver::SECOND;
}

function earlyExit(Receiver $receiver): void {
    if (!$receiver::MISSING) { return; }
    echo $receiver::MISSING;
}

function ternary(Receiver $receiver): void {
    $receiver::MISSING ? print($receiver::MISSING) : print($receiver::MISSING);
    echo $receiver::MISSING;
}

function branchMutation(Receiver $receiver, bool $flag): void {
    if ($receiver::MISSING) {
        if ($flag) { $receiver = new Other; }
        echo $receiver::MISSING;
    }
    echo $receiver::MISSING;
}

function falseBranchMutation(Receiver $receiver): void {
    if ($receiver::MISSING) {} else { $receiver = new Receiver; }
    echo $receiver::MISSING;
}

function branchLocal(Receiver $receiver, bool $flag): void {
    if ($flag) { if ($receiver::MISSING) {} }
    echo $receiver::MISSING;
}

function branchCommon(Receiver $receiver, bool $flag): void {
    if ($flag) { if ($receiver::MISSING) {} } else { if ($receiver::MISSING) {} }
    echo $receiver::MISSING;
}

function mutate(Receiver $receiver, Receiver $receiverLong): void {
    if ($receiver::MISSING) {
        $receiverLong = new Receiver;
        echo ($receiver)::MISSING;
        $receiver = new Receiver;
        echo $receiver::MISSING;
    }
    echo $receiver::MISSING;
}

/** @param 'MISSING'|'PRIVATE_NAME' $name */
function dynamic(Receiver $receiver, string $name): void {
    if ($receiver::{$name}) {
        echo $receiver::{$name};
        echo $receiver::MISSING;
        $name = 'PRIVATE_NAME';
        echo $receiver::{$name};
    }
    echo $receiver::{$name};
}

/** @param 'MISSING'|'PRIVATE_NAME' $name */
function writtenDoesNotGuardDynamic(Receiver $receiver, string $name): void {
    if ($receiver::MISSING) { echo $receiver::{$name}; }
}

function named(): void {
    if (Alias::MISSING) { echo Receiver::MISSING; }
    echo \ConstantExpressionGuards\Receiver::MISSING;
    if (Receiver::PRIVATE_NAME === 1) { echo Alias::PRIVATE_NAME; }
    echo Alias::PRIVATE_NAME;
}

class Holder {
    public Receiver $receiver;
    /** @var array<Receiver> */
    public array $items;
    public function nested(bool $flag): void {
        if ($this->receiver::MISSING) {
            echo $this->receiver::MISSING;
            $this->receiver = new Receiver;
            echo $this->receiver::MISSING;
        }
        if ($this->items[0]::MISSING) {
            echo $this->items[0]::MISSING;
            $this->items[1] = new Receiver;
            echo $this->items[0]::MISSING;
            $this->items = [];
            echo $this->items[0]::MISSING;
        }
    }
}

/** @param Receiver|null $receiver */
function nullable(?Receiver $receiver): void {
    if ($receiver::MISSING) { echo $receiver::MISSING; }
}

function scalar(int $receiver): void {
    if ($receiver::MISSING) { echo $receiver::MISSING; }
}

/** @param UnknownReceiver $receiver */
function unknown($receiver): void {
    if ($receiver::MISSING) { echo $receiver::MISSING; }
}

function mixedReceiver(mixed $receiver): void {
    if ($receiver::MISSING) { echo $receiver::MISSING; }
}

function dynamicClass(string $receiver): void {
    if ($receiver::class) { echo $receiver::class; }
}

/** @param array<Receiver> $receivers */
function incrementIndex(array $receivers, int $index): void {
    if ($receivers[$index]::MISSING) {
        echo $receivers[$index]::MISSING;
        ++$index;
        echo $receivers[$index]::MISSING;
        $index++;
        echo $receivers[$index]::MISSING;
    }
}

/** @param array<Receiver> $receivers */
function appendAndForeach(Receiver $receiver, array $receivers): void {
    if ($receivers[0]::MISSING) {
        $receivers[] = new Receiver;
        echo $receivers[0]::MISSING;
    }
    if ($receiver::MISSING) {
        foreach ($receivers as $receiver) { echo $receiver::MISSING; }
        echo $receiver::MISSING;
    }
}

function destructuring(Receiver $receiver): void {
    if ($receiver::MISSING) {
        list(0 => $receiver) = [new Receiver];
        echo $receiver::MISSING;
        [$receiver] = [new Receiver];
        echo $receiver::MISSING;
        list(, $receiver) = [new Receiver, new Receiver];
        echo $receiver::MISSING;
    }
}

function unsetReceiver(Receiver $receiver): void {
    if ($receiver::MISSING) {
        unset($receiver);
        echo $receiver::MISSING;
    }
}

class StaticHolder extends Receiver {
    public static Receiver $receiver;
    public function keywords(): void {
        if (self::MISSING) { echo self::MISSING; }
        if (static::MISSING) { echo static::MISSING; }
        if (parent::MISSING) { echo parent::MISSING; }
        if (self::$receiver::MISSING) {
            echo self::$receiver::MISSING;
            self::$receiver = new Receiver;
            echo self::$receiver::MISSING;
        }
    }
}

/** @param 'MISSING'|'PRIVATE_NAME' $name */
function conditionalHolderInvalidation(Receiver $receiver, string $name): void {
    if ($receiver::{$name}) { $name = 'PRIVATE_NAME'; }
    echo $receiver::{$name};
    $receiver = new Receiver;
    echo $receiver::{$name};
}

/** @param 'MISSING'|'PRIVATE_NAME' $name */
function conditionalSelectorInvalidation(Receiver $receiver, string $name): void {
    if ($receiver::{$name}) { $name = 'PRIVATE_NAME'; }
    echo $receiver::{$name};
    $name = 'MISSING';
    echo $receiver::{$name};
}

function namedHolderIsNotConditional(Receiver $receiver, bool $flag): void {
    if ($flag) { $name = 'PRIVATE_NAME'; } else { if ($receiver::MISSING) {} }
    echo $receiver::MISSING;
}

function receiverIdentity(Receiver $left, Receiver $right, Receiver $leftLong): void {
    if ($left::MISSING) {
        echo $right::MISSING;
        $leftLong = new Receiver;
        echo $left::MISSING;
    }
}

function shortTernary(Receiver $receiver): void {
    $receiver::MISSING ?: print($receiver::MISSING);
    echo $receiver::MISSING;
}

function falseSentinels(Receiver $receiver): void {
    if ($receiver::MISSING != 0) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    if ('' == $receiver::SECOND) { echo $receiver::SECOND; } else { echo $receiver::SECOND; }
}

function rememberedCheck(Receiver $receiver): void {
    $check = $receiver::MISSING;
    if ($check) { echo $receiver::MISSING; }
    $receiver = new Receiver;
    if ($check) { echo $receiver::MISSING; }
}

class Selector { public const NAME = 'MISSING'; }

function literalSelectors(Receiver $receiver): void {
    if ($receiver::{'MISSING'}) { echo $receiver::{"MISSING"}; }
    echo $receiver::MISSING;
    if ($receiver::{Selector::NAME}) { echo $receiver::{Selector::NAME}; }
    echo $receiver::MISSING;
    if ($receiver::{'MISSING-NAME'}) { echo $receiver::{'MISSING-NAME'}; }
}

/** @param 'MISSING'|'PRIVATE_NAME' $name */
function conditionalHolderJoins(Receiver $receiver, string $name, bool $flag): void {
    if ($receiver::{$name}) { $name = 'PRIVATE_NAME'; }
    if ($flag) { echo $receiver::{$name}; } else { echo $receiver::{$name}; }
    echo $receiver::{$name};
    $name = 'MISSING';
    echo $receiver::{$name};
}

function comparisonOperators(Receiver $receiver): void {
    if ($receiver::MISSING <=> 0) { echo $receiver::MISSING; } else { echo $receiver::MISSING; }
    echo $receiver::MISSING;
    if ($receiver::SECOND <> 1) { echo $receiver::SECOND; } else { echo $receiver::SECOND; }
    echo $receiver::SECOND;
}

function elseIfJoins(Receiver $receiver, bool $flag): void {
    if ($flag) { if ($receiver::MISSING) {} }
    elseif ($receiver::MISSING) { echo $receiver::MISSING; }
    echo $receiver::MISSING;
}

function terminatingBranchMutation(Receiver $receiver, bool $flag): void {
    if ($receiver::MISSING) {
        if ($flag) { $receiver = new Other; return; }
        echo $receiver::MISSING;
    }
    echo $receiver::MISSING;
}

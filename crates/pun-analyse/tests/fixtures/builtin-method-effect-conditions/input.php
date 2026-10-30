<?php

namespace BuiltinMethodEffects;

function inheritedMutator(\SplQueue $queue): void
{
    if ($queue->count() > 0) {
        \PHPStan\dumpType((bool) $queue->count());
        $queue->shift();
        \PHPStan\dumpType((bool) $queue->count());
    }
}

function anotherMutator(\SplDoublyLinkedList $list): void
{
    if ($list->count() > 0) {
        $list->pop();
        \PHPStan\dumpType((bool) $list->count());
    }
}

function unknownBuiltinEffect(\SplQueue $queue): void
{
    if ($queue->count() > 0) {
        $queue->isEmpty();
        \PHPStan\dumpType((bool) $queue->count());
    }
}

function unrelatedReceiver(\SplQueue $queue, \SplQueue $other): void
{
    if ($queue->count() > 0) {
        $other->shift();
        \PHPStan\dumpType((bool) $queue->count());
    }
}

function knownPure(\DateTimeImmutable $date): void
{
    if ($date->getTimestamp() === 100) {
        $date->getTimezone();
        \PHPStan\dumpType($date->getTimestamp());
    }
}

function knownDateMutation(\DateTime $date): void
{
    if ($date->getTimestamp() === 100) {
        $date->modify('+1 day');
        \PHPStan\dumpType($date->getTimestamp());
    }
}

class UnknownOverride extends \SplQueue
{
    public function shift(): mixed
    {
        return parent::shift();
    }
}

class ImpureOverride extends \SplQueue
{
    /** @phpstan-impure */
    public function shift(): mixed
    {
        return parent::shift();
    }
}

function userOverrides(UnknownOverride $unknown, ImpureOverride $impure): void
{
    if ($unknown->count() > 0) {
        $unknown->shift();
        \PHPStan\dumpType((bool) $unknown->count());
    }
    if ($impure->count() > 0) {
        $impure->shift();
        \PHPStan\dumpType((bool) $impure->count());
    }
}

function nestedQueue(): void
{
    $queue = new \SplQueue();
    $queue->push(1);
    if ($queue->count() > 0) {
        for ($i = 0; $i < 5; $i++) {
            while ($queue->count() > 0 && $value = $queue->shift()) {
            }
        }
    }
    \PHPStan\dumpType((bool) $queue->count());
}

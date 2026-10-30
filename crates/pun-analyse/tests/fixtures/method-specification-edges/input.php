<?php

namespace MethodSpecificationEdges;

class Value {}

interface Scanner
{
    public function isDone(): bool;
    public function value(): ?Value;
    /** @phpstan-impure */
    public function read(): string;
}

function truthyRightOperand(Scanner $scanner): void
{
    if (!$scanner->isDone() && $scanner->read() !== '') {
        \PHPStan\dumpType($scanner->isDone());
    }
}

function falseyRightOperand(Scanner $scanner): void
{
    if ($scanner->isDone() || $scanner->read() === '') {
        return;
    }
    \PHPStan\dumpType($scanner->isDone());
}

function unchangedReceiver(Scanner $scanner): void
{
    if ($scanner->value() === null) {
        return;
    }
    for ($i = 0; $i < 2; $i++) {
        \PHPStan\dumpType($scanner->value());
    }
    \PHPStan\dumpType($scanner->value());
}

function unrelatedReceiver(Scanner $scanner, Scanner $other): void
{
    if ($scanner->value() === null) {
        return;
    }
    for ($i = 0; $i < 2; $i++) {
        $other->read();
        \PHPStan\dumpType($scanner->value());
    }
    \PHPStan\dumpType($scanner->value());
}

function mutatingBody(Scanner $scanner): void
{
    if ($scanner->value() === null) {
        return;
    }
    while ($scanner->isDone()) {
        $scanner->read();
    }
    \PHPStan\dumpType($scanner->value());
}

function mutatingBackEdge(Scanner $scanner): void
{
    if ($scanner->isDone()) {
        return;
    }
    while (!$scanner->isDone()) {
        \PHPStan\dumpType($scanner->isDone());
        $scanner->read();
        \PHPStan\dumpType($scanner->isDone());
    }
}

function unchangedBackEdge(Scanner $scanner): void
{
    if (!$scanner->isDone()) {
        return;
    }
    for ($i = 0; $i < 2; $i++) {
        \PHPStan\dumpType($scanner->isDone());
    }
}

function mutatingBreak(Scanner $scanner): void
{
    if ($scanner->value() === null) {
        return;
    }
    while ($scanner->isDone()) {
        $scanner->read();
        break;
    }
    \PHPStan\dumpType($scanner->value());
}

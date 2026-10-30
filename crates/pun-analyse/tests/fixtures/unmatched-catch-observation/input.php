<?php

namespace UnmatchedCatchObservation;

function nothing(): void {}

function beforeAndAfter(): void
{
    $value = 1;
    try {
        $value = 'written';
    } catch (\RuntimeException $error) {
        $value; // observe:pre-try
        $error; // observe:exception
        nothing(); // observe:catch-void
        $value = 2;
        $value; // observe:catch-assignment
    } finally {
        $value; // observe:finally-live
    }
    $value; // observe:continuation
}

function nested(): void
{
    $value = 3;
    try {
        $value = 4;
    } catch (\RuntimeException $outer) {
        $value; // observe:outer-pre-try
        try {
            $value = 5;
            throw new \LogicException();
        } catch (\LogicException $inner) {
            $value; // observe:inner-matching
            $inner; // observe:inner-exception
        } catch (\RuntimeException $wrong) {
            $value; // observe:inner-nonmatching
            $value = 6;
        } finally {
            $value; // observe:inner-finally
        }
        $value; // observe:outer-catch-tail
        throw new \LogicException();
    } catch (\LogicException $other) {
        $value; // observe:sibling-not-fed-by-dead-catch
    } finally {
        $value; // observe:outer-finally
    }
    $value; // observe:nested-continuation
}

function sharedFinally(): void
{
    $value = 7;
    try {
        try {
            $value = 8;
        } catch (\RuntimeException $error) {
            $value = 'dead';
            return;
        }
        $value; // observe:after-inner
    } finally {
        $value; // observe:shared-finally
    }
    $value; // observe:after-shared-finally
}

/** @param string $value */
function channels(int|string $value): void
{
    /** @var string $value */
    $value; // observe:channels-before
    try {
        $value = 9;
    } catch (\RuntimeException $error) {
        $value; // observe:channels-pre-try
    }
    $value; // observe:channels-after
}

function repeatedTermination(): int
{
    try {
        return 10;
    } catch (\RuntimeException $error) {
        $value = 11;
        return $value; // observe:first-return
        $value = 12;
        return $value; // observe:second-return
        $value = 13;
        exit;
        $value = 14;
        $value; // observe:after-exit
    }
}

function noVariable(): void
{
    try {
        $value = 15;
    } catch (\RuntimeException) {
        16; // observe:catch-without-variable
    }
}

function multipleCaughtTypes(): void
{
    try {
        $value = 21;
    } catch (\RuntimeException|\LogicException $error) {
        $error; // observe:multiple-exception-types
    }
}

function yieldsDoNotEscape(): void
{
    try {
        $value = 17;
    } catch (\RuntimeException $error) {
        yield 18;
    }
    $value; // observe:after-dead-yield
}

function matchedTermination(bool $pick): void
{
    $value = 19;
    try {
        if ($pick) {
            throw new \RuntimeException();
        }
        $value = 20;
    } catch (\RuntimeException $error) {
        $value; // observe:matched-entry
        return;
    } finally {
        $value; // observe:matched-finally
    }
    $value; // observe:matched-continuation
}

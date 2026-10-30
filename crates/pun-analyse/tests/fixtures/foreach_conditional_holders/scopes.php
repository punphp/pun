<?php declare(strict_types=1);

namespace ForeachConditionalHolders;

function keyed(): void {
    $list = ['test1' => 'Test', 'test2' => 'Test2'];
    foreach ($list as $key => $value) {
        $derived = 'fetch' . $value;
        if ($key === 'test1') {
            $value; // observe
            $list[$key]; // observe
            $derived; // observe
        } else {
            $value; // observe
            $list[$key]; // observe
            $derived; // observe
        }
    }
}

function listShape(): void {
    $list = ['one', 'two'];
    foreach ($list as $key => $value) {
        if ($key === 0) {
            $value; // observe
        } else {
            $value; // observe
        }
    }
}

function literal(): void {
    foreach (['a' => 'one', 'b' => 'two'] as $key => $value) {
        if ($key === 'a') {
            $value; // observe
        } else {
            $value; // observe
        }
    }
}

function reference(): void {
    $list = ['a' => 'one', 'b' => 'two'];
    foreach ($list as $key => &$value) {
        if ($key === 'a') {
            $value; // observe
            $list[$key]; // observe
        } else {
            $value; // observe
            $list[$key]; // observe
        }
    }
}

/** @param array<'a'|'b', 'one'|'two'> $list */
function generic(array $list): void {
    foreach ($list as $key => $value) {
        if ($key === 'a') {
            $value; // observe
        }
    }
}

function documented(): void {
    $list = ['a' => 'one', 'b' => 'two'];
    /** The foreach doccomment deliberately disables conditional holders. */
    foreach ($list as $key => $value) {
        if ($key === 'a') {
            $value; // observe
        }
    }
}

function overwritten(): void {
    $list = ['a' => 'one', 'b' => 'two'];
    foreach ($list as $key => $value) {
        $derived = 'fetch' . $value;
        $derived = 'replacement';
        if ($key === 'a') {
            $derived; // observe
        }
    }
}

/** @param array{a?: 'one', b: 'two'} $list */
function optional(array $list): void {
    foreach ($list as $key => $value) {
        if ($key === 'a') {
            $value; // observe
        }
    }
}

/** @param array{a: 'one', b: 'two'}|array{a: 'three', c: 'four'} $list */
function alternatives(array $list): void {
    foreach ($list as $key => $value) {
        if ($key === 'b') {
            $value; // observe
        }
    }
}

/** @template T of array{a: 'one', b: 'two'}
 * @param T $list
 */
function template(array $list): void {
    foreach ($list as $key => $value) {
        if ($key === 'a') {
            $value; // observe
        }
    }
}

function offsetValue(): void {
    $list = ['a' => 'one', 'b' => 'two'];
    $target = ['value' => 'seed'];
    foreach ($list as $key => $target['value']) {
        if ($key === 'a') {
            $target['value']; // observe
        }
    }
}

function offsetKey(): void {
    $list = ['a' => 'one', 'b' => 'two'];
    $target = ['key' => 'seed'];
    foreach ($list as $target['key'] => $value) {
        if ($target['key'] === 'a') {
            $value; // observe
        }
    }
}

function valueOnly(): void {
    foreach (['a' => 'one', 'b' => 'two'] as $value) {
        $value; // observe
    }
}

/** @param array<int, object|'fetch'>&callable $list */
function callableArray(array $list): void {
    foreach ($list as $key => $value) {
        if ($key === 1) {
            $value; // observe
        }
    }
}

function overwrittenKey(): void {
    $list = ['a' => 'one', 'b' => 'two'];
    foreach ($list as $key => $value) {
        $key = 'a';
        if ($key === 'a') {
            $value; // observe
        }
    }
}

function overwrittenValue(): void {
    $list = ['a' => 'one', 'b' => 'two'];
    foreach ($list as $key => $value) {
        $value = 'replacement';
        if ($key === 'a') {
            $value; // observe
        }
    }
}

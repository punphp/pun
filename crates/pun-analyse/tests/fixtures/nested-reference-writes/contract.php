<?php

namespace NestedReferenceContract;

final class Probe {
    public function record(mixed ...$values): void {}
}
function observe(array $value): void {}
function identity(int $value): int { return $value; }

function nativeControl(int $value): void {
    $copy = $value; // probe-native
    identity($copy /* probe-native */); // probe
    $array = [];
    replace($array);
    identity($value); // probe
}

/** @param-out array{done: 7} $value */
function replace(array &$value): void { $value = ['done' => 7]; }

/** @return 'slot' */
function keyName(): string { return 'slot'; }

/** @param list<int> $seed */
function straight(Probe $probe, array $seed, string $key): void {
    $array = [$key => $seed];
    $untouched = 23;
    array_unshift($array[$key], 7);
    $probe->record($array[$key], $array, $untouched); // probe
    array_push($array[$key], 9);
    $probe->record($array[$key], $array, $untouched); // probe
    observe($array[$key]);
    $probe->record($array[$key], $array, $untouched); // probe
    replace($array[$key]);
    $probe->record($array[$key], $array[$key]['done'], $untouched); // probe
}

function literal(Probe $probe): void {
    $array = ['slot' => ['done' => 3], 'keep' => 23];
    $probe->record($array['slot']['done']); // probe
    replace($array['slot']);
    $probe->record($array['slot'], $array['slot']['done'], $array['keep']); // probe
}

function complex(Probe $probe): void {
    $array = ['slot' => ['done' => 3], 'keep' => 23];
    replace($array[keyName()]);
    $probe->record($array['slot'], $array['slot']['done'], $array['keep']); // probe
}

function mutate(Probe $probe, array $items, string $key): void {
    $array = [$key => []];
    foreach ($items as $item) {
        array_push($array[$key], $item);
        $probe->record($array[$key]); // probe
        if (count($array[$key]) > 1) {
            throw new \RuntimeException();
        }
    }
}

function prepend(Probe $probe, array $items, string $key): void {
    $array = [$key => []];
    foreach ($items as $item) {
        array_unshift($array[$key], $item);
        $probe->record($array[$key]); // probe
        if (count($array[$key]) > 1) {
            throw new \RuntimeException();
        }
    }
}

function replacement(Probe $probe, array $items, string $key): void {
    $array = [$key => []];
    foreach ($items as $item) {
        replace($array[$key]);
        $probe->record($array[$key], $array[$key]['done']); // probe
    }
}

function nonmutating(Probe $probe, array $items, string $key): void {
    $array = [$key => []];
    foreach ($items as $item) {
        observe($array[$key]);
        $probe->record($array[$key]); // probe
    }
}

function complexLoop(Probe $probe, array $items): void {
    $array = ['slot' => [], 'keep' => 23];
    foreach ($items as $item) {
        replace($array[keyName()]);
        $probe->record($array['slot'], $array['slot']['done'], $array['keep']); // probe
    }
}

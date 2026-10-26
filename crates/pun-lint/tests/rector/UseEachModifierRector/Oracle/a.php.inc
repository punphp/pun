<?php

test('a', function () {
    // comment before foreach
    foreach ($items as $item) {
        // comment inside
        expect($item)
            ->toBeString()
            ->not->toBeEmpty();
    }

    foreach ($this->items() as $k => $item) {
        expect($item)->toBe(fn () => $k);
    }

    foreach ($items as $k => $item) {
        expect($item)->toBe(fn () => $other, foo: 1, ...$rest);
    }

    foreach ($items as $item) expect($item)->toBeInt();

    foreach ($items as $item):
        expect($item)->toBeInt();
    endforeach;

    foreach ($items as &$item) {
        expect($item, 'message')->toBeInt();
    }

    foreach ($items as $item) {
        expect(value: $item)->toBe(function () {
            return 1;
        });
    }

    foreach ($items as $item) {
        expect($item)->toBeInt()->and($item)->toBe(1);
    }

    foreach ($items as $item) {
        expect($item)->each->toBeInt();
    }

    foreach ($items as $item) {
        expect($other)->toBeInt();
    }

    foreach ($items as [$a, $b]) {
        expect($a)->toBeInt();
    }

    foreach ($items as $item) {
        \expect($item)->toBeInt();
    }

    foreach ($items as $item) {
        Expect($item)->toBeInt()->{'toBe'}(1)->{$m}(2)->$n(3);
    }

    foreach ($items as $item) {
        expect($item)->toBeInt(); // trailing
    }

    foreach ($items as $item) {
        expect(($item))->toBeInt(/* c */);
    }

    if ($x) {
        foreach ($items as $item) {
            expect($item)->toBeInt();
        }
    }

    foreach (($items) as $item) {
        expect($item)->toBeInt();
    }

    foreach ($items as $item) {
        expect($item)->toBeInt(
            1,
            2,
        );
    }
    foreach ($items as $item) {
        expect($item)->toBe([
            'a' => 1,
        ]);
    }
});
-----
<?php

test('a', function () {
    expect($items)->each->toBeString()->not->toBeEmpty();

    foreach ($this->items() as $k => $item) {
        expect($item)->toBe(fn () => $k);
    }

    expect($items)->each->toBe(fn () => $other, foo: 1, ...$rest);

    expect($items)->each->toBeInt();

    expect($items)->each->toBeInt();

    expect($items, 'message')->each->toBeInt();

    expect($items)->each->toBe(function () {
        return 1;
    });

    foreach ($items as $item) {
        expect($item)->toBeInt()->and($item)->toBe(1);
    }

    expect($items)->each->each->toBeInt();

    foreach ($items as $item) {
        expect($other)->toBeInt();
    }

    foreach ($items as [$a, $b]) {
        expect($a)->toBeInt();
    }

    \expect($items)->each->toBeInt();

    Expect($items)->each->toBeInt()->{'toBe'}(1)->{$m}(2)->{$n}(3);

    foreach ($items as $item) {
        expect($item)->toBeInt(); // trailing
    }

    expect($items)->each->toBeInt();

    if ($x) {
        expect($items)->each->toBeInt();
    }

    expect($items)->each->toBeInt();

    expect($items)->each->toBeInt(1, 2);
    expect($items)->each->toBe([
        'a' => 1,
    ]);
});

<?php

test('a', function (array $list, iterable $it, string $s, $untyped, \Traversable $t, \Countable $c, \ArrayIterator $ai) {
    expect($list)->each->toBeInstanceOf(User::class);
    expect($it)->each->toBeInstanceOf(User::class);
    expect($s)->each->toBeInstanceOf(User::class);
    expect($untyped)->each->toBeInstanceOf(User::class);
    expect($t)->each->toBeInstanceOf(User::class);
    expect($c)->each->toBeInstanceOf(User::class);
    expect($ai)->each->toBeInstanceOf(User::class);
    expect($undefined)->each->toBeInstanceOf(User::class);
    expect($list)->not->each->toBeInstanceOf(User::class);
    expect($list)->each->not->toBeInstanceOf(User::class);
    expect($list)->each->toBeInstanceOf(User::class, 'x');
    expect($list)->each->toBeInstanceOf();
    expect($list)->toBeInstanceOf(User::class);
    expect($list)
        ->each
        ->toBeInstanceOf(User::class);
    expect($a)->and($list)->each->toBeInstanceOf(User::class);
    expect($list)->each->TOBEINSTANCEOF(User::class);
    expect($list)->EACH->toBeInstanceOf(User::class);
    expect([1, 2])->each->toBeInstanceOf(User::class);
    expect(1)->each->toBeInstanceOf(User::class);
    expect(null)->each->toBeInstanceOf(User::class);
});
-----
<?php

test('a', function (array $list, iterable $it, string $s, $untyped, \Traversable $t, \Countable $c, \ArrayIterator $ai) {
    expect($list)->toContainOnlyInstancesOf(User::class);
    expect($it)->toContainOnlyInstancesOf(User::class);
    expect($s)->each->toBeInstanceOf(User::class);
    expect($untyped)->each->toBeInstanceOf(User::class);
    expect($t)->toContainOnlyInstancesOf(User::class);
    expect($c)->each->toBeInstanceOf(User::class);
    expect($ai)->toContainOnlyInstancesOf(User::class);
    expect($undefined)->each->toBeInstanceOf(User::class);
    expect($list)->not->each->toBeInstanceOf(User::class);
    expect($list)->each->not->toBeInstanceOf(User::class);
    expect($list)->each->toBeInstanceOf(User::class, 'x');
    expect($list)->each->toBeInstanceOf();
    expect($list)->toBeInstanceOf(User::class);
    expect($list)
        ->toContainOnlyInstancesOf(User::class);
    expect($a)->and($list)->toContainOnlyInstancesOf(User::class);
    expect($list)->toContainOnlyInstancesOf(User::class);
    expect($list)->toContainOnlyInstancesOf(User::class);
    expect([1, 2])->toContainOnlyInstancesOf(User::class);
    expect(1)->each->toBeInstanceOf(User::class);
    expect(null)->each->toBeInstanceOf(User::class);
});

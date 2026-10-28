<?php
class BoundarySink {
    public function foo($value) {}
    public function bar($value) {}
}
/** @template T
 * @param T ...$values
 */
function boundaryVariadic(...$values): int { return 1; }
function boundaryReceiver(): BoundarySink { return new BoundarySink(); }
function boundaryCalls(BoundarySink $sink, bool $condition): void {
    if ($condition) {
        $name = 'foo';
        $values = [1];
    } else {
        $name = 'bar';
        $values = ['value'];
    }
    $sink->$name(boundaryVariadic(...$values)); // observe
    $value = 7;
    $sink->foo((function (): int {
        unset($value);
        [$value];
        return 7;
    })()); // observe
    boundaryReceiver()->$name([7]); // observe
}
$sink = new BoundarySink();
$name = 'foo';
$sink->foo([$ambient]); // observe
$sink->$name([$ambient]); // observe
function boundarySingle($value): void {}
function boundarySkipped(): array { return [7]; }
boundarySingle(...boundarySkipped());

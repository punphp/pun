<?php
class PreparedPositionSink {
    public ?self $child = null;
    public function foo($value, $later = null, $last = null): ?self { return $this; }
}
class PreparedPositionBox {
    /** @var array{slot: PreparedPositionSink|null} */
    public array $slots;
}
/** @return array{old: 0} */
function preparedPositionRoot(): array { return ['old' => 0]; }
function preparedPositionIndex(): string { return 'old'; }
/** @param array{old: 0, side: 0} $rows */
function preparedPositionReads(PreparedPositionSink $sink, PreparedPositionBox $box, array $rows): void {
    $sink->foo($born = 1, [$born]); // observe
    $sink->foo($rows['old'] = 2, [$rows]); // observe
    $sink->foo(preparedPositionRoot()[preparedPositionIndex()] = 2); // observe
    $sink->foo($sink->child = null, [$sink->child]); // observe
    $name = 'child';
    $sink->foo($sink->$name = null, [$sink->child]); // observe
    $sink->foo($sink->{'child'} = null, [$sink->child]); // observe
    $nested = ['a' => ['b' => 0]];
    $sink->foo($nested['a']['b'] = 2, [$nested]); // observe
    $append = [];
    $sink->foo($append[] = 2, [$append]); // observe
    $box->slots['slot']?->foo($box->slots['slot'] = null, [$box->slots]); // observe
    ($assigned = $sink)?->foo($assigned, [$assigned]); // observe
}

function preparedMissingOffsets(PreparedPositionSink $sink): void {
    $empty = [];
    $sink->foo($empty['new'] = 1); // observe
    $sink->foo($empty['second']['leaf'] = 1); // observe
}

class PreparedPositionStatics { public static int $stored = 0; }
function preparedPropertyLinks(PreparedPositionSink $sink): void {
    $sink->child = $sink;
    $sink->foo($sink?->child); // observe
    $sink->foo(PreparedPositionStatics::$stored = 1); // observe
}

function preparedDynamicName(PreparedPositionSink $sink): void {
    $sink->child = null;
    $sink->foo($sink->{($property = 'child')} = null); // observe
}

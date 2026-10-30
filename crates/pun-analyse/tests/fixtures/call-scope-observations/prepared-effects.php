<?php
class PreparedEffectSink {
    public function foo($value): ?self { return $this; }
}
class PreparedEffectBox {
    /** @var array{slot: PreparedEffectSink|null} */
    public array $slots;
}
function preparedEffectBox(): PreparedEffectBox { return new PreparedEffectBox(); }
function preparedEffectValue(): ?PreparedEffectSink { return null; }
function preparedEffectPositions(PreparedEffectSink $sink): void {
    $sink->foo(preparedEffectBox()->slots['slot'] = preparedEffectValue()); // observe
}

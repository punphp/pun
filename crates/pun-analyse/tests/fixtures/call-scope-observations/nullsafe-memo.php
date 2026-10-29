<?php
class nullsafememosink {
    public ?self $child = null;
    /** @phpstan-pure */
    public function read(): ?self { return $this->child; }
    public function foo($value) {}
    public function bar($value) {}
}
function nullsafeMemoHolder(nullsafememosink $sink, bool $condition): void {
    $name = $condition ? 'foo' : 'bar';
    $sink->read()?->$name([$sink->read()]); // observe
}
function nullsafeNoTypeFactory(): ?nullsafememosink { return null; }
function nullsafeNoTypeReceiver(): void {
    nullsafeNoTypeFactory()?->foo(0); // observe
}
function nullsafeMemoOneName(nullsafememosink $sink): void {
    $name = 'foo';
    $sink->read()?->$name([$sink->read()]); // observe
}

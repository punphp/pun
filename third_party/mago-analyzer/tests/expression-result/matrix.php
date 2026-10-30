<?php
function resultVoid(): void {}
function resultProvider($value = null): mixed { return $value; }
function resultEager(): mixed {}
function resultOverride(): void {}
class ResultSink {
    public function read(): ?self { return null; }
    public function stop(): void {}
    public static function staticStop(): void {}
    public function fromProvider(): mixed {}
}
function resultMatrix(ResultSink $sink, callable $callback): void {
    resultVoid()/* result */;
    (resultVoid())/* result */;
    $sink->stop()/* result */;
    ResultSink::staticStop()/* result */;
    $method = 'stop';
    $sink->$method()/* result */;
    $static = 'staticStop';
    ResultSink::$static()/* result */;
    $f = resultVoid(...);
    $f()/* result */;
    'resultVoid'()/* result */;
    $methodValue = [$sink, 'stop'];
    $methodValue()/* result */;
    (resultVoid(...))()/* result */;
    resultProvider(resultVoid())/* result */;
    $sink->fromProvider()/* result */;
    resultEager()/* result */;
    resultOverride()/* result */;
    $values = [resultVoid()];
    $values/* result */;
}
function resultNative(): void {
    resultVoid()/* result */;
}
function resultNativeProvider(): void {
    resultProvider(resultVoid())/* result */;
}
class ResultHolder {
    public function read(): ?self { return null; }
    public function foo(array $values): void {}
}
function resultHeld(ResultHolder $sink): void {
    $sink->read()?->foo([$sink->read()/* result */]);
}
function resultFallbacks(): void {
    resultMissing()/* result */;
    $nothing = null;
    $nothing?->stop()/* result */;
}

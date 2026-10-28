<?php
class ContextSink {
    public function foo($value) {}
    public function bar($value) {}
}
class IntegerContext {
    /**
     * @template T
     * @param Closure(int): T $callback
     * @return T
     */
    public function read(Closure $callback) { return $callback(1); }
}
class StringContext {
    /**
     * @template T
     * @param Closure(string): T $callback
     * @return T
     */
    public function read(Closure $callback) { return $callback('value'); }
}
class EqualIntegerContext {
    /**
     * @template T
     * @param Closure(int): T $callback
     * @return T
     */
    public function read(Closure $callback) { return $callback(2); }
}
function argumentContexts(ContextSink $sink, bool $condition): void {
    if ($condition) {
        $name = 'foo';
        $worker = new IntegerContext();
    } else {
        $name = 'bar';
        $worker = new StringContext();
    }
    $sink->$name($worker->read(fn ($value) => $value)); // observe
    $sink->$name($worker->read(function ($value) { return $value; })); // observe
}
function equalArgumentContexts(ContextSink $sink, bool $condition): void {
    if ($condition) {
        $name = 'foo';
        $worker = new IntegerContext();
    } else {
        $name = 'bar';
        $worker = new EqualIntegerContext();
    }
    $sink->$name($worker->read(fn ($value) => $value)); // observe
}

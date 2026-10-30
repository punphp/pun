<?php
class DeclarationSink {
    public function foo($value) {}
    public function bar($value) {}
}
class DeclarationIntegerContext {
    /** @template T
     * @param Closure(int): T $callback
     * @return T
     */
    public function read(Closure $callback) { return $callback(1); }
}
class DeclarationStringContext {
    /** @template T
     * @param Closure(string): T $callback
     * @return T
     */
    public function read(Closure $callback) { return $callback('value'); }
}
/** @template T
 * @template U
 * @param T $input
 * @param Closure(T): U $callback
 * @return U
 */
function declarationMap($input, Closure $callback) { return $callback($input); }
function declarationContexts(DeclarationSink $sink, bool $condition): void {
    if ($condition) {
        $name = 'foo';
        $worker = new DeclarationIntegerContext();
        $value = 1;
    } else {
        $name = 'bar';
        $worker = new DeclarationStringContext();
        $value = 'value';
    }
    $sink->$name($worker->read(function ($value) {
        function pricingLocalFunction($input) { return $input; }
        class PricingLocalClass {}
        interface PricingLocalInterface {}
        trait PricingLocalTrait {}
        enum PricingLocalEnum { case A; }
        return $value;
    })); // observe
    $sink->$name(declarationMap($value, fn ($input) => $input)); // observe
    $sink->$name(declarationMap(1, fn (int $input) => 1)); // observe
}

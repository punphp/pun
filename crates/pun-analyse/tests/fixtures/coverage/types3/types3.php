<?php

namespace Types3;

final class Invokable
{
    public function __invoke(int $a): int
    {
        return $a;
    }

    private function hidden(): void {}

    public static function make(int $a): int
    {
        return $a;
    }
}

final class Plain {}

/**
 * @param callable(int, int): void $typed
 * @param 'strlen'|'trim' $names
 * @param int|Invokable $maybe
 */
function callables(Invokable $inv, Plain $plain, callable $typed, string $names, $maybe, \Nope $unknown, mixed $m): void
{
    $f = 'strlen';
    $_ = $f();
    $c = function (int $a, int $b = 1): void {};
    $_ = $c();
    $x = $c(1);
    $s = 'Types3\Invokable::make';
    $_ = $s();
    $arr = [$inv, 'hidden'];
    $_ = $arr();
    $_ = $inv();
    $_ = $inv('a', 2);
    $_ = $plain();
    $_ = $typed(i: 1);
    $_ = $names();
    $_ = $maybe();
    $_ = $unknown();
    $_ = $m();
    $g = [Invokable::class, 'make'];
    $_ = $g('x');
    $dyn = [$inv, $names];
    $_ = $dyn();
    $_ = call_user_func('strlen');
    $_ = call_user_func(callback: 'strlen', string: 'a', extra: 1);
    $_ = call_user_func_array('strlen', ['string' => 1, 'b' => 2]);
    $_ = call_user_func_array('strlen', [1 => 'a', '2' => 'b']);
    $_ = call_user_func_array('strlen', [...$names]);
    $_ = call_user_func_array('strlen', [$names => 1]);
    $_ = call_user_func_array('strlen', $m);
    $_ = call_user_func_array('strlen', ['' => 1]);
    $_ = call_user_func_array(args: ['a'], callback: 'strlen');
    $_ = call_user_func($m);
    $_ = call_user_func([$inv, 'hidden']);
    $_ = call_user_func('nope_function');
    $_ = call_user_func(args: 1);
    $_ = call_user_func_array('strlen');
}

function regexes(string $s, string $d): void
{
    preg_match('/a' . preg_quote($s) . '/', $s);
    preg_match('#a' . preg_quote($s) . '#', $s);
    preg_match('/a' . preg_quote($s, '#') . '/', $s);
    preg_match('/a' . preg_quote($s, '~') . '/', $s);
    preg_match('/a' . preg_quote($s, '/') . '/', $s);
    preg_match('/a' . preg_quote($s, 'ab') . '/', $s);
    preg_match(('/a' . 'b') . preg_quote($s, '~') . '/', $s);
    preg_match(preg_quote($s) . '/a/', $s);
    preg_match(pattern: '/a' . preg_quote($s) . '/', subject: $s);
    preg_match('/a', $s);
    $p = rand(0, 1) ? '/a' : '~a';
    preg_match($p . preg_quote($s) . '/', $s);
    preg_match($p . preg_quote($s, '%') . '/', $s);
    preg_match($d . preg_quote($s) . '/', $s);
    preg_match(' ' . preg_quote($s) . '/', $s);
    \preg_match('/a' . \preg_quote($s, delimiter: '~') . '/', $s);
    preg_match(...['/a' . preg_quote($s) . '/', $s]);
}

trait SomeTrait {}

/**
 * @template T of Missing
 * @param class-string<T> $a
 * @param array<int, NotAValue> $b
 * @param array{a: int, ...<int, InExtras>} $c
 * @param-out OutClass $d
 * @param-closure-this ThisClass $e
 * @param list<SomeTrait> $f
 * @param callable(CallableParam): CallableReturn $g
 * @param object{a: ShapeProp} $h
 * @param ($a is string ? CondThen : CondElse) $i
 * @return self|parent|static
 */
function phpdoc($a, $b, $c, &$d, $e, $f, $g, $h, $i)
{
}

/**
 * @return SomeTrait|ReturnMissing
 */
function phpdocReturn(): object
{
}

/**
 * @template C of Missing2
 */
class WithTemplates
{
    /**
     * @param C $c
     * @param ?Nullable $n
     * @param Arr[] $arr
     * @param int $int
     */
    public function method($c, $n, $arr, $int): void
    {
    }
}

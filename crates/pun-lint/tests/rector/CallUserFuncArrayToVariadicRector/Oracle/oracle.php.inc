<?php

class Redirector { public function redirect($a) {} }

final class A
{
    private $untyped;
    private $assigned;
    private Redirector $typed;
    private ?Redirector $nullable;

    public function __construct()
    {
        $this->assigned = new Redirector();
    }

    public function run($param, Redirector $r, $items)
    {
        call_user_func_array([$this->untyped, 'redirect'], $items);
        call_user_func_array([$this->typed, 'redirect'], $items);
        call_user_func_array([$this->nullable, 'redirect'], $items);
        call_user_func_array([$param, 'redirect'], $items);
        call_user_func_array([$r, 'redirect'], $items);
        call_user_func_array([$this, 'run'], $items);
        call_user_func_array(['A', 'run'], $items);
        call_user_func_array("str" . "len", $items);
        call_user_func_array('Foo\bar', [1, 2]);
        call_user_func_array(callback: 'strlen', args: $items);
        call_user_func_array('strlen', $items, 3);
        $x = new Redirector();
        call_user_func_array([$x, 'redirect'], $items);
        return CALL_USER_FUNC_ARRAY('strlen', $items);
    }
}

function more(A $a, $items)
{
    call_user_func_array(['k' => $a, 'v' => 'run'], $items);
    call_user_func_array(array($a, 'run'), $items);
    call_user_func_array([$a, $items], $items);
    call_user_func_array([$a::$b, 'run'], $items);
    call_user_func_array($a->run(1), $items);
    call_user_func_array(strlen(...), $items);
    call_user_func_array("str\x6cen", $items ?: []);
    \call_user_func_array('strlen', [1]);
    call_user_func_array([$a, 'run', 3], $items);
}
-----
<?php

class Redirector { public function redirect($a) {} }

final class A
{
    private $untyped;
    private $assigned;
    private Redirector $typed;
    private ?Redirector $nullable;

    public function __construct()
    {
        $this->assigned = new Redirector();
    }

    public function run($param, Redirector $r, $items)
    {
        call_user_func_array([$this->untyped, 'redirect'], $items);
        $this->typed->redirect(...$items);
        call_user_func_array([$this->nullable, 'redirect'], $items);
        call_user_func_array([$param, 'redirect'], $items);
        $r->redirect(...$items);
        $this->run(...$items);
        call_user_func_array(['A', 'run'], $items);
        call_user_func_array("str" . "len", $items);
        Foo\bar(...[1, 2]);
        strlen(...$items);
        strlen(...$items);
        $x = new Redirector();
        $x->redirect(...$items);
        return strlen(...$items);
    }
}

function more(A $a, $items)
{
    $a->run(...$items);
    $a->run(...$items);
    call_user_func_array([$a, $items], $items);
    call_user_func_array([$a::$b, 'run'], $items);
    call_user_func_array($a->run(1), $items);
    call_user_func_array(strlen(...), $items);
    strlen(...$items ?: []);
    strlen(...[1]);
    call_user_func_array([$a, 'run', 3], $items);
}

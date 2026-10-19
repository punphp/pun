<?php

class Redirector { public function redirect($a) {} }

final class Inferred
{
    private $assigned;
    private $reassigned;
    public $public;
    private $twice, $other;
    private $compound;
    private $never;

    public function __construct()
    {
        $this->assigned = new Redirector();
        $this->public = new Redirector();
        $this->twice = 1;
        $this->twice = new Redirector();
        $this->reassigned = new Redirector();
        $this->reassigned = 'text';
        $this->compound = new Redirector();
        $this->compound .= 'x';
        $other->never = new Redirector();
        $this->{'never'} = new Redirector();
        foo();
    }

    public function run($items)
    {
        call_user_func_array([$this->assigned, 'redirect'], $items);
        call_user_func_array([$this->public, 'redirect'], $items);
        call_user_func_array([$this->twice, 'redirect'], $items);
        call_user_func_array([$this->reassigned, 'redirect'], $items);
        call_user_func_array([$this->compound, 'redirect'], $items);
        call_user_func_array([$this->never, 'redirect'], $items);
        call_user_func_array([$that->assigned, 'redirect'], $items);
        call_user_func_array([[...$items][0], 'redirect'], $items);
        call_user_func_array([&$this->assigned, 'redirect'], $items);
        call_user_func_array(['x' => $this->assigned, 'y' => 'redirect'], $items);
    }
}

final class NoConstructor
{
    private $assigned;

    public function run($items)
    {
        call_user_func_array([$this->assigned, 'redirect'], $items);
    }
}

abstract class AbstractConstructor
{
    private $assigned;

    abstract public function __construct();

    public function run($items)
    {
        call_user_func_array([$this->assigned, 'redirect'], $items);
        return new class {
            private $assigned;

            public function __construct()
            {
                $this->assigned = new Redirector();
            }

            public function run($items)
            {
                call_user_func_array([$this->assigned, 'redirect'], $items);
            }
        };
    }
}
-----
<?php

class Redirector { public function redirect($a) {} }

final class Inferred
{
    private $assigned;
    private $reassigned;
    public $public;
    private $twice, $other;
    private $compound;
    private $never;

    public function __construct()
    {
        $this->assigned = new Redirector();
        $this->public = new Redirector();
        $this->twice = 1;
        $this->twice = new Redirector();
        $this->reassigned = new Redirector();
        $this->reassigned = 'text';
        $this->compound = new Redirector();
        $this->compound .= 'x';
        $other->never = new Redirector();
        $this->{'never'} = new Redirector();
        foo();
    }

    public function run($items)
    {
        $this->assigned->redirect(...$items);
        call_user_func_array([$this->public, 'redirect'], $items);
        $this->twice->redirect(...$items);
        call_user_func_array([$this->reassigned, 'redirect'], $items);
        $this->compound->redirect(...$items);
        call_user_func_array([$this->never, 'redirect'], $items);
        call_user_func_array([$that->assigned, 'redirect'], $items);
        call_user_func_array([[...$items][0], 'redirect'], $items);
        $this->assigned->redirect(...$items);
        $this->assigned->redirect(...$items);
    }
}

final class NoConstructor
{
    private $assigned;

    public function run($items)
    {
        call_user_func_array([$this->assigned, 'redirect'], $items);
    }
}

abstract class AbstractConstructor
{
    private $assigned;

    abstract public function __construct();

    public function run($items)
    {
        call_user_func_array([$this->assigned, 'redirect'], $items);
        return new class {
            private $assigned;

            public function __construct()
            {
                $this->assigned = new Redirector();
            }

            public function run($items)
            {
                call_user_func_array([$this->assigned, 'redirect'], $items);
            }
        };
    }
}

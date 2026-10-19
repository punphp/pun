<?php

abstract class Base
{
    abstract public function none();

    public function same()
    {
        $args = func_get_args();
        $inner = function () {
            $args = func_get_args();
            return $args;
        };
        return [$args, $inner];
    }

    public function second()
    {
        echo 1;
        $first = FUNC_GET_ARGS();
        $second = func_get_args();
        return [$first, $second];
    }

    public function nested()
    {
        if (true) {
            $args = func_get_args();
        }
        $this->items[] = func_get_args();
        return $args;
    }

    public function compound()
    {
        $args .= func_get_args();
        $more = func_num_args();
        return $args;
    }

    public function scoped()
    {
        $fn = fn () => $args = func_get_args();
        new class {
            public function run()
            {
                echo 1;
            }
        };
        return $fn;
    }

}

function only_nested()
{
    if (rand()) {
        $args = func_get_args();
    }
}

function with_comment()
{
    // the arguments
    $values = func_get_args();

    return $values;
}

$x = function () {
    $list = func_get_args();
    $f = function () {
        $list = \func_get_args();
    };
};
-----
<?php

abstract class Base
{
    abstract public function none();

    public function same()
    {
        $inner = function (...$args) {
            return $args;
        };
        return [$args, $inner];
    }

    public function second(...$first)
    {
        echo 1;
        $second = func_get_args();
        return [$first, $second];
    }

    public function nested(...$args)
    {
        if (true) {
            $args = func_get_args();
        }
        $this->items[] = $args;
        return $args;
    }

    public function compound()
    {
        $args .= func_get_args();
        $more = func_num_args();
        return $args;
    }

    public function scoped()
    {
        $fn = fn () => $args = func_get_args();
        new class {
            public function run()
            {
                echo 1;
            }
        };
        return $fn;
    }

}

function only_nested()
{
    if (rand()) {
        $args = func_get_args();
    }
}

function with_comment(...$values)
{
    return $values;
}

$x = function () {
    $f = function (...$list) {
    };
};

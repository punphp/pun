<?php

namespace App;

class Holder
{
}

final class Closed
{
}

class CustomA extends \Exception
{
    public function __construct(string $message = '', int $code = 0, ?Holder $holder = null, ?\Throwable $previous = null)
    {
        parent::__construct($message, $code, $previous);
    }
}

class CustomB extends \Exception
{
    public function __construct(string $message = '', int $code = 0, ?Closed $holder = null, ?\Throwable $previous = null)
    {
        parent::__construct($message, $code, $previous);
    }
}

class CustomC extends \Exception
{
    public function __construct(string $message, object $thing, ?\Throwable $previous = null)
    {
        parent::__construct($message, 0, $previous);
    }
}

class Runner
{
    public function run($x)
    {
        try {
            $x();
        } catch (\Throwable $e) {
            throw new CustomA('m', 1, null);
        }
        try {
            $x();
        } catch (\Throwable $e) {
            throw new CustomB('m', 1, null);
        }
        try {
            $x();
        } catch (\Throwable $e) {
            throw new CustomC('m');
        }
        try {
            $x();
        } catch (\RuntimeException $outer) {
            try {
                $x();
            } catch (\LogicException $inner) {
                throw new \RuntimeException('nested');
            }
        }
        try {
            $x();
        } catch (\Exception $e) {
            $y = $x ?? throw new \LogicException(sprintf(
                'Failed %s',
                $x
            ));
            throw new \RuntimeException(
                'multi',
                5
            );
        }
        try {
            $x();
        } catch (\Exception $e) {
            throw new \RuntimeException(
                // the message
                'commented',
            );
        }
        try {
            $x();
        } catch (\Exception $e) {
            throw new Unknown(previous : $e, other: 1);
        }
        try {
            $x();
        } catch (\Exception $e) {
            $f = function () use ($e) {
                throw new \InvalidArgumentException('closure', array_map(fn ($v) => [
                    'a' => $v,
                ], [1])[0]);
            };
        }
        try {
            $x();
        } catch (\Exception) {
            throw new \RuntimeException('no var');
        }
        try {
            $x();
        } catch (\Exception $e) {
            throw new self('self');
        }
    }
}
-----
<?php

namespace App;

class Holder
{
}

final class Closed
{
}

class CustomA extends \Exception
{
    public function __construct(string $message = '', int $code = 0, ?Holder $holder = null, ?\Throwable $previous = null)
    {
        parent::__construct($message, $code, $previous);
    }
}

class CustomB extends \Exception
{
    public function __construct(string $message = '', int $code = 0, ?Closed $holder = null, ?\Throwable $previous = null)
    {
        parent::__construct($message, $code, $previous);
    }
}

class CustomC extends \Exception
{
    public function __construct(string $message, object $thing, ?\Throwable $previous = null)
    {
        parent::__construct($message, 0, $previous);
    }
}

class Runner
{
    public function run($x)
    {
        try {
            $x();
        } catch (\Throwable $e) {
            throw new CustomA('m', 1, null, $e);
        }
        try {
            $x();
        } catch (\Throwable $e) {
            throw new CustomB('m', 1, null, $e);
        }
        try {
            $x();
        } catch (\Throwable $e) {
            throw new CustomC('m');
        }
        try {
            $x();
        } catch (\RuntimeException $outer) {
            try {
                $x();
            } catch (\LogicException $inner) {
                throw new \RuntimeException('nested', $outer->getCode(), $outer);
            }
        }
        try {
            $x();
        } catch (\Exception $e) {
            $y = $x ?? throw new \LogicException(sprintf(
                'Failed %s',
                $x
            ), $e->getCode(), $e);
            throw new \RuntimeException('multi', 5, $e);
        }
        try {
            $x();
        } catch (\Exception $e) {
            throw new \RuntimeException(
                // the message
                'commented',
                $e->getCode(),
                $e
            );
        }
        try {
            $x();
        } catch (\Exception $e) {
            throw new Unknown(message : $e->getMessage(), other: 1);
        }
        try {
            $x();
        } catch (\Exception $e) {
            $f = function () use ($e) {
                throw new \InvalidArgumentException('closure', array_map(fn ($v) => [
                    'a' => $v,
                ], [1])[0], $e);
            };
        }
        try {
            $x();
        } catch (\Exception) {
            throw new \RuntimeException('no var');
        }
        try {
            $x();
        } catch (\Exception $e) {
            throw new self('self');
        }
    }
}

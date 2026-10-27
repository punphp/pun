<?php

namespace App\Previous;

use Exception;
use Throwable;

final class NoCode extends Exception
{
    public function __construct(string $message, ?Throwable $previous = null)
    {
        parent::__construct($message, 0, $previous);
    }
}

final class Runner
{
    public function rethrow(): void
    {
        try {
            $this->run();
        } catch (Exception $e) {
            throw $e;
        }
    }

    public function dynamic(string $class): void
    {
        try {
            $this->run();
        } catch (Exception $e) {
            throw new $class('failed');
        }
    }

    public function namedPrevious(Exception $other): void
    {
        try {
            $this->run();
        } catch (Exception $e) {
            throw new NoCode('failed', previous: $other);
        }
    }

    private function run(): void
    {
    }
}

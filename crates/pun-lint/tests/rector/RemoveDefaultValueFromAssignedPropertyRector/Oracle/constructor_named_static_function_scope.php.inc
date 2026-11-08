<?php

namespace ConstructorFunctionScope;

final class ArrayWriterInFunction
{
    private array $value = [];

    public function __construct()
    {
        $this->value = [];
    }

    public function declareWriter(): void
    {
        function writeArray(): void
        {
            ArrayWriterInFunction::$value[] = 1;
        }
    }
}

final class ConditionalWriterInFunction
{
    private int $value = 0;

    public function __construct()
    {
        function writeValue(bool $flag): void
        {
            if ($flag) {
                ConditionalWriterInFunction::$value = 2;
            } else {
                ConditionalWriterInFunction::$value = 3;
            }
        }
    }
}

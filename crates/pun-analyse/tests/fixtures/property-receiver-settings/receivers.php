<?php

declare(strict_types=1);

namespace PropertyReceiverSettings;

class Subject
{
    public int $value = 1;
    public static int $shared = 1;

    public function probe(Subject $ordinary, ?Subject $nullable, int $number): void
    {
        echo $this->missingThis;
        echo ($this)->missingParenthesizedThis;
        echo $this?->missingNullsafeThis;
        echo ($this)?->missingParenthesizedNullsafeThis;

        $copy = $this;
        echo $copy->missingCopy;
        echo ($copy)->missingParenthesizedCopy;
        echo $ordinary->missingOrdinary;
        echo ($ordinary)->missingParenthesizedOrdinary;
        echo $number->missingOnNumber;
        echo $nullable?->missingNullable;
        echo ($nullable)?->missingParenthesizedNullable;

        echo $this::$missingStaticThis;
        echo ($this)::$missingParenthesizedStaticThis;
        echo $copy::$missingStaticCopy;
        echo ($copy)::$missingParenthesizedStaticCopy;
        echo $ordinary::$missingDynamicClass;
        echo ($ordinary)::$missingParenthesizedDynamicClass;
        echo $number::$missingStaticOnNumber;

        echo self::$missingNamedSelf;
        echo static::$missingNamedStatic;
        echo Subject::$missingNamedClass;

        echo $this->value;
        echo ($this)->value;
        echo $ordinary->value;
        echo $nullable?->value;
        echo $this::$shared;
        echo ($this)::$shared;
        echo $ordinary::$shared;
    }
}

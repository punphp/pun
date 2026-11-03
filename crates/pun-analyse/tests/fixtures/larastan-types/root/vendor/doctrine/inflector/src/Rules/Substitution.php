<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules;

final class Substitution
{
    /** @var Word */
    private $from;

    /** @var Word */
    private $to;

    public function __construct(Word $from, Word $to)
    {}

    public function getFrom(): Word
    {}

    public function getTo(): Word
    {}
}

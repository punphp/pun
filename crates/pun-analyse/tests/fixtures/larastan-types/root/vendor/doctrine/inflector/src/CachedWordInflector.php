<?php

declare(strict_types=1);

namespace Doctrine\Inflector;

class CachedWordInflector implements WordInflector
{
    /** @var WordInflector */
    private $wordInflector;

    /** @var string[] */
    private $cache = [];

    public function __construct(WordInflector $wordInflector)
    {}

    public function inflect(string $word): string
    {}
}

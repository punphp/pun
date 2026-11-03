<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Yaml;

use Symfony\Component\Yaml\Exception\ParseException;
use Symfony\Component\Yaml\Tag\TaggedValue;

/**
 * @internal
 */
final class ParserState
{
    public int $maxNestingLevel = Parser::DEFAULT_MAX_NESTING_LEVEL;
    public int $currentNestingLevel = 0;
    public int $maxAliasesForCollections = Parser::DEFAULT_MAX_ALIASES_FOR_COLLECTIONS;
    public int $collectionAliasCount = 0;
    public bool $aliasesEnabled = true;

    public function reset(): void
    {}

    public function enterNestingLevel(int $line, ?string $snippet, ?string $filename): void
    {}

    public function leaveNestingLevel(): void
    {}

    public function countAlias(mixed $refValue, int $line, ?string $snippet, ?string $filename): void
    {}
}

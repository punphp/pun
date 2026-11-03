<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * Original code based on the CommonMark JS reference parser (https://bitly.com/commonmark-js)
 *  - (c) John MacFarlane
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Reference;

use League\CommonMark\Normalizer\TextNormalizer;

/**
 * A collection of references, indexed by label
 */
final class ReferenceMap implements ReferenceMapInterface
{
    /** @psalm-readonly */
    private TextNormalizer $normalizer;

    /**
     * @var array<string, ReferenceInterface>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $references = [];

    public function __construct()
    {}

    public function add(ReferenceInterface $reference): void
    {}

    public function contains(string $label): bool
    {}

    public function get(string $label): ?ReferenceInterface
    {}

    /**
     * @return \Traversable<string, ReferenceInterface>
     */
    public function getIterator(): \Traversable
    { yield; }

    public function count(): int
    {}
}

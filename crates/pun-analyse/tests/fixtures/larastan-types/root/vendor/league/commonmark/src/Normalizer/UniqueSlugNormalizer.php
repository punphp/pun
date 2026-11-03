<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Normalizer;

// phpcs:disable Squiz.Strings.DoubleQuoteUsage.ContainsVar
final class UniqueSlugNormalizer implements UniqueSlugNormalizerInterface
{
    private TextNormalizerInterface $innerNormalizer;

    /**
     * Slugs claimed by the surrounding page (in final, normalized form), seeded as if they'd
     * already been handed out once, so the first colliding slug gets a "-1" suffix.
     * Unlike regular history, these survive clearHistory().
     *
     * @var array<string, int>
     */
    private array $reserved = [];

    /**
     * Every slug we've handed out, mapped to the next numeric suffix to try for it
     *
     * @var array<string, int>
     */
    private array $alreadyUsed;

    /**
     * @param iterable<string> $reservedSlugs
     */
    public function __construct(TextNormalizerInterface $innerNormalizer, iterable $reservedSlugs = [])
    {}

    public function clearHistory(): void
    {}

    /**
     * {@inheritDoc}
     *
     * @psalm-allow-private-mutation
     */
    public function normalize(string $text, array $context = []): string
    {}
}

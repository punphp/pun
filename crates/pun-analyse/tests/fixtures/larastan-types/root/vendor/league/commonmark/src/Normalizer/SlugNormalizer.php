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

use League\Config\ConfigurationAwareInterface;
use League\Config\ConfigurationInterface;

/**
 * Creates URL-friendly strings based on the given string input
 */
final class SlugNormalizer implements TextNormalizerInterface, ConfigurationAwareInterface
{
    /** @psalm-allow-private-mutation */
    private int $defaultMaxLength = 255;

    public function setConfiguration(ConfigurationInterface $configuration): void
    {}

    /**
     * {@inheritDoc}
     *
     * @psalm-immutable
     */
    public function normalize(string $text, array $context = []): string
    {}
}

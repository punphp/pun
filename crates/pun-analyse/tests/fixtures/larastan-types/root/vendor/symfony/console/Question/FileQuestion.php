<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Question;

use Symfony\Component\Console\Exception\InvalidArgumentException;

/**
 * Represents a question that accepts file input (paste or path).
 *
 * @author Robin Chalas <robin.chalas@gmail.com>
 */
class FileQuestion extends Question
{
    public function __construct(
        string $question,
        private bool $allowPaste = true,
        private bool $allowPath = true,
    ) {}

    public function isPasteAllowed(): bool
    {}

    public function isPathAllowed(): bool
    {}
}

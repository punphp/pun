<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

use Symfony\Component\Console\Formatter\OutputFormatter;

/**
 * The Formatter class provides helpers to format messages.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class FormatterHelper extends Helper
{
    /**
     * Formats a message within a section.
     */
    public function formatSection(string $section, string $message, string $style = 'info'): string
    {}

    /**
     * Formats a message as a block of text.
     */
    public function formatBlock(string|array $messages, string $style, bool $large = false): string
    {}

    /**
     * Truncates a message to the given length.
     */
    public function truncate(string $message, int $length, string $suffix = '...'): string
    {}

    public function getName(): string
    {}
}

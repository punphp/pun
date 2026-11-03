<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Finder\Iterator;

/**
 * MultiplePcreFilterIterator filters files using patterns (regexps, globs or strings).
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @template-covariant TKey
 * @template-covariant TValue
 *
 * @extends \FilterIterator<TKey, TValue>
 */
abstract class MultiplePcreFilterIterator extends \FilterIterator
{
    protected array $matchRegexps = [];
    protected array $noMatchRegexps = [];

    /**
     * @param \Iterator<TKey, TValue> $iterator        The Iterator to filter
     * @param string[]                $matchPatterns   An array of patterns that need to match
     * @param string[]                $noMatchPatterns An array of patterns that need to not match
     */
    public function __construct(\Iterator $iterator, array $matchPatterns, array $noMatchPatterns)
    {}

    /**
     * Checks whether the string is accepted by the regex filters.
     *
     * If there is no regexps defined in the class, this method will accept the string.
     * Such case can be handled by child classes before calling the method if they want to
     * apply a different behavior.
     */
    protected function isAccepted(string $string): bool
    {}

    /**
     * Checks whether the string is a regex.
     */
    protected function isRegex(string $str): bool
    {}

    /**
     * Converts string into regexp.
     */
    abstract protected function toRegex(string $str): string;
}

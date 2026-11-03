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

use Symfony\Component\Finder\SplFileInfo;

/**
 * FilecontentFilterIterator filters files by their contents using patterns (regexps or strings).
 *
 * @author Fabien Potencier  <fabien@symfony.com>
 * @author Włodzimierz Gajda <gajdaw@gajdaw.pl>
 *
 * @extends MultiplePcreFilterIterator<string, SplFileInfo>
 */
class FilecontentFilterIterator extends MultiplePcreFilterIterator
{
    /**
     * Filters the iterator values.
     */
    public function accept(): bool
    {}

    /**
     * Converts string to regexp if necessary.
     *
     * @param string $str Pattern: string or regexp
     */
    protected function toRegex(string $str): string
    {}
}

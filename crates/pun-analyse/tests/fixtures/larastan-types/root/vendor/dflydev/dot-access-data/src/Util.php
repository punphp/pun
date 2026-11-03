<?php

declare(strict_types=1);

/*
 * This file is a part of dflydev/dot-access-data.
 *
 * (c) Dragonfly Development Inc.
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Dflydev\DotAccessData;

class Util
{
    /**
     * Test if array is an associative array
     *
     * Note that this function will return true if an array is empty. Meaning
     * empty arrays will be treated as if they are associative arrays.
     *
     * @param array<mixed> $arr
     *
     * @return bool
     *
     * @psalm-pure
     */
    public static function isAssoc(array $arr): bool
    {}

    /**
     * Merge contents from one associtative array to another
     *
     * @param mixed $to
     * @param mixed $from
     * @param DataInterface::PRESERVE|DataInterface::REPLACE|DataInterface::MERGE $mode
     *
     * @return mixed
     *
     * @psalm-pure
     */
    public static function mergeAssocArray($to, $from, int $mode = DataInterface::REPLACE)
    {}

    /**
     * @param mixed $value
     *
     * @return bool
     *
     * @psalm-pure
     */
    private static function isList($value): bool
    {}
}

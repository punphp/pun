<?php declare(strict_types=1);
/*
 * This file is part of sebastian/object-enumerator.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\ObjectEnumerator;

use function is_array;
use function is_object;
use SebastianBergmann\RecursionContext\Context;

final class Enumerator
{
    /**
     * @param array<mixed>|object $variable
     *
     * @return list<object>
     */
    public function enumerate(array|object $variable, Context $processed = new Context): array
    {}

    /**
     * @param array<mixed>|object $variable
     * @param list<object>        $objects
     */
    private function process(array|object &$variable, Context $processed, array &$objects): void
    {}
}

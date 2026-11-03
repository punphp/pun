<?php declare(strict_types=1);
/*
 * This file is part of sebastian/recursion-context.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\RecursionContext;

use const PHP_INT_MAX;
use const PHP_INT_MIN;
use function array_key_exists;
use function array_key_last;
use function array_pop;
use function count;
use function is_array;
use function random_int;
use function spl_object_id;
use SplObjectStorage;

final class Context
{
    /**
     * @var list<array<mixed>>
     */
    private array $arrays = [];

    /**
     * @var SplObjectStorage<object, null>
     */
    private SplObjectStorage $objects;

    public function __construct()
    {}

    /**
     * @codeCoverageIgnore
     */
    public function __destruct()
    {}

    /**
     * @template T of object|array
     *
     * @param T $value
     *
     * @param-out T $value
     */
    public function add(array|object &$value): int
    {}

    /**
     * @template T of object|array
     *
     * @param T $value
     *
     * @param-out T $value
     */
    public function contains(array|object &$value): false|int
    {}

    /**
     * @param array<mixed> $array
     */
    private function addArray(array &$array): int
    {}

    private function addObject(object $object): int
    {}

    /**
     * @param array<mixed> $array
     */
    private function containsArray(array $array): false|int
    {}

    private function containsObject(object $value): false|int
    {}
}

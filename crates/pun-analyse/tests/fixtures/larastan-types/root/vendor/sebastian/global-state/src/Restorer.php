<?php declare(strict_types=1);
/*
 * This file is part of sebastian/global-state.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\GlobalState;

use function array_diff;
use function array_key_exists;
use function array_keys;
use function array_merge;
use function assert;
use function class_exists;
use function in_array;
use function is_array;
use function property_exists;
use ReflectionClass;
use ReflectionProperty;

final class Restorer
{
    public function restoreGlobalVariables(Snapshot $snapshot): void
    {}

    public function restoreStaticProperties(Snapshot $snapshot): void
    {}

    private function restoreSuperGlobalArray(Snapshot $snapshot, string $superGlobalArray): void
    {}
}

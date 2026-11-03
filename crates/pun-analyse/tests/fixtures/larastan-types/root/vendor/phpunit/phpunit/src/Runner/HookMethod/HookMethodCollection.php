<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner;

use function array_map;
use function strtolower;
use function usort;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class HookMethodCollection
{
    private readonly HookMethod $defaultHookMethod;
    private readonly bool $shouldPrepend;

    /**
     * @var non-empty-list<HookMethod>
     */
    private array $hookMethods;

    public static function defaultBeforeClass(): self
    {}

    public static function defaultBefore(): self
    {}

    public static function defaultPreCondition(): self
    {}

    public static function defaultPostCondition(): self
    {}

    public static function defaultAfter(): self
    {}

    public static function defaultAfterClass(): self
    {}

    private function __construct(HookMethod $default, bool $shouldPrepend)
    {}

    /**
     * @param non-empty-string $methodName
     */
    public function isDefaultHookMethod(string $methodName): bool
    {}

    public function add(HookMethod $hookMethod): self
    {}

    /**
     * @return list<non-empty-string>
     */
    public function methodNamesSortedByPriority(): array
    {}
}
